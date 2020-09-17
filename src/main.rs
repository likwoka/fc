//! fc -- Convert temperature between Fahrenheit and Celsius
use std::{env, io};

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
#[derive(Debug, PartialEq)]
struct T {
    value: f64,
    unit: TUnit
}

/// Temperature unit.
#[derive(Debug, PartialEq)]
enum TUnit {
    F,
    C
}

/// Convert value from f to c.
fn to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

/// Convert value from c to f.
fn to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

#[derive(Debug, PartialEq)]
enum CmdLineMode {
    ConvertTemperature(Input),
    PrintFormula,
    PrintHelp
}

#[derive(Debug, PartialEq)]
enum MyError {
    WrongSyntax,
    UnitNotRecognized,
    ValueNotANumber
}

#[derive(Debug, PartialEq)]
enum Input {
    Valid(T),
    Fuzzy(f64)
}

/// 
enum Output {
    Single(T),
    Double(T, T)
}

fn convert(input: Input) -> Output {
    match input {
        Input::Valid(T { value: f, unit: TUnit::F }) => {
            Output::Single(
                T { value: to_c(f), unit: TUnit::C }
            )
        },
        Input::Valid(T { value: c, unit: TUnit::C }) => {
            Output::Single(
                T { value: to_f(c), unit: TUnit::F }
            )
        },
        Input::Fuzzy(u) => {
            Output::Double(
                T { value: to_f(u), unit: TUnit::F },
                T { value: to_c(u), unit: TUnit::C }
            )
        }
    }
}

fn parse_cmdline_input(args: Vec<String>) -> Result<CmdLineMode, MyError> {
    if args.len() == 2 {
        match parse_string(args[1]) {
            Ok(input) => {
                Ok(CmdLineMode::ConvertTemperature(input))
            },
            Err(e) => {
                Err(e)
            }
        }
    } else if args.len() == 2 && args[1] == "-h" {
        Ok(CmdLineMode::PrintHelp)
    } else if args.len() == 2 && args[1] == "-f" {
        Ok(CmdLineMode::PrintFormula)
    } else {
        Err(MyError::WrongSyntax)
    }
}

// TODO:ALEX
fn parse_string(arg: String) -> Result<Input, MyError> {
    let maybe_unit = value_n_unit.chars().last().unwrap();
    // if maybe_unit == "f" {

    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = parse_cmdline_input(args);
    
    match mode {
        Ok(CmdLineMode::ConvertTemperature(input)) => {
            let output = convert(input);
            match output {
                Output::Single(o) => {
                    println!("xx => yy");           // TODO:ALEX
                },
                Output::Double(j, k) => {
                    println!("xx => yy");           // TODO:ALEX
                    println!("xx => yy");
                }
            }
        },
        Ok(CmdLineMode::PrintHelp) => {
            println!("{}", HELP_MSG);
        },
        Ok(CmdLineMode::PrintFormula) => {
            println!("F to C: (f - 32.0) / 1.8");
            println!("C to F: c * 1.8 + 32.0");
        },
        Err(MyError::WrongSyntax) => {
            println!("Wrong syntax!");
        },
        Err(MyError::ValueNotANumber) => {
            println!("Value not a number!")
        },
        Err(MyError::UnitNotRecognized) => {
            println!("Unit not recognized!");
        }        
    }
}


#[cfg(test)]
mod tests {
    use super::{Input, T, TUnit, convert};

    #[test]
    fn fuzzy_convert_ok_from_C() {
        let t = T {value: 32.0, unit: TUnit::F };
        let input = Input::Valid(t);
        convert(input);
    }

    #[test]
    fn fuzzy_convert_ok_from_c() {
    }

    #[test]
    fn fuzzy_convert_ok_from_F() {
    }

    #[test]
    fn fuzzy_convert_ok_from_f() {
    }

    #[test]
    fn fuzzy_convert_ok_from_unknown() {
    }

    #[test]
    fn fuzzy_convert_fail_from_invalid() {
    }
}
