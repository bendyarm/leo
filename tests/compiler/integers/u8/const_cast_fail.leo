/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: u8 = 4294967295u128 as u8;

    return true;
}
