/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

import std.transaction_model.record.Record;

@transition
function noop_0() {}

@transition
function noop_1() -> () {}

@transition
function insert() -> Record {
    return Record {
        owner: aleo1fuge6ah8c9custvmlju5t30gk8p8lar5x36jlfa2glhgy9n0fuxsreeh2c,
        value: 10,
        payload: [0u8; 128],
        program_id: [0u8; 48],
    };
}

@transition
function update(rec: Record) -> Record {
    return rec;
}

@transition
function remove_0(rec: Record) {}

@transition
function remove_1(rec: Record) -> () {}

@transition
function double_insert() -> (Record, Record) {
    let rec1: Record = Record {
        owner: aleo1fuge6ah8c9custvmlju5t30gk8p8lar5x36jlfa2glhgy9n0fuxsreeh2c,
        value: 10,
        payload: [0u8; 128],
        program_id: [0u8; 48],
    };

    let rec2: Record = Record {
            owner: aleo1fuge6ah8c9custvmlju5t30gk8p8lar5x36jlfa2glhgy9n0fuxsreeh2c,
            value: 11,
            payload: [0u8; 128],
            program_id: [0u8; 48],
    };

    return (rec1, rec2);
}

@transition
function double_remove_0(rec1: Record, rec2: Record) {}

@transition
function double_remove_1(rec1: Record, rec2: Record) -> () {}

@transition
function double_remove_2(recs: (Record, Record)) {}

@transition
function double_remove_3(recs: (Record, Record)) -> () {}

@transition
function join_0(rec1: Record, rec2: Record) -> Record {
    return rec1;
}

@transition
function join_1(recs: (Record, Record)) -> Record {
    return recs.0;
}

@transition
function split(rec: Record) -> (Record, Record) {
    return (rec, rec);
}

@transition
function full_0(rec1: Record, rec2: Record) -> (Record, Record) {
    return (rec1, rec2);
}

@transition
function full_1(recs: (Record, Record)) -> (Record, Record) {
    return recs;
}

type TwoRecords = (Record, Record);
@transition
function full_alias(recs: TwoRecords) -> TwoRecords {
    return recs;
}

// Dummy main function
function main() {}