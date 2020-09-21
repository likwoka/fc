//! fc -- Convert temperature between Fahrenheit and Celsius
use std::env;

const HELP_MSG: &str = r"fc -- convert temperature between Fahrenheit and Celsius
Usage:
fc 32C   # output: 32C => 89.6F
fc 32c   # output: 32C => 89.6F
fc 81f   # output: 81F => 27.2C
fc 81F   # output: 81F => 21.2C
fc 32    # output: 32C => 89.6F; 32F => 0C
fc 81    # output: 81F => 27.2C; 81C => 177.8F
fc -f    # print formula
fc -h    # print this help message";

/// Temperature.
#[derive(Debug, Copy, Clone)]
struct T {
    value: f64,
    unit: TUnit,
}

impl T {
    /// Convert value from f to c.
    fn to_c(&self) -> T {
        match self.unit {
            TUnit::C => self.clone(),
            _ => T {
                value: (self.value - 32.0) / 1.8,
                unit: TUnit::C,
            },
        }
    }

    /// Convert value from c to f.
    fn to_f(&self) -> T {
        match self.unit {
            TUnit::F => self.clone(),
            _ => T {
                value: self.value * 1.8 + 32.0,
                unit: TUnit::F,
            },
        }
    }
}

/// Temperature unit.
#[derive(Debug, Copy, Clone)]
enum TUnit {
    F,
    C,
    Unknown,
}

#[derive(Debug)]
enum CmdLineMode {
    ConvertTemperature(T),
    PrintFormula,
    PrintHelp,
}

#[derive(Debug)]
enum MyError {
    WrongSyntax,
    UnitNotRecognized,
    ValueNotANumber,
}

/// Output of temperature conversion function.
/// Can output 2 temperatures if it is a fuzzy input.
enum Output {
    Single(T),
    Double(T, T),
}

fn convert(input: T) -> Output {
    match input.unit {
        TUnit::F => Output::Single(input.to_c()),
        TUnit::C => Output::Single(input.to_f()),
        TUnit::Unknown => Output::Double(input.to_f(), input.to_c()),
    }
}

fn parse_cmdline(args: &Vec<String>) -> Result<CmdLineMode, MyError> {
    match args.len() {
        2 if args[1] == "-h" => Ok(CmdLineMode::PrintHelp),
        2 if args[1] == "-f" => Ok(CmdLineMode::PrintFormula),
        2 => Ok(CmdLineMode::ConvertTemperature(parse_string(&args[1])?)),
        _ => Err(MyError::WrongSyntax),
    }
}

fn parse_string(arg: &str) -> Result<T, MyError> {
    // the last char has to be c, C, f, F, or nothing.
    let unit = match arg.chars().last().unwrap() {
        'c' | 'C' => TUnit::C,
        'f' | 'F' => TUnit::F,
        u if u.is_digit(10) => TUnit::Unknown,
        _ => return Err(MyError::UnitNotRecognized),
    };

    match unit {
        TUnit::Unknown => match arg.parse::<f64>() {
            Ok(value) => return Ok(T { value, unit }),
            Err(_) => return Err(MyError::ValueNotANumber),
        },
        _ => match arg[0..arg.len() - 1].parse::<f64>() {
            Ok(value) => return Ok(T { value, unit }),
            Err(_) => return Err(MyError::ValueNotANumber),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_cmdline(&args) {
        Ok(CmdLineMode::ConvertTemperature(i)) => match convert(i) {
            Output::Single(o) => {
                println!("{}{:?} => {:.1}{:?}", i.value, i.unit, o.value, o.unit);
            }
            Output::Double(j, k) => {
                println!("{}C => {:.1}{:?}", i.value, j.value, j.unit);
                println!("{}F => {:.1}{:?}", i.value, k.value, k.unit);
            }
        },
        Ok(CmdLineMode::PrintHelp) => println!("{}", HELP_MSG),
        Ok(CmdLineMode::PrintFormula) => {
            println!("F to C: (f - 32.0) / 1.8");
            println!("C to F: c * 1.8 + 32.0");
        }
        Err(MyError::WrongSyntax) => println!("Wrong syntax!"),
        Err(MyError::ValueNotANumber) => println!("Value not a number!"),
        Err(MyError::UnitNotRecognized) => println!("Unit not recognized!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_ok_from_c() {}

    #[test]
    fn convert_ok_from_f() {}

    #[test]
    fn convert_ok_from_unknown() {}

    #[test]
    fn parse_cmdline_ok_help_mesg() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_cmdline_ok_print_formula() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_cmdline_err_too_little_params() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_cmdline_err_too_many_params() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_ok_lowercase_f() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_f() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_ok_lowercase_c() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_ok_uppercase_c() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_ok_no_unit() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_err_invalid() -> Result<(), MyError> {
        Ok(())
    }

    #[test]
    fn parse_string_err_invalid2() -> Result<(), MyError> {
        Ok(())
    }
}
