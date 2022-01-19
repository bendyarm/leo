/*
namespace: Compile
expectation: Pass
inputs:
 - u128.in: |
    [main]
    a: u128 = 1;
    b: u128 = 2;
    c: u128 = 3;
    d: u128 = 4;
    e: u128 = 5;
    f: u128 = 6;
    g: u128 = 7;
    h: u128 = 8;
    i: u128 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: u128, b: u128, c: u128, d: u128, e: u128, f: u128, g: u128, h: u128, i: u128) -> bool {
    return (a as i8) == 1 && b as i16 == 2 &&
		(c as i32) == 3 && (d as i64) == 4 &&
		(e as i128) == 5 && (f as u8) == 6 &&
		(g as u16) == 7 && (h as u32) == 8 &&
		(i as u64) == 9;
}
