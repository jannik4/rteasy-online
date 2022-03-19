mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

const SOURCE: &'static str = r#"
declare register A(7:0), IDX(5:0)
declare register array ARR(7:0)[64]

ARR[0] <- 12 + ARR[1]; # 1

IDX <- 1;
ARR[IDX] <- ARR[IDX - 1] + 3; # 2

ARR["111111"].A(7) <- 0b101; # 3
"#;

#[test]
fn register_array() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    assert_eq!(simulator.register_array_page_count(&Ident("ARR".to_string())).unwrap(), 2);

    // 1
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_array_page(&Ident("ARR".to_string()), 1).unwrap()[0].1,
        Value::parse_dec("12").unwrap()
    );

    // 2
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_array_page(&Ident("ARR".to_string()), 1).unwrap()[1].1,
        Value::parse_dec("15").unwrap()
    );

    // 3
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_array_page(&Ident("ARR".to_string()), 2).unwrap()[31].1,
        Value::parse_bin("10").unwrap()
    );
}
