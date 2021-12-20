/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/


@transition
function three_in_out(rec1: Record, rec2: Record, rec3: Record) -> (Record, Record, Record) {
    return (rec1, rec2, rec3);
}

@transition
function four_in_tup(recs: (Record, Record, Record, Record)) -> (Record, Record, Record, Record) {
    return recs;
}

type FiveRecords = (Record, Record, Record, Record, Record,);
@transition
function five_in_alias(recs: FiveRecords) -> FiveRecords {
    return recs;
}



