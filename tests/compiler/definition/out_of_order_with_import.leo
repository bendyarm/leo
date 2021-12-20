/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

@test 
function fake_test() {}

function main(y: bool) -> bool {
    return y == true;
}

// use std import to test import order
// TODO: Blake2s doesn't exist anymore, change this example when we have other stdlib circuits
// As of now, this test has the same semantics and `out_of_order.leo`
// import std.unstable.blake2s.Blake2s;

circuit Foo {}
