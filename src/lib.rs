//! # printerator
//!
//! Print iterators without having to collect them.
//! ```rust
//! # use printerator::PrinterateDebug;
//! # use std::fmt::Write;
//! let ints: [u32; _] = [0xcafebabe, 0xabad1dea, 0xdeadc0de];
//! assert_eq!(
//!     format!(
//!         "{:.1}",
//!         ints.iter().map(|&int| (int as f32).sqrt()).printerd_with_options(false, false),
//!     ),
//!     ""
//! );
//! ```
//!
//! Available in 2 flavors: debug_printer, display_printer.
//!
//! Formatting options are passed as-is to the Item type.
//! That means you should pass them in as if you were formatting 1 single item.
//!
//! Options
//! - pretty: true if you want newlines and brackets:
//! ```ignore
//! [
//!     item,
//!     item2,
//!     item3,
//! ]
//! ```
//! otherwise, false:
//! ```ignore
//! item, item2, item3
//! ```
//!
//! - indices: true if you want the indices too:
//! ```ignore
//! {
//!     0: item,
//!     1: item2,
//! }
//! // or
//! 0: item, 1: item2, 2: item3
//! ```
//! otherwise, false:
//! ```ignore
//! {
//!     item,
//!     item2,
//! }
//! // or
//! item, item2
//! ```
use parking_lot::Mutex;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

// //////////////////// TYPES ////////////////////
pub struct PrintingIteratorDisplay<T: Display, I: Iterator<Item = T>> {
    // todo: store Box<dyn Iterator<Item = T>> instead to save code size?
    // it's not like the perf of the iterator matters when printing
    iter: Mutex<I>,
    pretty: bool,
    indices: bool,
}

pub struct PrintingIteratorDebug<T: Debug, I: Iterator<Item = T>> {
    iter: Mutex<I>,
    pretty: bool,
    indices: bool,
}

// //////////////////// TRAITS ////////////////////
pub trait PrinterateDebug<T: Debug>: Iterator<Item = T> + Sized {
    fn printerd(self) -> PrintingIteratorDebug<T, Self> {
        Self::printerd_with_options(self, false, true)
    }

    fn printerd_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDebug<T, Self>;
}

pub trait PrinterateDisplay<T: Display>: Iterator<Item = T> + Sized {
    fn printer(self) -> PrintingIteratorDisplay<T, Self> {
        Self::printer_with_options(self, false, true)
    }

    fn printer_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDisplay<T, Self>;
}

// //////////////////// CONSTRUCTORS ////////////////////
impl<T: Debug, I: Iterator<Item = T>> PrinterateDebug<T> for I {
    fn printerd_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDebug<T, Self> {
        PrintingIteratorDebug {
            iter: Mutex::new(self),
            pretty,
            indices,
        }
    }
}

impl<T: Display, I: Iterator<Item = T>> PrinterateDisplay<T> for I {
    fn printer_with_options(self, pretty: bool, indices: bool) -> PrintingIteratorDisplay<T, Self> {
        PrintingIteratorDisplay {
            iter: Mutex::new(self),
            pretty,
            indices,
        }
    }
}

// //////////////////// FMT IMPLS ////////////////////
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
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// //////////////////// FMT IMPLS ////////////////////
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
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
