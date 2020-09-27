//! # fc
//!
//! A library of functions to converts temperature between Fahrenheit and Celsius.

/// Possible errors from the conversion.
#[derive(Debug)]
pub enum MyError {
    UnitNotRecognized,
    ValueNotANumber,
    WrongSyntax,
}

/// Temperature unit.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TUnit {
    F,
    C,
    Unknown,
}

/// Temperature.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct T {
    pub value: f32,
    pub unit: TUnit,
}

fn round_to_one_dec(f: f32) -> f32 {
    (f * 10.0).round() / 10.0
}

impl T {
    /// Convert value from f to c.
    pub fn to_c(&self) -> T {
        match self.unit {
            TUnit::C => self.clone(),
            _ => T {
                value: round_to_one_dec((self.value - 32.0) / 1.8),
                unit: TUnit::C,
            },
        }
    }

    /// Convert value from c to f.
    pub fn to_f(&self) -> T {
        match self.unit {
            TUnit::F => self.clone(),
            _ => T {
                value: round_to_one_dec(self.value * 1.8 + 32.0),
                unit: TUnit::F,
            },
        }
    }
}

/// Parse a string input (string slice) to a temperature (T).
pub fn parse_str_to_t(arg: &str) -> Result<T, MyError> {
    // the last char has to be c, C, f, F, or nothing.
    let unit = match arg.chars().last().unwrap() {
        'c' | 'C' => TUnit::C,
        'f' | 'F' => TUnit::F,
        u if u.is_digit(10) => TUnit::Unknown,
        _ => return Err(MyError::UnitNotRecognized),
    };

    match unit {
        TUnit::Unknown => match arg.parse::<f32>() {
            Ok(value) => return Ok(T { value, unit }),
            Err(_) => return Err(MyError::ValueNotANumber),
        },
        _ => match arg[0..arg.len() - 1].parse::<f32>() {
            Ok(value) => return Ok(T { value, unit }),
            Err(_) => return Err(MyError::ValueNotANumber),
        },
    }
}

/// Convert an input temperature to one or more output temperatures.
pub fn convert(input: T) -> Vec<T> {
    match input.unit {
        TUnit::F => vec![input.to_c()],
        TUnit::C => vec![input.to_f()],
        TUnit::Unknown => vec![input.to_f(), input.to_c()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_ok_lowercase_f() -> Result<(), MyError> {
        parse_str_to_t("65f")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_f() -> Result<(), MyError> {
        parse_str_to_t("65F")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_lowercase_c() -> Result<(), MyError> {
        parse_str_to_t("23c")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_c() -> Result<(), MyError> {
        parse_str_to_t("23C")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_no_unit() -> Result<(), MyError> {
        parse_str_to_t("30.4")?;
        Ok(())
    }

    #[test]
    fn parse_string_err_invalid() -> Result<(), String> {
        //assert_eq!(Err(MyError::UnitNotRecognized), parse_string("30INVALID"));
        match parse_str_to_t("Not even a string") {
            Err(MyError::UnitNotRecognized) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                MyError::UnitNotRecognized
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid2() -> Result<(), String> {
        match parse_str_to_t("38E") {
            Err(MyError::UnitNotRecognized) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                MyError::UnitNotRecognized
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid3() -> Result<(), String> {
        match parse_str_to_t("InvalidC") {
            Err(MyError::ValueNotANumber) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                MyError::ValueNotANumber
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid4() -> Result<(), String> {
        match parse_str_to_t("Invalidf") {
            Err(MyError::ValueNotANumber) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                MyError::ValueNotANumber
            ))),
        }
    }

    #[test]
    fn convert_ok_from_c() {
        let actual = convert(T {
            value: 23.0,
            unit: TUnit::C,
        });
        assert_eq!(1, actual.len());
        assert_eq!(
            T {
                value: 73.4,
                unit: TUnit::F
            },
            actual[0]
        );
    }

    #[test]
    fn convert_ok_from_f() {
        let actual = convert(T {
            value: 73.4,
            unit: TUnit::F,
        });
        assert_eq!(1, actual.len());
        assert!((23.0 - actual[0].value).abs() <= f32::EPSILON);
    }

    #[test]
    fn convert_ok_from_unknown() {
        let actual = convert(T {
            value: 73.4,
            unit: TUnit::Unknown,
        });
        assert_eq!(2, actual.len());
        assert_eq!(
            T {
                value: 164.1,
                unit: TUnit::F
            },
            actual[0]
        );
        assert_eq!(
            T {
                value: 23.0,
                unit: TUnit::C
            },
            actual[1]
        );
    }
}
