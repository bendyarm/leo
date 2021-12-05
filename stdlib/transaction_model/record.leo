@CoreCircuit
circuit Record {
    owner: address;
    value: i64; // Effectively a u62 as two MSBs are required to be zero.
    payload: [u8; 128];
    program_id: [u8; 48];

    function is_dummy(self) -> bool {
        // TODO: Need some way of natively getting program_id
        return value == 0 && payload == [0; 128];
    }
}