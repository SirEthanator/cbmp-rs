#[macro_export]
macro_rules! __log_base {
    // $($arg:tt)* captures list of tokens (tt = token tree)
    ($level:expr, $color:expr, $newline:expr, $($arg:tt)*) => {
        use $crate::colors;
        let prefix = format!("{}[{}]{}", $color, $level, colors::RESET);

        match ($level, $newline) {
            // format_args!($($arg)*) "forwards" the token tree as arguments
            ("ERROR", true) => eprintln!("{} {}", prefix, format_args!($($arg)*)),
            ("ERROR", false) => eprint!("{} {}", prefix, format_args!($($arg)*)),
            (_, true) => println!("{} {}", prefix, format_args!($($arg)*)),
            (_, false) => print!("{} {}", prefix, format_args!($($arg)*)),
        }
    };
}

#[macro_export]
macro_rules! log_done {
    ($($arg:tt)*) => {
        __log_base!("DONE", $crate::colors::SUCCESS, false, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_doneln {
    ($($arg:tt)*) => {
        __log_base!("DONE", $crate::colors::SUCCESS, true, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        __log_base!("INFO", $crate::colors::INFO, false, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_infoln {
    ($($arg:tt)*) => {
        __log_base!("INFO", $crate::colors::INFO, true, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_task {
    ($($arg:tt)*) => {
        __log_base!("TASK", $crate::colors::INFO, false, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_taskln {
    ($($arg:tt)*) => {
        __log_base!("TASK", $crate::colors::INFO, true, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        __log_base!("WARN", $crate::colors::WARN, false, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_warnln {
    ($($arg:tt)*) => {
        __log_base!("WARN", $crate::colors::WARN, true, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        __log_base!("ERROR", $crate::colors::ERROR, false, $($arg)*)
    };
}

#[macro_export]
macro_rules! log_errorln {
    ($($arg:tt)*) => {
        __log_base!("ERROR", $crate::colors::ERROR, true, $($arg)*)
    };
}
