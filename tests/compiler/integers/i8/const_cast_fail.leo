/*
namespace: Compile
expectation: Fail
inputs:
 - i8.in: |
    [main]
    y: bool = false;
    [registers]
    r0: bool = true;
*/

function main(y: bool) -> bool {
    let x: i8 = 233u32 as i8;

    return y;
}