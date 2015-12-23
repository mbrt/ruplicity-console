#[macro_use]

macro_rules! console_print {
    ($out:expr, $pfn:expr, $arg:expr) => (
        writeln!($out, "{}", $pfn($arg));
    );
    ($out:expr, $pfn:expr, $fmt:expr, $($arg:tt)+) => (
        writeln!($out, $fmt, $($pfn($arg))+)
    )
}


macro_rules! console_err {
    ($($arg:tt)+) => (
        console_print!(&mut std::io::stderr(), $crate::console::err, $($arg)+)
    )
}

macro_rules! console_warn {
    ($($arg:tt)+) => (
        console_print!(&mut std::io::stdout(), $crate::console::warn, $($arg)+)
    )
}

macro_rules! console_good {
    ($($arg:tt)+) => (
        console_print!(&mut std::io::stdout(), $crate::console::good, $($arg)+)
    )
}


pub use self::imp::{err, good, warn};


#[cfg(all(feature = "color", not(target_os = "windows")))]
mod imp {
    use std::fmt;

    use ansi_term::Colour::{Green, Red, Yellow};
    use ansi_term::ANSIString;

    pub fn err<T: AsRef<str>>(t: T) -> Format<T> {
        Format::Error(t)
    }

    pub fn warn<T: AsRef<str>>(t: T) -> Format<T> {
        Format::Warning(t)
    }

    pub fn good<T: AsRef<str>>(t: T) -> Format<T> {
        Format::Good(t)
    }


    /// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
    /// and Good=Green
    #[derive(Debug)]
    pub enum Format<T> {
        /// Defines the style used for errors, defaults to Red
        Error(T),
        /// Defines the style used for warnings, defaults to Yellow
        Warning(T),
        /// Defines the style used for good values, defaults to Green
        Good(T),
    }

    impl<T: AsRef<str>> Format<T> {
        fn format(&self) -> ANSIString {
            match *self {
                Format::Error(ref e) => Red.bold().paint(e.as_ref()),
                Format::Warning(ref e) => Yellow.paint(e.as_ref()),
                Format::Good(ref e) => Green.paint(e.as_ref()),
            }
        }
    }

    impl<T: AsRef<str>> fmt::Display for Format<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.format())
        }
    }
}


#[cfg(any(not(feature = "color"), target_os = "windows"))]
mod imp {
    pub fn err<T: AsRef<str>>(t: T) -> T {
        t
    }

    pub fn warn<T: AsRef<str>>(t: T) -> T {
        t
    }

    pub fn good<T: AsRef<str>>(t: T) -> T {
        t
    }
}
