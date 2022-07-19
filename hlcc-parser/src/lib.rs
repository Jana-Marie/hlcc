// hlcc-parser -  hormone level conversion calculator parser library
// Copyright (C) 2022 Jana Marie Hemsing

extern crate nom;

use nom::{
    bytes::complete::{take_till, take_while, take_while1, tag_no_case, tag},
    error::context,
    sequence::{tuple, pair},
    branch::alt,
    number::complete::double,
    combinator::peek,
    };
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Unit {
    Gram,
    Mole,
    Litre,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Hormone {
    Cholesterol,
    Testosterone,
    Dihydrotestosterone,
    Dehydroepiandrosterone,
    Estrone,
    Estradiol,
    Estriol,
    Estetrol,
    Progesterone,
    Aldosterone,
    Androstenedione,
    Cortisol,
    Gonadorelin,
    Fsh,
    Lh,
    Thyrotropin,
    Shbg,
    Prolactin,
    Thyroxine,
    Triiodothyronine,
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
            "cholesterol"|"cholesterin"     => Hormone::Cholesterol,
            "t"|"testo"|"testosterone"      => Hormone::Testosterone,
            "dht"|"dihydrotestosterone"     => Hormone::Dihydrotestosterone,
            "dhea"|"dehydroepiandrosterone"|"androstenolone" => Hormone::Dehydroepiandrosterone,
            "e1"|"estrone"|"oestrone"       => Hormone::Estrone,
            "e2"|"estradiol"|"oestradiol"   => Hormone::Estradiol,
            "e3"|"estriol"|"oestriol"       => Hormone::Estriol,
            "e4"|"estetrol"|"oestetrol"     => Hormone::Estetrol,
            "p4"|"prog"|"progesterone"      => Hormone::Progesterone,
            "aldosterone"|"aldocorten"      => Hormone::Aldosterone,
            "a4"|"androstenedione"          => Hormone::Androstenedione,
            "cortisol"                      => Hormone::Cortisol,
            "gonadorelin"|"gnrh"            => Hormone::Gonadorelin,
            "fsh"                           => Hormone::Fsh,
            "lh"|"lutropin"|"lutrophin"     => Hormone::Lh,
            "tsh"|"thyrotropin"             => Hormone::Thyrotropin,
            "shbg"|"abp"|"sbp"|"tebg"       => Hormone::Shbg,
            "prolactin"|"prl"|"lth"         => Hormone::Prolactin,
            "t4"|"thyroxine"                => Hormone::Thyroxine,
            "t3"|"triiodothyronine"         => Hormone::Triiodothyronine,
            _                               => unimplemented!("no other Hormones supported")
        }
    }
}

impl From<Hormone> for f64 {
    fn from(i: Hormone) -> Self {
        match i {
            Hormone::Cholesterol            => 386.65,
            Hormone::Testosterone           => 288.431,
            Hormone::Dihydrotestosterone    => 290.447,
            Hormone::Dehydroepiandrosterone => 288.424
            Hormone::Estrone                => 270.336,
            Hormone::Estradiol              => 272.38,
            Hormone::Estriol                => 288.387,
            Hormone::Estetrol               => 304.386,
            Hormone::Progesterone           => 314.469,
            Hormone::Aldosterone            => 360.450,
            Hormone::Androstenedione        => 286.415,
            Hormone::Cortisol               => 362.460,
            Hormone::Gonadorelin            => 1182.311,
            Hormone::Fsh                    => 30.0, // not sure if this is correct, mass is 30kDa
            Hormone::Lh                     => 33.0, // again 33kDa
            Hormone::Thyrotropin            => 28.0, // again 28kDa
            Hormone::Shbg                   => 43.7, // again 43.7kDa
            Hormone::Prolactin              => 22.892, // again 22.9kDa
            Hormone::Thyroxine              => 776.87,
            Hormone::Triiodothyronine       => 650.977,
        }
    }
}

