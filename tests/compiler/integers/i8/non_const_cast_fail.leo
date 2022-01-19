/*
namespace: Compile
expectation: Fail
inputs:
 - non_const_cast_fail.in: |
    [main]
    x: u128 = 340282366920938463463374607431768211455;

    [registers]
    r0: bool = true;
*/

function main(x: u128) -> bool {
    let y: i8 = x as i8;

    return false;
}
