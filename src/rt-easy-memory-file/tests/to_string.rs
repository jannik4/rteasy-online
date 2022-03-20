use rt_easy_memory_file::{MemoryFile, Value};
use std::collections::HashMap;

#[test]
fn header() {
    let mem = MemoryFile::new(12, 4);

    assert_eq!(mem.to_string(), "H 12 4\n");
}

#[test]
fn data_consecutive() {
    let mem = MemoryFile::new_unchecked(
        8,
        32,
        HashMap::from([
            (Value::parse_hex("0").unwrap(), Value::parse_hex("00FA").unwrap()),
            (Value::parse_hex("1").unwrap(), Value::parse_hex("FF").unwrap()),
            (Value::parse_hex("2").unwrap(), Value::parse_hex("0").unwrap()),
            (Value::parse_hex("3").unwrap(), Value::parse_hex("1").unwrap()),
            (Value::parse_hex("4").unwrap(), Value::parse_hex("2").unwrap()),
            (Value::parse_hex("5").unwrap(), Value::parse_hex("F01").unwrap()),
        ]),
    );

    assert_eq!(
        mem.to_string(),
        r#"H 8 32

FA
FF
0
1
2
F01
"#
    );
}

#[test]
fn data_scattered() {
    let mem = MemoryFile::new_unchecked(
        8,
        32,
        HashMap::from([
            (Value::parse_hex("FF").unwrap(), Value::parse_hex("13").unwrap()),
            (Value::parse_hex("1").unwrap(), Value::parse_hex("A").unwrap()),
            (Value::parse_hex("2").unwrap(), Value::parse_hex("B").unwrap()),
            (Value::parse_hex("3").unwrap(), Value::parse_hex("C").unwrap()),
            (Value::parse_hex("5").unwrap(), Value::parse_hex("0").unwrap()),
            (Value::parse_hex("6").unwrap(), Value::parse_hex("AA").unwrap()),
        ]),
    );

    assert_eq!(
        mem.to_string(),
        r#"H 8 32


1:
A
B
C

5:
0
AA

FF:
13
"#
    );
}
