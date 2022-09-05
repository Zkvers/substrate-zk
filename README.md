# substrate-zk

A Substrate-based chain supporting ZK-SNARK(chaneg Bellman's verifier to `no_std` to adapt substrate)
you can use this `verifier pallet` to verify the proof(uncompressed data) with `bls12_381` curve

## Build and Run
you should have known how to build a substrate-based chain. 

build the chain:
```
cargo build --release
```

run the chain:
```
./target/release/node-template --dev --tmp
```

## Test the zk-verifier
you can use this [adapter](https://github.com/DoraFactory/snarkjs-bellman-adapter) to generate uncompressed proof and verification.
i will show a minimal example with this [circuit](https://github.com/DoraFactory/snarkjs-bellman-adapter/tree/main/circuit): `a*b=c`.
- you can call the function `generate_proof_vkey` of `pallet-maci-verifier` to store `proof` and `verification key`
- call the `verifier` function to verify the proof
