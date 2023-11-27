//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}
// /// Prints to the standard output, with a info
#[macro_export]
macro_rules! pinfo {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        if 1>=option_env!("AX_DEBUG").unwrap_or("0").parse::<u8>().unwrap(){
            $crate::io::__pinfo_impl(format_args!("{}\n", format_args!($($arg)*)));
        }
    }
}

/// Prints to the standard output, with a dev
#[macro_export]
macro_rules! pdev {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        if 2>=option_env!("AX_DEBUG").unwrap_or("0").parse::<u8>().unwrap(){
            $crate::io::__pdev_impl(format_args!("{}\n", format_args!($($arg)*)));
        }
    }
}

/// Prints to the standard output, with a debug
#[macro_export]
macro_rules! pdebug {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        if 3>=option_env!("AX_DEBUG").unwrap_or("0").parse::<u8>().unwrap(){
            $crate::io::__pdebug_impl(format_args!("{}\n", format_args!($($arg)*)));
        }
    }
}

