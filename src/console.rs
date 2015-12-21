#[cfg(feature = "color")]
pub use self::imp::Console;
pub use self::imp::error;


#[cfg(not(feature = "color"))]
mod imp {
    use std::io;

    pub fn error() -> io::Stderr {
        io::stderr()
    }
}


#[cfg(feature = "color")]
mod imp {
    use std::io;
    use std::ops::Drop;
    use term;

    pub struct Console(Box<term::StderrTerminal>);

    pub fn error() -> Console {
        let mut t = term::stderr().unwrap();
        let _ = t.fg(term::color::RED);
        Console(t)
    }

    impl Drop for Console {
        fn drop(&mut self) {
            let _ = self.0.reset();
        }
    }

    impl io::Write for Console {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.0.flush()
        }
    }
}
