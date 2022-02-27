// hormone-conversion-calculator
// Jana Marie Hemsing - 2022

extern crate nom;

use nom::{
    bytes::complete::{take_till, take_while, take_while1, tag_no_case, tag},
    error::context,
    sequence::{tuple, pair},
    branch::alt,
    number::complete::double,
    combinator::peek,
    };

#[derive(Debug, PartialEq)]
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
    None,
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

#[derive(Debug, PartialEq)]
enum Unit {
    Gram,
    Mole,
    Litre,
    None,
}

#[derive(Debug, PartialEq)]
enum Hormone {
    Testosterone,
    Estradiol,
    Progesterone,
}

#[derive(Debug, PartialEq)]
struct UnitSingle {
    prefix: Prefix,
    unit: Unit,
}

#[derive(Debug, PartialEq)]
struct UnitRatio {
    numerator: UnitSingle,
    denominator: UnitSingle,
}

#[derive(Debug, PartialEq)]
struct Expression {
    hormone: Hormone,
    unit_in: UnitRatio,
    unit_out: UnitRatio,
    in_val: f64,
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
            ""  => Prefix::None,
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
            _ => Unit::None // ugly, should panic, but no idea how to solve else
        }
    }
}

fn hormone(i: &str) -> nom::IResult<&str, Hormone> {
    context(
        "Hormone",
        take_till(|c| c == ' '))(i).map(|(next_i, res)| (next_i, res.into()))
}

fn value(i: &str) -> nom::IResult<&str, f64> {
    double(i)
}

fn prefix_dummy(i: &str) -> nom::IResult<&str, Prefix> {
    Ok((i, Prefix::None))
}

fn prefix(i: &str) -> nom::IResult<&str, Prefix> {
    context(
        "Prefix",
        alt((
            tag("p"),
            tag("n"),
            tag("u"),
            tag("µ"),
            tag("m"),
            tag("c"),
            tag("d"),
        )))(i).map(|(next_i, res)| (next_i, res.into()))
}

fn fractional_bar(i: &str) -> nom::IResult<&str, &str> {
    tag("/")(i)
}

fn is_unit(i: &str) -> nom::IResult<&str, Unit> {
    peek(context(
        "Unit",
        take_till(|c| c == ' ' || c == '/')))(i).map(|(next_i, res)| (next_i, res.into()))
}

fn unit(i: &str) -> nom::IResult<&str, Unit> {
    context(
        "Unit",
        take_till(|c| c == ' ' || c == '/'))(i).map(|(next_i, res)| (next_i, res.into()))
}

fn unit_prefixed(i: &str) -> nom::IResult<&str, (Prefix, Unit)> { // todo return UnitSingle instead of touple
    match is_unit(i) {
        Ok((_, Unit::None)) => pair(prefix, unit)(i),
        Ok(_) => pair(prefix_dummy, unit)(i),
        Err(_) => unimplemented!("Weird Error, this should never happen."),
    }
}

fn conjunction(i: &str) -> nom::IResult<&str, &str> {
    alt((
        tag_no_case("in"),
        tag_no_case("to"),
        tag("->"),
        tag(">"),
    ))(i)
}

fn space1(i: &str) -> nom::IResult<&str, &str> {
    take_while1(|c| c == ' ')(i)
}

fn space(i: &str) -> nom::IResult<&str, &str> {
    take_while(|c| c == ' ')(i)
}


fn hcc_parser(i: &str) -> nom::IResult<&str, Expression> {
    context(
        "Expression",
        tuple((
            hormone,
            space1,
            value,
            space,
            unit_prefixed,
            fractional_bar,
            unit_prefixed,
            space1,
            conjunction,
            space1,
            unit_prefixed,
            fractional_bar,
            unit_prefixed,
        )),
    )(i).map(|(next_i, res)| {
        let (hormone, _, in_val, _, unit_in_num, _, unit_in_den, _, _, _,  unit_out_num, _, unit_out_den) = res;
        let unit_in = UnitRatio {
            numerator: UnitSingle {
                prefix: unit_in_num.0,
                unit: unit_in_num.1},
            denominator: UnitSingle {
                prefix: unit_in_den.0,
                unit: unit_in_den.1}};
        let unit_out = UnitRatio {
            numerator: UnitSingle {
                prefix: unit_out_num.0,
                unit: unit_out_num.1},
            denominator: UnitSingle {
                prefix: unit_out_den.0,
                unit: unit_out_den.1}};
        (
            next_i,
            Expression {
                hormone,
                unit_in,
                unit_out,
                in_val,
            },
        )
    })
}

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

