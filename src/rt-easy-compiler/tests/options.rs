mod util;

use rt_easy_compiler::Options;

const SOURCE: &'static str = r#"
declare input IN(2:17)
declare output OUT

declare register A(7:0), B
declare register AR(7:0), DR(31:0)
declare bus BUS(7), BUS2(31:0)

declare memory MEM(AR,DR)
declare register array ARRAY(2:0)[32]

START:
    A <- B + B, read MEM;
    write MEM, ARRAY[IN(2:6) + 1] <- B and BUS;
    A <- B.B."0".BUS(7).IN(6);
    BUS(7).BUS2 <- 0xF, A.ARRAY[2].OUT <- sxt 0b1001;
    goto MAIN;

MAIN:
    if OUT <> IN then nop else goto START fi;
    if A = 0 then
        if B = 0 then goto LOOP fi
    fi, nop;

LOOP:
    switch AR {
        case 0 + 1: nop, goto START
        case 2 and 2: nop
        case -1: if B = 0 then goto LOOP fi
        default: goto LOOP
    };
    goto LOOP;
"#;

#[test]
fn compiler_options() {
    util::check_with_options(SOURCE, &Options { print_mir_unordered: true, print_mir: true });
}
