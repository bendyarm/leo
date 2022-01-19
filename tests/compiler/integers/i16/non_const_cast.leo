/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 1;
    b: i16 = 2;
    c: i16 = 3;
    d: i16 = 4;
    e: i16 = 5;
    f: i16 = 6;
    g: i16 = 7;
    h: i16 = 8;
    i: i16 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16, h: i16, i: i16) -> bool {
    return (a as u8) == 1 && b as u16 == 2 &&
		(c as u32) == 3 && (d as u64) == 4 &&
		(e as u128) == 5 && (f as i8) == 6 &&
		(g as i32) == 7 && (h as i64) == 8 &&
		(i as i128) == 9;
}
