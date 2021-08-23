fn main() {}

/*use rt_easy_simulator::Simulator;
use rtcore::{program::*, value::Value};

fn main() {
    let program = program();
    let mut simulator = Simulator::init(program);

    // A
    simulator
        .write_into_bus(
            &Bus { ident: Ident("INBUS".to_string()), range: None },
            Value::parse_dec("3").unwrap(),
        )
        .unwrap();
    simulator.step().unwrap();

    // FACTOR
    simulator
        .write_into_bus(
            &Bus { ident: Ident("INBUS".to_string()), range: None },
            Value::parse_dec("7").unwrap(),
        )
        .unwrap();
    simulator.step().unwrap();

    // Run to end
    while !simulator.is_finished() {
        simulator.micro_step().unwrap();
    }

    assert_eq!(
        simulator.state().read_register(&Ident("RES".to_string()), None).unwrap(),
        Value::parse_dec("21").unwrap()
    );

    println!("{:#?}", simulator.state());
}

fn program() -> Program {
    let declarations = Declarations {
        registers: vec![
            Register { ident: Ident("A".to_string()), range: Some(BitRange { msb: 7, lsb: 0 }) },
            Register {
                ident: Ident("FACTOR".to_string()),
                range: Some(BitRange { msb: 7, lsb: 0 }),
            },
            Register { ident: Ident("RES".to_string()), range: Some(BitRange { msb: 7, lsb: 0 }) },
        ],
        buses: vec![
            Bus { ident: Ident("INBUS".to_string()), range: Some(BitRange { msb: 7, lsb: 0 }) },
            Bus { ident: Ident("OUTBUS".to_string()), range: Some(BitRange { msb: 7, lsb: 0 }) },
        ],
        memories: vec![],
    };
    let statements = vec![
        Statement {
            label: Some(Label("BEGIN".to_string())),
            kind: StatementKind::Simple {
                operations: vec![
                    Operation {
                        kind: OperationKind::Assignment(
                            Assignment::AssignToReg {
                                lhs: Either::Left(Register {
                                    ident: Ident("A".to_string()),
                                    range: None,
                                }),
                                rhs: Expression::Atom(Atom::Bus(Bus {
                                    ident: Ident("INBUS".to_string()),
                                    range: None,
                                })),
                            },
                            (),
                        ),
                        span: 0..0,
                    },
                    Operation {
                        kind: OperationKind::Assignment(
                            Assignment::AssignToReg {
                                lhs: Either::Left(Register {
                                    ident: Ident("RES".to_string()),
                                    range: None,
                                }),
                                rhs: Expression::Atom(Atom::Number(Number {
                                    value: Value::zero(1),
                                })),
                            },
                            (),
                        ),
                        span: 0..0,
                    },
                ],
            },
            span: 0..0,
        },
        Statement {
            label: None,
            kind: StatementKind::Simple {
                operations: vec![Operation {
                    kind: OperationKind::Assignment(
                        Assignment::AssignToReg {
                            lhs: Either::Left(Register {
                                ident: Ident("FACTOR".to_string()),
                                range: None,
                            }),
                            rhs: Expression::Atom(Atom::Bus(Bus {
                                ident: Ident("INBUS".to_string()),
                                range: None,
                            })),
                        },
                        (),
                    ),
                    span: 0..0,
                }],
            },
            span: 0..0,
        },
        Statement {
            label: Some(Label("LOOP".to_string())),
            kind: StatementKind::Simple {
                operations: vec![Operation {
                    kind: OperationKind::If(If {
                        condition: Expression::BinaryTerm(Box::new(BinaryTerm {
                            lhs: Expression::Atom(Atom::Register(Register {
                                ident: Ident("FACTOR".to_string()),
                                range: None,
                            })),
                            rhs: Expression::Atom(Atom::Number(Number { value: Value::zero(1) })),
                            operator: BinaryOperator::Ne,
                        })),
                        operations_if: vec![
                            Operation {
                                kind: OperationKind::Assignment(
                                    Assignment::AssignToReg {
                                        lhs: Either::Left(Register {
                                            ident: Ident("RES".to_string()),
                                            range: None,
                                        }),
                                        rhs: Expression::BinaryTerm(Box::new(BinaryTerm {
                                            lhs: Expression::Atom(Atom::Register(Register {
                                                ident: Ident("RES".to_string()),
                                                range: None,
                                            })),
                                            rhs: Expression::Atom(Atom::Register(Register {
                                                ident: Ident("A".to_string()),
                                                range: None,
                                            })),
                                            operator: BinaryOperator::Add,
                                        })),
                                    },
                                    (),
                                ),
                                span: 0..0,
                            },
                            Operation {
                                kind: OperationKind::Assignment(
                                    Assignment::AssignToReg {
                                        lhs: Either::Left(Register {
                                            ident: Ident("FACTOR".to_string()),
                                            range: None,
                                        }),
                                        rhs: Expression::BinaryTerm(Box::new(BinaryTerm {
                                            lhs: Expression::Atom(Atom::Register(Register {
                                                ident: Ident("FACTOR".to_string()),
                                                range: None,
                                            })),
                                            // TODO: Fixme use Sub here
                                            rhs: Expression::Atom(Atom::Number(Number {
                                                value: Value::filled(8),
                                            })),
                                            operator: BinaryOperator::Add,
                                        })),
                                    },
                                    (),
                                ),
                                span: 0..0,
                            },
                            Operation {
                                kind: OperationKind::Goto(Goto {
                                    label: Label("LOOP".to_string()),
                                }),
                                span: 0..0,
                            },
                        ],
                        operations_else: vec![Operation {
                            kind: OperationKind::Assignment(
                                Assignment::AssignToBus {
                                    lhs: Either::Left(Bus {
                                        ident: Ident("OUTBUS".to_string()),
                                        range: None,
                                    }),
                                    rhs: Expression::Atom(Atom::Register(Register {
                                        ident: Ident("RES".to_string()),
                                        range: None,
                                    })),
                                },
                                (),
                            ),
                            span: 0..0,
                        }],
                    }),
                    span: 0..0,
                }],
            },
            span: 0..0,
        },
    ];
    Program::new_unchecked(declarations, statements, None)
}*/
