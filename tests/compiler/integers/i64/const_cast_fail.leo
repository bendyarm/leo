/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: i64 = 4294967295u128 as i64;

    return true;
}
