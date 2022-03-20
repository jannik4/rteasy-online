use rt_easy_memory_file::{MemoryFile, Value};
use std::collections::HashMap;

#[test]
fn header() {
    let sources = ["B 12 4", "b 12 4", "H 12 4", "h 12 4"];

    for source in sources {
        let mem = MemoryFile::parse(&source).unwrap();
        assert_eq!(mem, MemoryFile { ar_size: 12, dr_size: 4, data: HashMap::new() });
    }
}

#[test]
fn data() {
    let sources = [
        r###"B 4 8
010111
0
11"###,
        r###"H 4 8
17
0
3"###,
    ];

    for source in sources {
        let mem = MemoryFile::parse(&source).unwrap();
        assert_eq!(
            mem,
            MemoryFile {
                ar_size: 4,
                dr_size: 8,
                data: HashMap::from([
                    (Value::parse_bin("0").unwrap(), Value::parse_bin("010111").unwrap()),
                    (Value::parse_bin("1").unwrap(), Value::parse_bin("0").unwrap()),
                    (Value::parse_bin("10").unwrap(), Value::parse_bin("11").unwrap()),
                ])
            }
        );
    }
}

#[test]
fn address() {
    let sources = [
        r###"B 4 8
110:
010111
0
0:
1111:
11"###,
        r###"H 4 8
6:
17
0
0:
F:
3"###,
    ];

    for source in sources {
        let mem = MemoryFile::parse(&source).unwrap();
        assert_eq!(
            mem,
            MemoryFile {
                ar_size: 4,
                dr_size: 8,
                data: HashMap::from([
                    (Value::parse_bin("110").unwrap(), Value::parse_bin("010111").unwrap()),
                    (Value::parse_bin("111").unwrap(), Value::parse_bin("0").unwrap()),
                    (Value::parse_bin("1111").unwrap(), Value::parse_bin("11").unwrap()),
                ])
            }
        );
    }
}

#[test]
fn comment() {
    let sources = [
        r###"B 4 8 # a comment
10#asd
1000:##hello world
0                           # 1234
10  ## 
11
"###,
        r###"B 4 8# a co##mment #
10
1000:
0
10 ##comment##
11


# comment"###,
    ];

    for source in sources {
        let mem = MemoryFile::parse(&source).unwrap();
        assert_eq!(
            mem,
            MemoryFile {
                ar_size: 4,
                dr_size: 8,
                data: HashMap::from([
                    (Value::parse_bin("0").unwrap(), Value::parse_bin("10").unwrap()),
                    (Value::parse_bin("1000").unwrap(), Value::parse_bin("0").unwrap()),
                    (Value::parse_bin("1001").unwrap(), Value::parse_bin("10").unwrap()),
                    (Value::parse_bin("1010").unwrap(), Value::parse_bin("11").unwrap()),
                ])
            }
        );
    }
}
