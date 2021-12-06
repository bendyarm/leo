/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

import std.transaction_model.record.Record;

@transition
function one(rec1: Record, rec2: Record) -> (Record, Record) {
    return (rec1, rec2);
}

// Dummy main function
function main() {}
