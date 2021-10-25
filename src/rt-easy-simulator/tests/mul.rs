mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

#[test]
fn mul_sm() {
    const SOURCE: &'static str = r#"
        declare input IN(7:0)
        declare output OUT(7:0)
        declare register A(7:0), Q(7:0), M(7:0), COUNT(2:0)

        INIT:
            A <- 0, COUNT <- 0,
            M <- IN;
            Q <- IN;

        ADD:
            if Q(0) = 1 then
                A(7:0) <- A(6:0) + M(6:0)
            else
                A(7:0) <- A(6:0) + 0
            fi;

        RSHIFT_AND_TEST:
            A(7) <- 0, A(6:0).Q <- A.Q(7:1),
            if COUNT <> 6 then
                COUNT <- COUNT + 1, goto ADD
            fi;

        SIGN:
            A(7) <- M(7) xor Q(0), A(6:0).Q <- A.Q(7:1);
        OUTPUT:
            OUT <- Q;
            OUT <- A;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 12 * 7
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("12").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("7").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("84").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );

    simulator.reset();

    // 31 * 47
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("31").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("47").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("177").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("5").unwrap()
    );
}

#[test]
fn mul_2c_fract() {
    const SOURCE: &'static str = r#"
        declare input IN(7:0)
        declare output OUT(7:0)
        declare register F, A(7:0), Q(7:0), M(7:0), COUNT(2:0)        

        INIT:
            A <- 0, COUNT <- 0, F <- 0,
            M <- IN;
            Q <- IN;

        ADD:
            if Q(0) then
                A <- A + M, F <- M(7) and Q(0) or F
            fi;

        RSHIFT:
            A(7) <- F, A(6:0).Q <- A.Q(7:1),
            COUNT <- COUNT + 1;
        TEST:
            if COUNT <> 7 then
                goto ADD
            else
                if Q(0) then
                    A <- A - M, Q(0) <- 0
                fi
            fi;

        OUTPUT:
            OUT <- Q;
            OUT <- A;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 0b11010101, 0b10110011
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_bin("11010101").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_bin("10110011").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_bin("11011110").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("00011001").unwrap()
    );
}

#[test]
fn mul_2c_int() {
    const SOURCE: &'static str = r#"
        declare input IN(7:0)
        declare output OUT(7:0)
        declare register F, A(7:0), Q(7:0), M(7:0), COUNT(2:0)

        INIT:
            A <- 0, COUNT <- 0, F <- 0,
            M <- IN;
            Q <- IN;

        ADD:
            if Q(0) then
                A <- A + M, F <- M(7) and Q(0) or F
        fi;

        RSHIFT:
            A(7) <- F, A(6:0).Q <- A.Q(7:1), COUNT <- COUNT + 1;
        TEST:
            if COUNT <> 7 then
                goto ADD
            else
                if Q(0) then
                    A <- A - M
                fi
            fi;

        ADJUST:
            A(7) <- A(7), A(6:0).Q <- A.Q(7:1);
        OUTPUT:
            OUT <- Q;
            OUT <- A;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 12 * 7
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("12").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("7").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("84").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );

    simulator.reset();

    // 31 * 47
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("31").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("47").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("177").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("5").unwrap()
    );
}

#[test]
fn mul_2c_booth() {
    const SOURCE: &'static str = r#"
        declare input IN(7:0)
        declare output OUT(7:0)
        declare register A(7:0), M(7:0), Q(7:0), Q_1, COUNT(2:0)

        INIT:
            A <- 0, COUNT <- 0,
            M <- IN;
            Q <- IN, Q_1 <- 0;

        SCAN:
            if Q(0) = 0 and Q_1 = 1 then
                A <- A + M, goto TEST
            else
                if Q(0) = 1 and Q_1 = 0 then
                    A <- A - M
                fi
            fi;
        TEST:
            if COUNT = 7 then
                A(7) <- A(7), A(6:0).Q <- A.Q(7:1), goto OUTPUT
            fi;

        RSHIFT:
            A(7) <- A(7), A(6:0).Q.Q_1 <- A.Q, COUNT <- COUNT + 1, goto SCAN;

        OUTPUT:
            OUT <- A;
            OUT <- Q;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 12 * 7
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("12").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("7").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("84").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );

    simulator.reset();

    // 31 * 47
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("31").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("47").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("177").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("5").unwrap()
    );
}
