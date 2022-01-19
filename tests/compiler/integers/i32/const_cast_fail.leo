/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: i32 = 4294967295u128 as i32;

    return true;
}
