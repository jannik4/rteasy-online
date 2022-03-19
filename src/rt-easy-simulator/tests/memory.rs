mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

const SOURCE: &'static str = r#"
    declare register AR(5:0), DR(7:0)
    declare memory MEM(AR,DR)

    AR <- 0, DR <- 42;
    write MEM;
    AR <- 2, DR <- 7;
    write MEM; # 1

    AR <- 0, DR <- 21;
    write MEM; # 2
"#;

#[test]
fn memory() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // Check page count
    assert_eq!(
        simulator.memory_page_count(&Ident("MEM".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );

    // Check next/prev page
    assert_eq!(
        simulator
            .memory_page_prev(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap())
            .unwrap(),
        None
    );
    assert_eq!(
        simulator
            .memory_page_prev(&Ident("MEM".to_string()), Value::parse_dec("2").unwrap())
            .unwrap(),
        Some(Value::parse_dec("1").unwrap())
    );
    assert_eq!(
        simulator
            .memory_page_next(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap())
            .unwrap(),
        Some(Value::parse_dec("2").unwrap())
    );
    assert_eq!(
        simulator
            .memory_page_next(&Ident("MEM".to_string()), Value::parse_dec("2").unwrap())
            .unwrap(),
        None
    );

    // 1
    for _ in 0..4 {
        simulator.step(false).unwrap();
    }
    let page =
        simulator.memory_page(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap()).unwrap();
    assert_eq!(page[0], (Value::parse_dec("0").unwrap(), Value::parse_dec("42").unwrap()));
    assert_eq!(page[2], (Value::parse_dec("2").unwrap(), Value::parse_dec("7").unwrap()));
    assert_eq!(page[3], (Value::parse_dec("3").unwrap(), Value::parse_dec("0").unwrap()));

    // Save memory
    let mut save_bytes = Vec::new();
    simulator.save_memory(&Ident("MEM".to_string()), &mut save_bytes).unwrap();
    let save = String::from_utf8(save_bytes).unwrap();

    // 2
    for _ in 0..2 {
        simulator.step(false).unwrap();
    }
    let page =
        simulator.memory_page(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap()).unwrap();
    assert_eq!(page[0], (Value::parse_dec("0").unwrap(), Value::parse_dec("21").unwrap()));

    // Load from save
    simulator.load_memory_from_save(&Ident("MEM".to_string()), save.as_bytes()).unwrap();
    let page =
        simulator.memory_page(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap()).unwrap();
    assert_eq!(page[0], (Value::parse_dec("0").unwrap(), Value::parse_dec("42").unwrap()));
}
