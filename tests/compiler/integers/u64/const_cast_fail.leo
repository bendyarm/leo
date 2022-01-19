/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: u64 = 4294967295u128 as u64;

    return true;
}
