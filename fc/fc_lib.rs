//! # fc
//!
//! A library of functions to converts temperature between Fahrenheit and Celsius.

/// Possible errors from the conversion.
#[derive(Debug)]
pub enum FcError {
    UnitNotRecognized,
    ValueNotANumber,
    WrongSyntax,
}

impl FcError {
    pub fn to_str(&self) -> &str {
        match &self {
            FcError::UnitNotRecognized => "Unit not recognized!",
            FcError::ValueNotANumber => "Value not a number!",
            FcError::WrongSyntax => "Wrong syntax!",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum T {
    F(f32),
    C(f32),
    Unknown(f32),
}

impl T {
    pub fn to_c(&self) -> T {
        match self {
            T::C(_) => self.clone(),
            T::F(v) | T::Unknown(v) => T::C(round_to_one_dec((v - 32.0) / 1.8)),
        }
    }

    /// Convert value from c to f.
    pub fn to_f(&self) -> T {
        match self {
            T::F(_) => self.clone(),
            T::C(v) | T::Unknown(v) => T::F(round_to_one_dec(v * 1.8 + 32.0)),
        }
    }

    pub fn unit(&self) -> &str {
        match self {
            T::C(_) => "C",
            T::F(_) => "F",
            T::Unknown(_) => "",
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            T::C(v) | T::F(v) | T::Unknown(v) => *v,
        }
    }
}

fn round_to_one_dec(f: f32) -> f32 {
    (f * 10.0).round() / 10.0
}

/// Parse a string input (string slice) to a temperature (T).
///
/// # Examples
/// ```
/// fc_lib::parse_str_to_t("23C");  // returns T::C(23.0)
/// fc_lib::parse_str_to_t("23c");  // returns T::C(23.0)
/// fc_lib::parse_str_to_t("74F");  // returns T::F(74.0)
/// fc_lib::parse_str_to_t("74f");  // returns T::F(74.0)
/// fc_lib::parse_str_to_t("74");   // returns T::Unknown(74.0)
/// fc_lib::parse_str_to_t("abc");  // FcError::ValueNotANumber
/// ```
pub fn parse_str_to_t(arg: &str) -> Result<T, FcError> {
    // the last char has to be c, C, f, F, or nothing.
    match arg.chars().last().unwrap() {
        'c' | 'C' => match arg[0..arg.len() - 1].parse::<f32>() {
            Ok(value) => return Ok(T::C(value)),
            Err(_) => return Err(FcError::ValueNotANumber),
        },
        'f' | 'F' => match arg[0..arg.len() - 1].parse::<f32>() {
            Ok(value) => return Ok(T::F(value)),
            Err(_) => return Err(FcError::ValueNotANumber),
        },
        u if u.is_digit(10) => match arg.parse::<f32>() {
            Ok(value) => return Ok(T::Unknown(value)),
            Err(_) => return Err(FcError::ValueNotANumber),
        },
        _ => Err(FcError::UnitNotRecognized),
    }
}

/// Convert an input temperature to one or more output temperatures.
///
/// # Examples
/// ```
/// // returns 1-element vector of T::F(73.4)
/// fc_lib::convert(fc_lib::T::C(23.0));
/// // returns 1-element vector of T::C23.0)   
/// fc_lib::convert(fc_lib::T::F(73.4));
/// // returns 2-element vector of T::F(165.2), T::C(23.3)
/// fc_lib::convert(fc_lib::T::Unknown(74.0));
/// ```
pub fn convert(input: T) -> Vec<T> {
    match input {
        T::F(_) => vec![input.to_c()],
        T::C(_) => vec![input.to_f()],
        T::Unknown(_) => vec![input.to_f(), input.to_c()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_ok_lowercase_f() -> Result<(), FcError> {
        parse_str_to_t("65f")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_f() -> Result<(), FcError> {
        parse_str_to_t("65F")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_lowercase_c() -> Result<(), FcError> {
        parse_str_to_t("23c")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_c() -> Result<(), FcError> {
        parse_str_to_t("23C")?;
        Ok(())
    }

    #[test]
    fn parse_string_ok_no_unit() -> Result<(), FcError> {
        parse_str_to_t("30.4")?;
        Ok(())
    }

    #[test]
    fn parse_string_err_invalid() -> Result<(), String> {
        //assert_eq!(Err(MyError::UnitNotRecognized), parse_string("30INVALID"));
        match parse_str_to_t("Not even a string") {
            Err(FcError::UnitNotRecognized) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                FcError::UnitNotRecognized
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid2() -> Result<(), String> {
        match parse_str_to_t("38E") {
            Err(FcError::UnitNotRecognized) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                FcError::UnitNotRecognized
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid3() -> Result<(), String> {
        match parse_str_to_t("InvalidC") {
            Err(FcError::ValueNotANumber) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                FcError::ValueNotANumber
            ))),
        }
    }

    #[test]
    fn parse_string_err_invalid4() -> Result<(), String> {
        match parse_str_to_t("Invalidf") {
            Err(FcError::ValueNotANumber) => Ok(()),
            _ => Err(String::from(format!(
                "Expect {:?}, but got Ok instead",
                FcError::ValueNotANumber
            ))),
        }
    }

    #[test]
    fn convert_ok_from_c() {
        let actual = convert(T::C(23.0));
        assert_eq!(1, actual.len());
        assert_eq!(T::F(73.4), actual[0]);
    }

    #[test]
    fn convert_ok_from_f() {
        let actual = convert(T::F(73.4));
        assert_eq!(1, actual.len());
        assert_eq!(T::C(23.0), actual[0]);
    }

    #[test]
    fn convert_ok_from_unknown() {
        let actual = convert(T::Unknown(73.4));
        assert_eq!(2, actual.len());
        assert_eq!(T::F(164.1), actual[0]);
        assert_eq!(T::C(23.0), actual[1]);
    }
}
