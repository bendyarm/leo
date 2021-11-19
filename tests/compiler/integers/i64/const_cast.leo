/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    y: bool = false;
    [registers]
    r0: bool = true;
*/

function main(y: bool) -> bool {
    let x: i64 = 127u32 as i64;

    return y;
}
