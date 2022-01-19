/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 1;
    b: i32 = 2;
    c: i32 = 3;
    d: i32 = 4;
    e: i32 = 5;
    f: i32 = 6;
    g: i32 = 7;
    h: i32 = 8;
    i: i32 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32) -> bool {
    return (a as u8) == 1 && b as u16 == 2 &&
		(c as u32) == 3 && (d as u64) == 4 &&
		(e as u128) == 5 && (f as i8) == 6 &&
		(g as i16) == 7 && (h as i64) == 8 &&
		(i as i128) == 9;
}
