/*
namespace: Compile
expectation: Fail
*/

// TODO: Blake2s doesn't exist anymore, change this example when we have other stdlib circuits
// As of now, this test has the same semantics and `core_package_invalid.leo`

import std.unstable.blake2s.BadCircuit; // `BadCircuit` is not included in the blake2s package

function main() {}
