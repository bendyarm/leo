/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 1;
    b: i64 = 2;
    c: i64 = 3;
    d: i64 = 4;
    e: i64 = 5;
    f: i64 = 6;
    g: i64 = 7;
    h: i64 = 8;
    i: i64 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64, i: i64) -> bool {
    return (a as u8) == 1 && b as u16 == 2 &&
		(c as u32) == 3 && (d as u64) == 4 &&
		(e as u128) == 5 && (f as i8) == 6 &&
		(g as i16) == 7 && (h as i32) == 8 &&
		(i as i128) == 9;
}
