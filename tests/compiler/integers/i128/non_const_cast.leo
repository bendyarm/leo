/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 1;
    b: i128 = 2;
    c: i128 = 3;
    d: i128 = 4;
    e: i128 = 5;
    f: i128 = 6;
    g: i128 = 7;
    h: i128 = 8;
    i: i128 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: i128, b: i128, c: i128, d: i128, e: i128, f: i128, g: i128, h: i128, i: i128) -> bool {
    return (a as u8) == 1 && b as u16 == 2 &&
		(c as u32) == 3 && (d as u64) == 4 &&
		(e as u128) == 5 && (f as i8) == 6 &&
		(g as i16) == 7 && (h as i32) == 8 &&
		(i as i64) == 9;
}
