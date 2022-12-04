mod days;
pub use days::*;

pub const MAX_DAY: u32 = 4;

pub fn calculate(day: u32) -> Result<(String, String), String> {
    match day {
        1 => Ok(d1::d1()),
        2 => Ok(d2::d2()),
        3 => Ok(d3::d3()),
        4 => Ok(d4::d4()),
        5 => Ok(d5::d5()),
        6 => Ok(d6::d6()),
        7 => Ok(d7::d7()),
        8 => Ok(d8::d8()),
        9 => Ok(d9::d9()),
        10 => Ok(d10::d10()),
        11 => Ok(d11::d11()),
        12 => Ok(d12::d12()),
        13 => Ok(d13::d13()),
        14 => Ok(d14::d14()),
        15 => Ok(d15::d15()),
        16 => Ok(d16::d16()),
        17 => Ok(d17::d17()),
        18 => Ok(d18::d18()),
        19 => Ok(d19::d19()),
        20 => Ok(d20::d20()),
        21 => Ok(d21::d21()),
        22 => Ok(d22::d22()),
        23 => Ok(d23::d23()),
        24 => Ok(d24::d24()),
        25 => Ok(d25::d25()),
        _ => Err(String::from("Invalid day"))
    }
}
