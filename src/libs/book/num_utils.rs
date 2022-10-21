use std::mem;

pub fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let seive: u64 = bits & 0xfffffffffffff;
    let mantissa = match exponent {
       0 => seive << 1,
       _ => seive | 0x10000000000000
    };
    (mantissa, exponent - (1023 + 52), sign)
}
