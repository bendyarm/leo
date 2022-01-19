/*
namespace: Compile
expectation: Fail
inputs:
 - non_const_cast_fail.in: |
    [main]
    x: i8 = -10;

    [registers]
    r0: bool = true;
*/

function main(x: i8) -> bool {
    let y = x as u128;

    return false;
}
