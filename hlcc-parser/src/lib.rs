// hlcc-parser -  hormone level conversion calculator parser library
// Copyright (C) 2022 Jana Marie Hemsing
// Copyright (C) 2023 Yureka Lilian

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Prefix {
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
pub(crate) enum Unit {
    Gram,
    Mole,
    Litre,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Hormone {
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
    Cholecalciferol,
    Cobalamin,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UnitSingle {
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

impl From<Hormone> for f64 {
    fn from(i: Hormone) -> Self {
        match i {
            Hormone::Cholesterol            => 386.65,
            Hormone::Testosterone           => 288.431,
            Hormone::Dihydrotestosterone    => 290.447,
            Hormone::Dehydroepiandrosterone => 288.424,
            Hormone::Estrone                => 270.336,
            Hormone::Estradiol              => 272.38,
            Hormone::Estriol                => 288.387,
            Hormone::Estetrol               => 304.386,
            Hormone::Progesterone           => 314.469,
            Hormone::Aldosterone            => 360.450,
            Hormone::Androstenedione        => 286.415,
            Hormone::Cortisol               => 362.460,
            Hormone::Gonadorelin            => 1182.311,
            Hormone::Fsh                    => 30.0, // not sure if this is correct, mass is 30kDa, should be
            Hormone::Lh                     => 33.0, // again 33kDa
            Hormone::Thyrotropin            => 28.0, // again 28kDa
            Hormone::Shbg                   => 43.7, // again 43.7kDa
            Hormone::Prolactin              => 22.892, // again 22.9kDa
            Hormone::Thyroxine              => 776.87,
            Hormone::Triiodothyronine       => 650.977,
            Hormone::Cholecalciferol        => 384.64,
            Hormone::Cobalamin              => 1355.388, // chonker
        }
    }
}

impl From<Hormone> for &str {
    fn from(i: Hormone) -> Self {
        match i {
            Hormone::Cholesterol            => "Cholesterol",
            Hormone::Testosterone           => "Testosterone",
            Hormone::Dihydrotestosterone    => "Dihydrotestosterone",
            Hormone::Dehydroepiandrosterone => "Dehydroepiandrosterone",
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
            Hormone::Cholecalciferol        => "Vitamin D3",
            Hormone::Cobalamin              => "Vitamin B12",
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

impl From<Unit> for &str {
    fn from(i: Unit) -> Self {
        match i {
            Unit::Gram  => "g",
            Unit::Mole  => "mol",
            Unit::Litre => "ℓ",
        }
    }
}



fn compute_result(expression: &Expression) -> f64 {
    let prefix_calculation = (i32::from(expression.unit_in.numerator.prefix) + i32::from(expression.unit_out.denominator.prefix)) - (i32::from(expression.unit_in.denominator.prefix) + i32::from(expression.unit_out.numerator.prefix)); // use fixed point math to not loose precision
    match (expression.unit_in.numerator.unit, expression.unit_in.denominator.unit, expression.unit_out.numerator.unit, expression.unit_out.denominator.unit) { // very ugly math engine, does not allow all calculations, probably just proof of concept
        (Unit::Mole, Unit::Litre, Unit::Gram, Unit::Litre) => f64::powf(10.0, prefix_calculation.into()) * (f64::from(expression.in_val) * f64::from(expression.hormone)),
        (Unit::Gram, Unit::Litre, Unit::Mole, Unit::Litre) => f64::powf(10.0, prefix_calculation.into())  * (f64::from(expression.in_val) / f64::from(expression.hormone)),
        (Unit::Gram, Unit::Litre, Unit::Gram, Unit::Litre) | (Unit::Mole, Unit::Litre, Unit::Mole, Unit::Litre) => f64::powf(10.0, prefix_calculation.into())  * f64::from(expression.in_val),
        (_,_,_,_) => -1.0,
    }
}

/// Computes result of conversion expression
pub fn compute(input: &str) -> Result<String, lalrpop_util::ParseError<usize, grammar::Token, &str>> {
    let expression = grammar::ExpressionParser::new().parse(input)?;
    Ok(format!("{:.03} {}/{} {}", compute_result(&expression), expression.unit_out.numerator, expression.unit_out.denominator, expression.hormone))
}
