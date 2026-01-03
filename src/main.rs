use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use tiny_skia::Pixmap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "cbmp-rs")]
#[command(version = VERSION)]
#[command(about = "A CLI for converting cursor SVG files to PNG files.", long_about = None)]
struct Cli {
    /// Path to JSON configuration file
    #[arg(value_name = "CONFIG")]
    config: PathBuf,

    /// Show less logs
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(flatten)]
    tasks: std::collections::HashMap<String, Task>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    dir: PathBuf,
    out: PathBuf,
    #[serde(default)]
    colors: Option<Vec<Color>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Color {
    // Rename match -> match_color because match is a keyword
    #[serde(rename = "match")]
    match_color: String,
    replace: String,
}

struct Converter {
    quiet: bool,
}

impl Converter {
    fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    fn svg_to_png(
        &self,
        svg_path: &Path,
        output_path: &Path,
        colors: Option<&[Color]>,
    ) -> Result<()> {
        let mut svg_data = fs::read_to_string(svg_path)
            .with_context(|| format!("Failed to read SVG file: {}", svg_path.display()))?;

        if let Some(color_list) = colors {
            for color in color_list {
                svg_data = svg_data.replace(&color.match_color, &color.replace);
            }
        }

        let options = usvg::Options::default();
        let tree = usvg::Tree::from_str(&svg_data, &options)
            .with_context(|| format!("Failed to parse SVG: {}", svg_path.display()))?;

        let size = tree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        let mut pixmap = Pixmap::new(width, height)
            .with_context(|| format!("Failed to create pixmap for: {}", svg_path.display()))?;

        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        pixmap
            .save_png(output_path)
            .with_context(|| format!("Failed to save PNG: {}", output_path.display()))?;

        Ok(())
    }

    fn process_directory(
        &self,
        input_dir: &Path,
        output_dir: &Path,
        colors: Option<&[Color]>,
        task_name: &str,
    ) -> Result<()> {
        // like mkdir -p
        fs::create_dir_all(output_dir).with_context(|| {
            format!(
                "Failed to create output directory: {}",
                output_dir.display()
            )
        })?;

        // Find SVG files
        let pattern = input_dir.join("**/*.svg");
        let pattern_str = pattern.to_str().unwrap();

        let svg_files: Vec<PathBuf> = glob::glob(pattern_str)
            .with_context(|| "Failed to read glob pattern")?
            .filter_map(|e| e.ok())
            .collect();

        if svg_files.is_empty() {
            warn(&format!("No SVG files found in: {}", input_dir.display()));
            return Ok(());
        }

        if !self.quiet {
            info("SVG files found");
        }

        for svg_path in &svg_files {
            let rel_path = svg_path.strip_prefix(input_dir).unwrap_or(svg_path);
            let output_path = output_dir
                .join(rel_path.file_name().unwrap())
                .with_extension("png");

            // Create parent dir if needed
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            if !self.quiet {
                print!("Processing item: {} ...", rel_path.display());
                let _ = stdout().flush();
            }

            match self.svg_to_png(svg_path, &output_path, colors) {
                Ok(_) => {
                    if !self.quiet {
                        println!(" {}DONE{}", colors::SUCCESS, colors::RESET);
                    }
                }
                Err(e) => {
                    error(
                        &format!("Failed to process {}: {}", rel_path.display(), e),
                        false,
                    );
                }
            }
        }

        println!(
            "{}DONE{}: Finished processing {} file(s) ({})",
            colors::SUCCESS,
            colors::RESET,
            svg_files.len(),
            task_name
        );

        Ok(())
    }

    fn process_config(&self, config_path: &Path) -> Result<()> {
        if !self.quiet {
            info(&format!(
                "Loading configuration from {}",
                config_path.display()
            ));
        }

        let config_data =
            fs::read_to_string(config_path).with_context(|| "Failed to read config file")?;

        let config: Config = serde_json::from_str(&config_data)
            .with_context(|| "Failed to parse JSON configuration")?;

        for (task_name, task) in &config.tasks {
            println!();
            info(&format!("Processing task: {}", &task_name));
            self.process_directory(&task.dir, &task.out, task.colors.as_deref(), task_name)?;
        }

        Ok(())
    }
}

mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const WARN: &str = "\x1b[33m";
    pub const ERROR: &str = "\x1b[31m";
    pub const INFO: &str = "\x1b[34m";
    pub const SUCCESS: &str = "\x1b[32m";
}

fn warn(msg: &str) {
    println!("{}WARN:{} {}", colors::WARN, colors::RESET, msg);
}

fn error(msg: &str, fatal: bool) {
    println!("{}ERROR:{} {}", colors::ERROR, colors::RESET, msg);
    if fatal {
        std::process::exit(1)
    }
}

fn info(msg: &str) {
    println!("{}INFO:{} {}", colors::INFO, colors::RESET, msg);
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let converter = Converter::new(cli.quiet);

    converter.process_config(&cli.config)
}
