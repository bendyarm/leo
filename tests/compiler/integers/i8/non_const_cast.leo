/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 1;
    b: i8 = 2;
    c: i8 = 3;
    d: i8 = 4;
    e: i8 = 5;
    f: i8 = 6;
    g: i8 = 7;
    h: i8 = 8;
    i: i8 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8) -> bool {
    return (a as u8) == 1 && b as u16 == 2 &&
		(c as u32) == 3 && (d as u64) == 4 &&
		(e as u128) == 5 && (f as i16) == 6 &&
		(g as i32) == 7 && (h as i64) == 8 &&
		(i as i128) == 9;
}
