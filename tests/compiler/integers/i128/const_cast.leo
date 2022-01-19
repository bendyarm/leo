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
    let x: i128 = 4294967295u128 as i128;

    return y;
}
