/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: u16 = 4294967295u128 as u16;

    return true;
}
