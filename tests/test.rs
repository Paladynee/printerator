use printerator::{PrinterateDebug, PrinterateDisplay};
const DATA: [u32; 6] = [0xdeadc0de, 0xcafebabe, 0xabad1dea, 0xaaaaaaaa, 0x00000000, 0x37539436];
const ONEDATA: [u32; 1] = [0x12345678];
const STRING: [&str; 1] = ["Hello, w🌍orld!"];

#[test]
fn printerd_string_quoted() {
    let formatted = format!("{:?}", STRING.iter().printerd_with_options(false, false));
    assert_eq!(formatted, "\"Hello, w🌍orld!\"");
}

#[test]
fn printer_string_unquoted() {
    let formatted = format!("{}", STRING.iter().printer_with_options(false, false));
    assert_eq!(formatted, "Hello, w🌍orld!");
}

#[test]
fn printerd_unpretty_noindices_enumerated() {
    let formatted = format!("{:?}", DATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(false, false));
    assert_eq!(formatted, "0, 1, 2, 3, 4, 5");
    let formatted = format!("{:?}", ONEDATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(false, false));
    assert_eq!(formatted, "0");
}

#[test]
fn printerd_unpretty_noindices_data() {
    let formatted = format!(
        "{:.1?}",
        DATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(false, false)
    );
    assert_eq!(formatted, "61122.2, 58358.3, 53668.0, 53509.9, 0.0, 30466.8");
    let formatted = format!(
        "{:.1?}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(false, false)
    );
    assert_eq!(formatted, "17476.3");
}

#[test]
fn printerd_pretty_noindices_enumerated() {
    let formatted = format!("{:?}", DATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(true, false));
    assert_eq!(formatted, "[\n    0,\n    1,\n    2,\n    3,\n    4,\n    5\n]");
    let formatted = format!("{:?}", ONEDATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(true, false));
    assert_eq!(formatted, "[\n    0\n]");
}

#[test]
fn printerd_pretty_noindices_data() {
    let formatted = format!(
        "{:.1?}",
        DATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(true, false)
    );
    assert_eq!(
        formatted,
        "[\n    61122.2,\n    58358.3,\n    53668.0,\n    53509.9,\n    0.0,\n    30466.8\n]"
    );
    let formatted = format!(
        "{:.1?}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(true, false)
    );
    assert_eq!(formatted, "[\n    17476.3\n]");
}

#[test]
fn printerd_pretty_indices_enumerated() {
    let formatted = format!("{:?}", DATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(true, true));
    assert_eq!(formatted, "[\n    0: 0,\n    1: 1,\n    2: 2,\n    3: 3,\n    4: 4,\n    5: 5\n]");
    let formatted = format!("{:?}", ONEDATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(true, true));
    assert_eq!(formatted, "[\n    0: 0\n]");
}

#[test]
fn printerd_pretty_indices_data() {
    let formatted = format!(
        "{:.1?}",
        DATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(true, true)
    );
    assert_eq!(
        formatted,
        "[\n    0: 61122.2,\n    1: 58358.3,\n    2: 53668.0,\n    3: 53509.9,\n    4: 0.0,\n    5: 30466.8\n]"
    );
    let formatted = format!(
        "{:.1?}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(true, true)
    );
    assert_eq!(formatted, "[\n    0: 17476.3\n]");
}

#[test]
fn printerd_unpretty_indices_enumerated() {
    let formatted = format!("{:?}", DATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(false, true));
    assert_eq!(formatted, "0: 0, 1: 1, 2: 2, 3: 3, 4: 4, 5: 5");
    let formatted = format!("{:?}", ONEDATA.into_iter().enumerate().map(|a| a.0).printerd_with_options(false, true));
    assert_eq!(formatted, "0: 0");
}

