// hormone-level-conversion-calculator
// Jana Marie Hemsing - 2022

extern crate nom;

use wasm_bindgen::prelude::*;
use nom::{
    bytes::complete::{take_till, take_while, take_while1, tag_no_case, tag},
    error::context,
    sequence::{tuple, pair},
    branch::alt,
    number::complete::double,
    combinator::peek,
    };
use std::env;

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
            "t"|"testo"|"testosterone"  => Hormone::Testosterone,
            "e2"|"estradiol"            => Hormone::Estradiol,
            "p4"|"prog"|"progesterone"  => Hormone::Progesterone,
            _                           => unimplemented!("no other Hormones supported")
        }
    }
}

impl From<Hormone> for f64 {
    fn from(i: Hormone) -> Self {
        match i {
            Hormone::Testosterone   => 288.42442,
            Hormone::Estradiol      => 272.38,
            Hormone::Progesterone   => 314.46,
        }
    }
}

impl From<&str> for Prefix {
    fn from(i: &str) -> Self {
        match i {
            "y"     => Prefix::Yocto,
            "z"     => Prefix::Zepto,
            "a"     => Prefix::Atto,
            "f"     => Prefix::Femto,
            "p"     => Prefix::Pico,
            "n"     => Prefix::Nano,
            "u"|"µ" => Prefix::Mikro,
            "m"     => Prefix::Milli,
            "c"     => Prefix::Centi,
            "d"     => Prefix::Deci,
            ""      => Prefix::None,
            "da"    => Prefix::Deka,
            "h"     => Prefix::Hecto,
            "k"     => Prefix::Kilo,
            "M"     => Prefix::Mega,
            "G"     => Prefix::Giga,
            "T"     => Prefix::Tera,
            "P"     => Prefix::Peta,
            "E"     => Prefix::Exa,
            "Z"     => Prefix::Zetta,
            "Y"     => Prefix::Yotta,
            _       => unimplemented!("no other Prefixes supported")
        }
    }
}

impl From<Prefix> for i32 {
    fn from(i: Prefix) -> Self {
        match i {
            Prefix::Yocto => -24,
            Prefix::Zepto => -21,
            Prefix::Atto  => -18,
            Prefix::Femto => -15,
            Prefix::Pico  => -12,
            Prefix::Nano  => -9,
            Prefix::Mikro => -6,
            Prefix::Milli => -3,
            Prefix::Centi => -2,
            Prefix::Deci  => -1,
            Prefix::None  =>  0,
            Prefix::Deka  =>  1,
            Prefix::Hecto =>  2,
            Prefix::Kilo  =>  3,
            Prefix::Mega  =>  6,
            Prefix::Giga  =>  9,
            Prefix::Tera  =>  12,
            Prefix::Peta  =>  15,
            Prefix::Exa   =>  18,
            Prefix::Zetta =>  21,
            Prefix::Yotta =>  24,
        }
    }
}

impl From<&str> for Unit {
    fn from(i: &str) -> Self {
        match i {
            "g"         => Unit::Gram,
            "mol"       => Unit::Mole,
            "l"|"L"|"ℓ" => Unit::Litre,
            _           => Unit::None // ugly, should panic, but no idea how to solve else
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
            tag("y"),
            tag("z"),
            tag("a"),
            tag("f"),
            tag("p"),
            tag("n"),
            tag("u"),
            tag("µ"),
            tag("m"),
            tag("c"),
            tag("d"),
            tag("da"),
            tag("h"),
            tag("k"),
            tag("M"),
            tag("G"),
            tag("T"),
            tag("P"),
            tag("E"),
            tag("Z"),
            tag("Y"),
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
        (
            next_i,
            Expression {
                hormone,
                unit_in: UnitRatio {
                    numerator: UnitSingle {
                        prefix: unit_in_num.0,
                        unit: unit_in_num.1},
                    denominator: UnitSingle {
                        prefix: unit_in_den.0,
                        unit: unit_in_den.1}},
                unit_out: UnitRatio {
                    numerator: UnitSingle {
                        prefix: unit_out_num.0,
                        unit: unit_out_num.1},
                    denominator: UnitSingle {
                        prefix: unit_out_den.0,
                        unit: unit_out_den.1}},
                in_val,
            },
        )
    })
}

fn compute_result(expression: Expression) -> f64 {
    let prefix_calculation = (i32::from(expression.unit_in.numerator.prefix) + i32::from(expression.unit_out.denominator.prefix)) - (i32::from(expression.unit_in.denominator.prefix) + i32::from(expression.unit_out.numerator.prefix)); // use fixed point math to not loose precision
    match (expression.unit_in.numerator.unit, expression.unit_out.numerator.unit) { // very ugly math engine, does not allow all calculations, probably just prrof of concept
        (Unit::Mole, Unit::Gram) => f64::powf(10.0, prefix_calculation.into()) * (f64::from(expression.in_val) * f64::from(expression.hormone)),
        (Unit::Gram, Unit::Mole) => f64::powf(10.0, prefix_calculation.into())  * (f64::from(expression.in_val) / f64::from(expression.hormone)),
        (_,_) => 0.0,
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compute_input(x: String) -> f64{
    if let Ok((_, expression)) = hcc_parser(&x) {
        let value_out = compute_result(expression);
        println!("Result: {:.2}", value_out);
        value_out
    } else {
        -1.0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);

    if let Ok((_, expression)) = hcc_parser(&args[1]) {
        let value_out = compute_result(expression);
        println!("Computes to: {:.3}", value_out);
    } else {
        println!("Yet unknown error!");
    }
}
