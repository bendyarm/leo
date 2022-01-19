/*
namespace: Compile
expectation: Pass
inputs:
 - u32.in: |
    [main]
    a: u32 = 1;
    b: u32 = 2;
    c: u32 = 3;
    d: u32 = 4;
    e: u32 = 5;
    f: u32 = 6;
    g: u32 = 7;
    h: u32 = 8;
    i: u32 = 9;

    [registers]
    r0: bool = true;
*/

function main(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32) -> bool {
    return (a as i8) == 1 && b as i16 == 2 &&
		(c as i32) == 3 && (d as i64) == 4 &&
		(e as i128) == 5 && (f as u8) == 6 &&
		(g as u16) == 7 && (h as u64) == 8 &&
		(i as u128) == 9;
}
