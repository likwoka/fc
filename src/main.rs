// c = (f - 32) / 1.8

/// # Usage:
/// fc 32C   => 89.6F
/// fc 32c   => 89.6F
/// fc 81f   => 27.2C
/// fc 81F   => 21.2C
/// fc 32    => 32C => 89.6F
///             32F => 0C

// Take command line input
// Print output to command line
// Unit tests
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let temp = &args[1];

    println!("{:?}", args);
//    println!("Hello, world!");
}


//fn parse_temp_and_unit(arg: String) -> (&str, &str) {
//}

enum Temp {
    F(f64),
    C(f64),
    Unknown(f64),
}


fn to_c(input: Temp) -> Temp {
    let out = (input - 32.0) / 1.8;
    Temp::C(out)
}

// Q1: can I do specific method for different value of enum?
// Temp::F.to_c(...) -> Temp
// Temp::C.to_f(...) -> Temp
// Temp::Unknown.f_to_c(...) -> Temp
// Temp::Unknown.c_to_f(...) -> Temp
// Temp::Unknown.to(...) -> Vec<Temp>

#[test]
fn smoke3() {
    let input = Temp::C(32.0);
    match input {
        Temp::F => to_c(input),
        Temp::C => to_f(input),
        Temp::Unknown => {
            to_c(input), to_f(input)
        }
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

#[test]
fn smoke() {
    assert_eq!(c_to_f(32.0), 89.6);
}

#[test]
fn smoke2() {
    assert_eq!(f_to_c(81.0), 27.2);
}
