mod util;

const SOURCE: &'static str = r#"
declare register A(7:0), FACTOR(7:0), RES(7:0)
declare input INPUT(7:0)
declare output OUTPUT(7:0)

BEGIN:
    A <- INPUT, RES <- 0;
    FACTOR <- INPUT;
LOOP:
    if FACTOR <> 0 then
        RES <- RES + A, FACTOR <- FACTOR - 1, goto LOOP
    else
        OUTPUT <- RES
    fi;
"#;

const SOURCE_UNIT_TEST: &'static str = r#"
# mult 4 * 7
INPUT <- 4
step
INPUT <- 7
run

assert OUTPUT + OUTPUT > 30
assert OUTPUT = 28

# Misc
reset
microStep 2
step 10

# mult 3 * 5
reset
set breakpoint BEGIN
remove breakpoint BEGIN
set breakpoint BEGIN
run
INPUT <- 3
step
INPUT <- 5
run
assert OUTPUT = 15
"#;

#[test]
fn mult() {
    let program = util::compile(SOURCE);
    let unit_test = util::compile_unit_test(SOURCE_UNIT_TEST);

    assert!(rt_easy_unit_test::run(program, unit_test).is_ok());
}
