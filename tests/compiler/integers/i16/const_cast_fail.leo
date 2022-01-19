/*
namespace: Compile
expectation: Fail
*/

function main() -> bool {
    let x: i16 = 4294967295u128 as i16;

    return true;
}
