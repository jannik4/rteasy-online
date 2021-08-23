use serde_json::Value;
use toktok_generator_tests::json::parse;

#[test]
fn test_json() {
    let source = r###"
    {
        "hello": "world",
        "x": [true, 12, false, -1, "true"],
        "y": { "zzz": "", "a": [] }
    }
    "###;
    let res = parse(source).unwrap();

    let expected = serde_json::from_str::<Value>(source).unwrap();
    assert_eq!(res, expected);
}
