// hormone-conversion-calculator
// Jana Marie Hemsing - 2022

extern crate nom;

use nom::{
    bytes::complete::{take_till, take_while, take_while1},
    error::context,
    Finish};

enum Prefix {
    Yocto,
    Zepto,
    Atto,
    Femto,
    Pico,
    Nano,
    Mikro,
    Milli,
    Centi,
    Deci,
    Deka,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
}

enum Unit {
    Gram,
    Mole,
    Litre,
}

enum Conjunction {
    To,
    In,
    Arrow,
}

#[derive(Debug, PartialEq)]
enum Hormone {
    Testosterone,
    Estradiol,
    Progesterone,
}

struct UnitSingle {
    prefix: Option<Prefix>,
    unit: Unit,
}

struct UnitRatio {
    numerator: UnitSingle,
    denominator: UnitSingle,
}

struct Expression {
    hormone: Hormone,
    unit_in: UnitRatio,
    unit_out: UnitRatio,
    in_val: f64,
}

impl From<&str> for Conjunction {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "in" => Conjunction::In,
            "to" => Conjunction::To,
            "->"|">" => Conjunction::Arrow,
            _ => unimplemented!("no other Hormones supported")
        }
    }
}

impl From<&str> for Hormone {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "t"|"testo"|"testosterone" => Hormone::Testosterone,
            "e2"|"estradiol" => Hormone::Estradiol,
            "p4"|"prog"|"progesterone" => Hormone::Progesterone,
            _ => unimplemented!("no other Hormones supported")
        }
    }
}

impl From<&str> for Prefix {
    fn from(i: &str) -> Self {
        match i {
            "y" => Prefix::Yocto,
            "z" => Prefix::Zepto,
            "a" => Prefix::Atto,
            "f" => Prefix::Femto,
            "p" => Prefix::Pico,
            "n" => Prefix::Nano,
            "u"|"µ" => Prefix::Mikro,
            "m" => Prefix::Milli,
            "c" => Prefix::Centi,
            "d" => Prefix::Deci,
            "da" => Prefix::Deka,
            "h" => Prefix::Hecto,
            "k" => Prefix::Kilo,
            "M" => Prefix::Mega,
            "G" => Prefix::Giga,
            "T" => Prefix::Tera,
            "P" => Prefix::Peta,
            "E" => Prefix::Exa,
            "Z" => Prefix::Zetta,
            "Y" => Prefix::Yotta,
            _ => unimplemented!("no other Prefixes supported")
        }
    }
}

impl From<&str> for Unit {
    fn from(i: &str) -> Self {
        match i {
            "g" => Unit::Gram,
            "mol" => Unit::Mole,
            "l"|"L"|"ℓ" => Unit::Litre,
            _ => unimplemented!("no other Units supported")
        }
    }
}

fn hormone(i: &str) -> nom::IResult<&str, Hormone> {
    context(
        "Hormone",
        take_till(|c| c == ' '))(i).map(|(next_i, res)| (next_i, res.into()))
}

fn space1(i: &str) -> nom::IResult<&str, &str> {
    take_while1(|c| c == ' ')(i)
}

fn space(i: &str) -> nom::IResult<&str, &str> {
    take_while(|c| c == ' ')(i)
}

/*
fn hcc_parser(i: &str) -> nom::IResult<&str, (Hormone, &str)> {
    let h = hormone(i);
    let _ = take_while(|c| c == ' ')(i);
}
*/

fn main() {
    //hcc_parser(data);
}

#[test]
fn test_hormone() {
    assert_eq!(hormone("Testosterone 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Testosterone)));
    assert_eq!(hormone("t 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Testosterone)));
    assert_eq!(hormone("Testo 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Testosterone)));
    assert_ne!(hormone("Testosterone 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to dg/dl", Hormone::Testosterone)));
    assert_ne!(hormone("Testosterone 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Estradiol)));

    assert_eq!(hormone("Estradiol 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Estradiol)));
    assert_eq!(hormone("E2 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Estradiol)));

    assert_eq!(hormone("P4 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Progesterone)));
    assert_eq!(hormone("Progesterone 1.8nmol/l to ng/dl"), Ok((" 1.8nmol/l to ng/dl", Hormone::Progesterone)));
}

#[test]
fn test_space() {
    assert_eq!(space1(" 1.8nmol/l to ng/dl"), Ok(("1.8nmol/l to ng/dl", " ")));
    assert_eq!(space1("  1.8nmol/l to ng/dl"), Ok(("1.8nmol/l to ng/dl", "  ")));
    assert_ne!(space1("1.8nmol/l to ng/dl"), Ok(("1.8nmol/l to ng/dl", " ")));

    assert_eq!(space(" 1.8nmol/l to ng/dl"), Ok(("1.8nmol/l to ng/dl", " ")));
    assert_ne!(space("1.8nmol/l to ng/dl"), Ok(("1.8nmol/l to ng/dl", " ")));
}
