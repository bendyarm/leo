/*
namespace: Compile
expectation: Pass
inputs:
 - u16.in: |
    [main]
    a: u16 = 1;
    b: u16 = 2;
    c: u16 = 3;
    d: u16 = 4;
    e: u16 = 5;
    f: u16 = 6;
    g: u16 = 7;
    h: u16 = 8;
    i: u16 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16, i: u16) -> bool {
    return (a as i8) == 1 && b as i16 == 2 &&
		(c as i32) == 3 && (d as i64) == 4 &&
		(e as i128) == 5 && (f as u8) == 6 &&
		(g as u32) == 7 && (h as u64) == 8 &&
		(i as u128) == 9;
}
