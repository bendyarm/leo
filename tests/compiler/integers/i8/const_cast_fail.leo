/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: i8 = 4294967295u128 as i8;

    return true;
}