#[test]
fn test_conjunction() {
    assert_eq!(conjunction("in ng/dl"), Ok((" ng/dl", "in")));
    assert_eq!(conjunction("to ng/dl"), Ok((" ng/dl", "to")));
    assert_eq!(conjunction("> ng/dl"), Ok((" ng/dl", ">")));
    assert_eq!(conjunction("-> ng/dl"), Ok((" ng/dl", "->")));

    assert_ne!(conjunction(" in ng/dl"), Ok((" ng/dl", " in")));
    assert_ne!(conjunction(" to ng/dl"), Ok((" ng/dl", " to")));
    assert_ne!(conjunction(" > ng/dl"), Ok((" ng/dl", " >")));
    assert_ne!(conjunction(" -> ng/dl"), Ok((" ng/dl", " ->")));

    assert_ne!(conjunction(" in ng/dl"), Ok((" ng/dl", "in")));
    assert_ne!(conjunction(" to ng/dl"), Ok((" ng/dl", "to")));
    assert_ne!(conjunction(" > ng/dl"), Ok((" ng/dl", ">")));
    assert_ne!(conjunction(" -> ng/dl"), Ok((" ng/dl", "->")));

    assert_ne!(conjunction("nope ng/dl"), Ok((" ng/dl", "nope")));
}

#[test]
fn test_number() {
    assert_eq!(value("1.8nmol/l to ng/dl"), Ok(("nmol/l to ng/dl", 1.8)));
    assert_eq!(value("1.8"), Ok(("", 1.8)));

    assert_ne!(value(" 1.8"), Ok(("", 1.8)));
}

#[test]
fn test_fraction_bar() {
    assert_eq!(fractional_bar("/l to ng/dl"), Ok(("l to ng/dl", "/")));
    assert_eq!(fractional_bar("/"), Ok(("", "/")));

    assert_ne!(fractional_bar("l to ng/dl"), Ok(("l to ng/dl", "")));
    assert_ne!(fractional_bar("l to ng/dl"), Ok(("l to ng/dl", "/")));
}

#[test]
fn test_prefix() {
    assert_eq!(prefix("nmol/l to ng/dl"), Ok(("mol/l to ng/dl", Prefix::Nano)));
}

#[test]
fn test_unit() {
    assert_eq!(unit("mol "), Ok((" ", Unit::Mole)));
    assert_eq!(unit("mol/dl"), Ok(("/dl", Unit::Mole)));
    //assert_ne!(unit("nmol/dl"), Ok(("nmol/dl", Unit::Mole)));
}

#[test]
fn test_unit_prefixed() {
    assert_eq!(unit_prefixed("mol/dl"), Ok(("/dl", (Prefix::None, Unit::Mole))));
    assert_eq!(unit_prefixed("g/dl"), Ok(("/dl", (Prefix::None, Unit::Gram))));
    assert_eq!(unit_prefixed("l/dl"), Ok(("/dl", (Prefix::None, Unit::Litre))));
    assert_eq!(unit_prefixed("L/dl"), Ok(("/dl", (Prefix::None, Unit::Litre))));
    assert_eq!(unit_prefixed("ℓ/dl"), Ok(("/dl", (Prefix::None, Unit::Litre))));

    assert_ne!(unit_prefixed("A/dl"), Ok(("/dl", (Prefix::None, Unit::Litre))));

    assert_eq!(unit_prefixed("nmol/dl"), Ok(("/dl", (Prefix::Nano, Unit::Mole))));
    assert_eq!(unit_prefixed("ng/dl"), Ok(("/dl", (Prefix::Nano, Unit::Gram))));
    assert_eq!(unit_prefixed("pl/dl"), Ok(("/dl", (Prefix::Pico, Unit::Litre))));
    assert_eq!(unit_prefixed("pL/dl"), Ok(("/dl", (Prefix::Pico, Unit::Litre))));
    assert_eq!(unit_prefixed("mℓ/dl"), Ok(("/dl", (Prefix::Milli, Unit::Litre))));

    assert_ne!(unit_prefixed("Ottl/dl"), Ok(("/dl", (Prefix::None, Unit::Litre))));
}

#[test]
fn test_hcc() {
    assert_eq!(hcc_parser("Testo 1.8nmol/l in pg/ml"), Ok(("", Expression {
                hormone: Hormone::Testosterone,
                unit_in: UnitRatio {
                    numerator: UnitSingle {
                        prefix: Prefix::Nano,
                        unit: Unit::Mole},
                    denominator: UnitSingle {
                        prefix: Prefix::None,
                        unit: Unit::Litre}},
                unit_out: UnitRatio {
                    numerator: UnitSingle {
                        prefix: Prefix::Pico,
                        unit: Unit::Gram},
                    denominator: UnitSingle {
                        prefix: Prefix::Milli,
                        unit: Unit::Litre}},
                in_val: 1.8,
            }
        ))
    );

    assert_eq!(hcc_parser("E2 111pg/ml > nmol/dl"), Ok(("", Expression {
                hormone: Hormone::Estradiol,
                unit_in: UnitRatio {
                    numerator: UnitSingle {
                        prefix: Prefix::Pico,
                        unit: Unit::Gram},
                    denominator: UnitSingle {
                        prefix: Prefix::Milli,
                        unit: Unit::Litre}},
                unit_out: UnitRatio {
                    numerator: UnitSingle {
                        prefix: Prefix::Nano,
                        unit: Unit::Mole},
                    denominator: UnitSingle {
                        prefix: Prefix::Deci,
                        unit: Unit::Litre}},
                in_val: 111.0,
            }
        ))
    );
}
