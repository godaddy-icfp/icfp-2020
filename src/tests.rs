use super::ast::Symbol::*;

#[test]
fn run_inc_1() {
    let symbol = super::run(":1096 = ap inc 1");
    dbg!(&symbol);

    assert_eq!(symbol, Lit(2))
}

#[test]
fn run_inc_var() {
    let symbol = super::run(
        ":1096 = ap inc 1
         :302 = ap inc :1096",
    );
    dbg!(&symbol);

    assert_eq!(symbol, Lit(3))
}

#[test]
fn run_start() {
    let symbol = super::run(
        ":1029 = ap ap cons 7 ap ap cons 123229502148636 nil
:1030 = ap ap cons 2 ap ap cons 7 nil
:1031 = ap ap cons 4 ap ap cons 21855 nil
:1032 = ap ap cons 7 ap ap cons 560803991675135 nil
:1034 = ap ap cons 5 ap ap cons 33554431 nil
:1035 = ap ap cons 5 ap ap cons 30309607 nil
:1036 = ap ap cons 3 ap ap cons 463 nil
:1037 = ap ap cons 4 ap ap cons 48063 nil
:1038 = ap ap cons 7 ap ap cons 10880 nil
:1039 = ap ap cons 5 ap ap cons 15265326 nil
:1040 = ap ap cons 5 ap ap cons 18472561 nil
:1041 = ap ap cons 4 ap ap cons 64959 nil
:1042 = ap ap cons 4 ap ap cons 63935 nil",
    );
    dbg!(&symbol);

    assert_eq!(
        symbol,
        Pair(Lit(4).into(), Pair(Lit(63935).into(), Nil.into()).into())
    )
}

#[test]
fn run_galaxy() {
    let symbol = super::run(include_str!("../data/galaxy.txt"));
    dbg!(&symbol);

    assert_eq!(
        symbol,
        Pair(Lit(4).into(), Pair(Lit(63935).into(), Nil.into()).into())
    )
}
