/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

@transition
function one() -> u32 {
    return 1u32;
}

function main(y: bool) -> bool {
    return one() == 1u32;
}