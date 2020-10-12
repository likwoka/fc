//! # fc
//!
//! A command line program that converts temperature between Fahrenheit and Celsius.
use std::env;

use fc;

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

#[derive(Debug)]
enum CmdLineMode {
    ConvertTemperature(fc::T),
    PrintFormula,
    PrintHelp,
}

fn parse_cmdline(args: &Vec<String>) -> Result<CmdLineMode, fc::FcError> {
    match args.len() {
        2 if args[1] == "-h" => Ok(CmdLineMode::PrintHelp),
        2 if args[1] == "-f" => Ok(CmdLineMode::PrintFormula),
        2 => Ok(CmdLineMode::ConvertTemperature(fc::parse_str_to_t(
            &args[1],
        )?)),
        _ => Err(fc::FcError::WrongSyntax),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_cmdline(&args) {
        Ok(CmdLineMode::ConvertTemperature(i)) => {
            let output = fc::convert(i);
            for o in output {
                let i_unit = match o.unit {
                    fc::TUnit::C => fc::TUnit::F,
                    _ => fc::TUnit::C,
                };
                println!("{}{:?} => {:.1}{:?}", i.value, i_unit, o.value, o.unit)
            }
        }
        Ok(CmdLineMode::PrintHelp) => println!("{}", HELP_MSG),
        Ok(CmdLineMode::PrintFormula) => {
            println!("F to C: (f - 32.0) / 1.8");
            println!("C to F: c * 1.8 + 32.0");
        }
        Err(e) => println!("{}", e.to_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cmdline_ok_help_mesg() -> Result<(), String> {
        match parse_cmdline(&vec!["fc".to_string(), "-h".into()]) {
            Ok(CmdLineMode::PrintHelp) => Ok(()),
            actual => Err(String::from(format!(
                "Expect {:?}, got {:?} instead",
                CmdLineMode::PrintHelp,
                actual
            ))),
        }
    }

    #[test]
    fn parse_cmdline_ok_print_formula() -> Result<(), String> {
        match parse_cmdline(&vec!["fc".to_string(), "-f".into()]) {
            Ok(CmdLineMode::PrintFormula) => Ok(()),
            actual => Err(String::from(format!(
                "Expect {:?}, got {:?} instead",
                CmdLineMode::PrintFormula,
                actual
            ))),
        }
    }

    #[test]
    fn parse_cmdline_err_too_little_params() -> Result<(), String> {
        match parse_cmdline(&vec!["fc".to_string()]) {
            Err(fc::FcError::WrongSyntax) => Ok(()),
            actual => Err(String::from(format!(
                "Expect {:?}, got {:?} instead",
                fc::FcError::WrongSyntax,
                actual
            ))),
        }
    }

    #[test]
    fn parse_cmdline_err_too_many_params() -> Result<(), String> {
        match parse_cmdline(&vec!["fc".to_string(), "blah".into(), "blob".into()]) {
            Err(fc::FcError::WrongSyntax) => Ok(()),
            actual => Err(String::from(format!(
                "Expect {:?}, got {:?} instead",
                fc::FcError::WrongSyntax,
                actual
            ))),
        }
    }
}
