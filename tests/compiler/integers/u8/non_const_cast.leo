/*
namespace: Compile
expectation: Pass
inputs:
 - u8.in: |
    [main]
    a: u8 = 1;
    b: u8 = 2;
    c: u8 = 3;
    d: u8 = 4;
    e: u8 = 5;
    f: u8 = 6;
    g: u8 = 7;
    h: u8 = 8;
    i: u8 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8, i: u8) -> bool {
    return (a as i8) == 1 && b as i16 == 2 &&
		(c as i32) == 3 && (d as i64) == 4 &&
		(e as i128) == 5 && (f as u16) == 6 &&
		(g as u32) == 7 && (h as u64) == 8 &&
		(i as u128) == 9;
}
