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
    let x: i32 = 43958u64 as i32;

    return y;
}
