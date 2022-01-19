/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: u32 = 4294967295u128 as u32;

    return true;
}
