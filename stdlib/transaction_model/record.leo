circuit Record {
    owner: address;
    value: i64; // Effectively a u62 as two MSBs are required to be zero.
    payload: [u8; 128];
    program_id: [u8; 48];
};