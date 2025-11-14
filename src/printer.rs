//! The [`Display`] printer.
use parking_lot::Mutex;
use std::fmt;
use std::fmt::Display;

pub struct PrintingIteratorDisplay<T: Display, I: Iterator<Item = T>> {
    // todo: store Mutex<dyn Iterator<Item = T>> instead to save code size?
    // it's not like the perf of the iterator matters when printing is slow
    iter: Mutex<I>,
    pretty: bool,
    indices: bool,
}

pub trait PrinterateDisplay<T: Display>: Iterator<Item = T> + Sized {
    fn printer(self) -> PrintingIteratorDisplay<T, Self> {
        Self::printer_with_options(self, true, true)
    }

    fn printer_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDisplay<T, Self>;
}

impl<T: Display, I: Iterator<Item = T>> PrinterateDisplay<T> for I {
    fn printer(self) -> PrintingIteratorDisplay<T, Self> {
        PrintingIteratorDisplay {
            iter: Mutex::new(self),
            pretty: true,
            indices: true,
        }
    }

    fn printer_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDisplay<T, Self> {
        PrintingIteratorDisplay {
            iter: Mutex::new(self),
            pretty,
            indices,
        }
    }
}

impl<T: Display, I: Iterator<Item = T>> Display for PrintingIteratorDisplay<T, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.pretty {
            write!(f, "[")?;
        }
        let mut index = 0;
        let mut item: Option<T> = None;

        let mut locked = self.iter.lock();
        loop {
            match item {
                None => {
                    match locked.next() {
                        None => break,
                        a => item = a,
                    };
                }
                Some(ref elem) => {
                    match locked.next() {
                        None => break,
                        a => {
                            if self.pretty && index == 0 {
                                writeln!(f)?;
                            }
                            if self.pretty {
                                f.write_str("    ")?;
                            }
                            if self.indices {
                                write!(f, "{}: ", index)?;
                            }
                            elem.fmt(f)?;
                            if self.pretty {
                                f.write_str(",\n")?;
                            } else {
                                f.write_str(", ")?;
                            }
                            item = a;
                            index += 1;
                        }
                    };
                }
            };
        }
        if let Some(ref element) = item {
            if self.pretty && index == 0 {
                writeln!(f)?;
            }
            if self.pretty {
                f.write_str("    ")?;
            }
            if self.indices {
                write!(f, "{}: ", index)?;
            }
            element.fmt(f)?;
            if self.pretty {
                f.write_str("\n")?;
            }
        }
        if self.pretty {
            write!(f, "]")?;
        }
        Ok(())
    }
}
