#![macro_use]

macro_rules! console_err {
    (target: $out:expr, $($arg:tt)+) => ({
        writeln!(&mut $out, "{}", $crate::console::err(format!($($arg)+))).unwrap();
    });
    ($($arg:tt)+) => ({
        writeln!(&mut ::std::io::stdout(), "{}", $crate::console::err(format!($($arg)+))).unwrap();
    })
}

macro_rules! console_warn {
    (target: $out:expr, $($arg:tt)+) => ({
        writeln!(&mut $out, "{}", $crate::console::warn(format!($($arg)+))).unwrap();
    });
    ($($arg:tt)+) => ({
        writeln!(&mut ::std::io::stdout(), "{}", $crate::console::warn(format!($($arg)+))).unwrap();
    })
}

macro_rules! console_good {
    (target: $out:expr, $($arg:tt)+) => ({
        writeln!(&mut $out, "{}", $crate::console::good(format!($($arg)+))).unwrap();
    });
    ($($arg:tt)+) => ({
        writeln!(&mut ::std::io::stdout(), "{}", $crate::console::good(format!($($arg)+))).unwrap();
    })
}

macro_rules! console_info {
    (target: $out:expr, $($arg:tt)+) => ({
        writeln!(&mut $out, $($arg)+).unwrap();
    });
    ($($arg:tt)+) => ({
        writeln!(&mut ::std::io::stdout(), $($arg)+).unwrap();
    })
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
