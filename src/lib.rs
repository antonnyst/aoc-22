mod days;
pub use days::*;

pub const MAX_DAY: u32 = 3;

pub fn calculate(day: u32) -> (String, String) {
    match day {
        0 => (String::from("Invalid day"), String::new()),
        1 => d1::d1(),
        2 => d2::d2(),
        3 => d3::d3(),
        4 => d4::d4(),
        5 => d5::d5(),
        6 => d6::d6(),
        7 => d7::d7(),
        8 => d8::d8(),
        9 => d9::d9(),
        10 => d10::d10(),
        11 => d11::d11(),
        12 => d12::d12(),
        13 => d13::d13(),
        14 => d14::d14(),
        15 => d15::d15(),
        16 => d16::d16(),
        17 => d17::d17(),
        18 => d18::d18(),
        19 => d19::d19(),
        20 => d20::d20(),
        21 => d21::d21(),
        22 => d22::d22(),
        23 => d23::d23(),
        24 => d24::d24(),
        25 => d25::d25(),
        _ => unimplemented!()
    }
}
