/*
namespace: Compile
expectation: Pass
input_file:
    - inputs/dummy.in
*/

import std.transaction_model.record.Record;


function records_equal(rec1: Record, rec2: Record) -> bool {
    return rec1.owner == rec2.owner && rec1.value == rec2.value && rec1.payload == rec2.payload && rec1.program_id == rec2.program_id;
}

function main() -> bool {
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

    return records_equal(rec1, rec2);
}
