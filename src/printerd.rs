//! The [`Debug`] printer.
use parking_lot::Mutex;
use std::fmt;
use std::fmt::Debug;

pub struct PrintingIteratorDebug<T: Debug, I: Iterator<Item = T>> {
    // todo: store Mutex<dyn Iterator<Item = T>> instead to save code size?
    // it's not like the perf of the iterator matters when printing is slow
    iter: Mutex<I>,
    pretty: bool,
    indices: bool,
}

pub trait PrinterateDebug<T: Debug>: Iterator<Item = T> + Sized {
    fn printerd(self) -> PrintingIteratorDebug<T, Self> {
        Self::printerd_with_options(self, true, true)
    }

    fn printerd_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDebug<T, Self>;
}

impl<T: Debug, I: Iterator<Item = T>> PrinterateDebug<T> for I {
    fn printerd(self) -> PrintingIteratorDebug<T, Self> {
        PrintingIteratorDebug {
            iter: Mutex::new(self),
            pretty: true,
            indices: true,
        }
    }

    fn printerd_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDebug<T, Self> {
        PrintingIteratorDebug {
            iter: Mutex::new(self),
            pretty,
            indices,
        }
    }
}

impl<T: Debug, I: Iterator<Item = T>> Debug for PrintingIteratorDebug<T, I> {
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
