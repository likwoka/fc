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

enum OutputMode {
    PrintHelp,
    PrintForula,
    TemperatureWithUnit(Temperature),
    TemperatureWithoutUnit(Temperature)
}

enum MyError {
    WrongSyntax
}

enum Temperature {
    F(f64),
    C(f64),
    Unknown(f64)
}

enum TOut {
    Single(Temperature),
    Double(Temperature, Temperature)
}

fn convert(input: Temperature) -> TOut {
    match input {
        Temperature::F(f) => {
            TOut::Single(Temperature::C((f - 32.0) / 1.8))
        },
        Temperature::C(c) => {
            TOut::Single(Temperature::F(c * 1.8 + 32.0))
        },
        Temperature::Unknown(u) => {
            TOut::Double(
                Temperature::F(u * 1.8 + 32.0),
                Temperature::C((u - 32.0) / 1.8)
            )
        }
    }
}

fn parse_output_mode(args: Vec<String>) -> Result<OutputMode, MyError> {

    if args.len() == 2 {
        
        Ok(OutputMode::TemperatureWithUnit(Temperature::C(args[1])))
    } else if args.len() == 2 && args[1] == "-h" {
        Ok(OutputMode::PrintHelp)
    } else if args.len() == 2 && args[1] == "-f" {
        Ok(OutputMode::PrintForula)
    } else {
        Err(MyError::WrongSyntax)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let temp = parse_output_mode(args);

    println!("{:?}", args);
}


#[cfg(test)]
mod tests {
    use super::{Temperature, convert};

    #[test]
    fn smoke3() {
        let input = Temperature::C(32.0);
        convert(input);
    }
    
    // #[test]
    // fn smoke1() {
    //     assert_eq!(convert(Temperature::C(32.0)), 89.6);
    // }
    
    // #[test]
    // fn smoke2() {
    //     assert_eq!(convert(Temperature::F(81.0)), 27.2);
    // }
}