#[test]
fn printerd_unpretty_indices_data() {
    let formatted = format!(
        "{:.1?}",
        DATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(false, true)
    );
    assert_eq!(formatted, "0: 61122.2, 1: 58358.3, 2: 53668.0, 3: 53509.9, 4: 0.0, 5: 30466.8");
    let formatted = format!(
        "{:.1?}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printerd_with_options(false, true)
    );
    assert_eq!(formatted, "0: 17476.3");
}

#[test]
fn printer_unpretty_noindices_enumerated() {
    let formatted = format!("{}", DATA.into_iter().enumerate().map(|a| a.0).printer_with_options(false, false));
    assert_eq!(formatted, "0, 1, 2, 3, 4, 5");
    let formatted = format!("{}", ONEDATA.into_iter().enumerate().map(|a| a.0).printer_with_options(false, false));
    assert_eq!(formatted, "0");
}

#[test]
fn printer_unpretty_noindices_data() {
    let formatted = format!(
        "{:.1}",
        DATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(false, false)
    );
    assert_eq!(formatted, "61122.2, 58358.3, 53668.0, 53509.9, 0.0, 30466.8");
    let formatted = format!(
        "{:.1}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(false, false)
    );
    assert_eq!(formatted, "17476.3");
}

#[test]
fn printer_pretty_noindices_enumerated() {
    let formatted = format!("{}", DATA.into_iter().enumerate().map(|a| a.0).printer_with_options(true, false));
    assert_eq!(formatted, "[\n    0,\n    1,\n    2,\n    3,\n    4,\n    5\n]");
    let formatted = format!("{}", ONEDATA.into_iter().enumerate().map(|a| a.0).printer_with_options(true, false));
    assert_eq!(formatted, "[\n    0\n]");
}

#[test]
fn printer_pretty_noindices_data() {
    let formatted = format!("{:.1}", DATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(true, false));
    assert_eq!(
        formatted,
        "[\n    61122.2,\n    58358.3,\n    53668.0,\n    53509.9,\n    0.0,\n    30466.8\n]"
    );
    let formatted = format!(
        "{:.1}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(true, false)
    );
    assert_eq!(formatted, "[\n    17476.3\n]");
}

#[test]
fn printer_pretty_indices_enumerated() {
    let formatted = format!("{}", DATA.into_iter().enumerate().map(|a| a.0).printer_with_options(true, true));
    assert_eq!(formatted, "[\n    0: 0,\n    1: 1,\n    2: 2,\n    3: 3,\n    4: 4,\n    5: 5\n]");
    let formatted = format!("{}", ONEDATA.into_iter().enumerate().map(|a| a.0).printer_with_options(true, true));
    assert_eq!(formatted, "[\n    0: 0\n]");
}

#[test]
fn printer_pretty_indices_data() {
    let formatted = format!("{:.1}", DATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(true, true));
    assert_eq!(
        formatted,
        "[\n    0: 61122.2,\n    1: 58358.3,\n    2: 53668.0,\n    3: 53509.9,\n    4: 0.0,\n    5: 30466.8\n]"
    );
    let formatted = format!(
        "{:.1}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(true, true)
    );
    assert_eq!(formatted, "[\n    0: 17476.3\n]");
}

#[test]
fn printer_unpretty_indices_enumerated() {
    let formatted = format!("{}", DATA.into_iter().enumerate().map(|a| a.0).printer_with_options(false, true));
    assert_eq!(formatted, "0: 0, 1: 1, 2: 2, 3: 3, 4: 4, 5: 5");
    let formatted = format!("{}", ONEDATA.into_iter().enumerate().map(|a| a.0).printer_with_options(false, true));
    assert_eq!(formatted, "0: 0");
}

#[test]
fn printer_unpretty_indices_data() {
    let formatted = format!("{:.1}", DATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(false, true));
    assert_eq!(formatted, "0: 61122.2, 1: 58358.3, 2: 53668.0, 3: 53509.9, 4: 0.0, 5: 30466.8");
    let formatted = format!(
        "{:.1}",
        ONEDATA.into_iter().map(|int| (int as f32).sqrt()).printer_with_options(false, true)
    );
    assert_eq!(formatted, "0: 17476.3");
}
