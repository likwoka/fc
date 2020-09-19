//! fc -- Convert temperature between Fahrenheit and Celsius
use std::{env};

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
    unit: TUnit
}

/// Temperature unit.
#[derive(Debug, Copy, Clone)]
enum TUnit {
    F,
    C,
    Unknown
}

/// Convert value from f to c.
fn to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

/// Convert value from c to f.
fn to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

#[derive(Debug)]
enum CmdLineMode {
    ConvertTemperature(T),
    PrintFormula,
    PrintHelp
}

#[derive(Debug)]
enum MyError {
    WrongSyntax,
    UnitNotRecognized,
    ValueNotANumber
}

/// 
enum Output {
    Single(T),
    Double(T, T)
}

fn convert(input: T) -> Output {
    match input.unit {
        TUnit::F => {
            Output::Single(
                T { value: to_c(input.value), unit: TUnit::C }
            )
        },
        TUnit::C => {
            Output::Single(
                T { value: to_f(input.value), unit: TUnit::F }
            )
        },
        TUnit::Unknown => {
            Output::Double(
                T { value: to_f(input.value), unit: TUnit::F },
                T { value: to_c(input.value), unit: TUnit::C }
            )
        }
    }
}

fn parse_cmdline_input(args: Vec<String>) -> Result<CmdLineMode, MyError> {
    if args.len() == 2 {
        if args[1] == "-h" {
            Ok(CmdLineMode::PrintHelp)
        } else if args[1] == "-f" {
            Ok(CmdLineMode::PrintFormula)
        } else {
            match parse_string(&args[1]) {
                Ok(input) => {
                    Ok(CmdLineMode::ConvertTemperature(input))
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
    } else {
        Err(MyError::WrongSyntax)
    }
}

// TODO:ALEX
fn parse_string(arg: &str) -> Result<T, MyError> {
    // the last char has to be c, C, f, F, or nothing.
    let maybe_unit = arg.chars().last().unwrap();
    let unit= match maybe_unit {
        'c' | 'C' => {
            TUnit::C    
        },
        'f' | 'F' => {
            TUnit::F
        },
        _ => {
            if maybe_unit.is_digit(10) {
                TUnit::Unknown
            } else {
                return Err(MyError::UnitNotRecognized);
            }
        }
    };

    // for c, C, f, F, then the rest should be a float
    match unit {
        TUnit::Unknown => {
            let v = arg.parse::<f64>();
            match v  {
                Err(_) => {
                    return Err(MyError::ValueNotANumber);
                },
                Ok(value) => {
                    return Ok(T { value, unit });
                }
            };
            
        },
        _ => {
            let v = arg[0..arg.len()-1].parse::<f64>();
            match v {
                Err(_) => {
                    return Err(MyError::ValueNotANumber);
                },
                Ok(value) => {
                    return Ok(T { value, unit });
                }
            }
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = parse_cmdline_input(args);
    
    match mode {
        Ok(CmdLineMode::ConvertTemperature(i)) => {
            let output = convert(i);
            match output {
                Output::Single(o) => {
                    println!("{}{:?} => {:.1}{:?}", i.value, i.unit, o.value, o.unit);
                },
                Output::Double(j, k) => {
                    println!("{}C => {:.1}{:?}", i.value, j.value, j.unit);
                    println!("{}F => {:.1}{:?}", i.value, k.value, k.unit);
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
    use super::{T, TUnit, convert};

    #[test]
    fn fuzzy_convert_ok_from_C() {
        let t = T { value: 32.0, unit: TUnit::F };
        convert(t);
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
