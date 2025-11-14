# printerator

Print iterators without having to collect them.

# Examples

```rust
# use printerator::PrinterateDebug;
let ints: [u32; 3] = [0xcafebabe, 0xabad1dea, 0xdeadc0de];
println!(
    "{:.1?}",
    ints.iter().map(|&int| (int as f32).sqrt()).printerd(),
);
```

```rust
# use printerator::PrinterateDisplay;
let data = "Hello, w🌍rld!";
let mut nonascii_iter = data.bytes().enumerate().filter(|&(_, b)| b >= 128);
println!(
    "Invalid ascii byte indices: {}",
    nonascii_iter.clone().map(|(i, _)| i).printer_with_options(false, false),
);
```

Available in 2 flavors: [`A debug printer`], and a [`display printer`], both of
which are createble over any iterator whose item is implementing either
[`Debug`] or [`Display`], using the methods [`printerd`] and [`printer`] respectively.

Formatting options are passed as-is to the [`Iterator::Item`]'s implementation of
[`Debug`] and [`Display`]. That means you should pass them in as if you were
formatting 1 single item.

[`printer{d}_with_options`] arguments:

- `pretty`: true if you want newlines and brackets:
```ignore
[
    item,
    item2,
    item3
]
[
    0: item,
    1: item2
]
```
otherwise, false:
```ignore
item, item2, item3
0: item, 1: item2
```

- `indices`: true if you want the indices too:
```ignore
[
    0: item,
    1: item2
]
0: item, 1: item2, 2: item3
```
otherwise, false:
```ignore
[
    item,
    item2
]
item, item2
```

[`A debug printer`]: crate::PrintingIteratorDebug
[`display printer`]: crate::PrintingIteratorDisplay
[`printerd`]: crate::PrinterateDebug::printerd
[`printer`]: crate::PrinterateDisplay::printer
[`printer{d}_with_options`]: crate::PrinterateDebug::printerd_with_options
