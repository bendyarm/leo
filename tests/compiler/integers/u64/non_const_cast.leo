/*
namespace: Compile
expectation: Pass
inputs:
 - u64.in: |
    [main]
    a: u64 = 1;
    b: u64 = 2;
    c: u64 = 3;
    d: u64 = 4;
    e: u64 = 5;
    f: u64 = 6;
    g: u64 = 7;
    h: u64 = 8;
    i: u64 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64) -> bool {
    return (a as i8) == 1 && b as i16 == 2 &&
		(c as i32) == 3 && (d as i64) == 4 &&
		(e as i128) == 5 && (f as u8) == 6 &&
		(g as u16) == 7 && (h as u32) == 8 &&
		(i as u128) == 9;
}
