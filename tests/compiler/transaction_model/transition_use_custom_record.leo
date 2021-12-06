/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

circuit Record {
    value: u64;
}

@transition
function one(rec1: Record, rec2: Record) -> (Record, Record) {
    return (rec1, rec2);
}

// Dummy main function
function main() {}