impl From<Hormone> for &str {
    fn from(i: Hormone) -> Self {
        match i {
            Hormone::Cholesterol            => "Cholesterol",
            Hormone::Testosterone           => "Testosterone",
            Hormone::Dihydrotestosterone    => "Dihydrotestosterone",
            Hormone::Dehydroepiandrosterone => "Dehydroepiandrosterone"
            Hormone::Estrone                => "Estrone",
            Hormone::Estradiol              => "Estradiol",
            Hormone::Estriol                => "Estriol",
            Hormone::Estetrol               => "Estetrol",
            Hormone::Progesterone           => "Progesterone",
            Hormone::Aldosterone            => "Aldosterone",
            Hormone::Androstenedione        => "Androstenedione",
            Hormone::Cortisol               => "Cortisol",
            Hormone::Gonadorelin            => "Gonadorelin",
            Hormone::Fsh                    => "Follicle-stimulating hormone",
            Hormone::Lh                     => "Luteinising hormone",
            Hormone::Thyrotropin            => "Thyroid-stimulating hormone",
            Hormone::Shbg                   => "Sex hormone-binding globulin",
            Hormone::Prolactin              => "Prolactin",
            Hormone::Thyroxine              => "Thyroxine",
            Hormone::Triiodothyronine       => "Triiodothyronine",
        }
    }
}

impl fmt::Display for Hormone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}

impl fmt::Display for UnitSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", <&str>::from(self.prefix), <&str>::from(self.unit))
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

impl From<Prefix> for &str {
    fn from(i: Prefix) -> Self {
        match i {
            Prefix::Yocto => "y",
            Prefix::Zepto => "z",
            Prefix::Atto  => "a",
            Prefix::Femto => "f",
            Prefix::Pico  => "p",
            Prefix::Nano  => "n",
            Prefix::Mikro => "µ",
            Prefix::Milli => "m",
            Prefix::Centi => "c",
            Prefix::Deci  => "d",
            Prefix::None  => "",
            Prefix::Deka  => "da",
            Prefix::Hecto => "h",
            Prefix::Kilo  => "k",
            Prefix::Mega  => "M",
            Prefix::Giga  => "G",
            Prefix::Tera  => "T",
            Prefix::Peta  => "P",
            Prefix::Exa   => "E",
            Prefix::Zetta => "Z",
            Prefix::Yotta => "Y",
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

impl From<Unit> for &str {
    fn from(i: Unit) -> Self {
        match i {
            Unit::Gram  => "g",
            Unit::Mole  => "mol",
            Unit::Litre => "ℓ",
            Unit::None  => "",
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
            tag("da"),
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

fn compute_result(expression: &Expression) -> f64 {
    let prefix_calculation = (i32::from(expression.unit_in.numerator.prefix) + i32::from(expression.unit_out.denominator.prefix)) - (i32::from(expression.unit_in.denominator.prefix) + i32::from(expression.unit_out.numerator.prefix)); // use fixed point math to not loose precision
    match (expression.unit_in.numerator.unit, expression.unit_out.numerator.unit) { // very ugly math engine, does not allow all calculations, probably just proof of concept
        (Unit::Mole, Unit::Gram) => f64::powf(10.0, prefix_calculation.into()) * (f64::from(expression.in_val) * f64::from(expression.hormone)),
        (Unit::Gram, Unit::Mole) => f64::powf(10.0, prefix_calculation.into())  * (f64::from(expression.in_val) / f64::from(expression.hormone)),
        (Unit::Gram, Unit::Gram) | (Unit::Mole, Unit::Mole) => f64::powf(10.0, prefix_calculation.into())  * f64::from(expression.in_val),
        (_,_) => 0.0,
    }
}

/// Computes result of conversion expression
pub fn compute(input: &str) -> Option<String> {
    if let Ok((_, expression)) = hcc_parser(input) {
        Some(format!("{:.03} {}/{} {}", compute_result(&expression), expression.unit_out.numerator, expression.unit_out.denominator, expression.hormone))
    } else {
        None
    }
}
