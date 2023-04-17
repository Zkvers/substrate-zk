# substrate-zk

A Substrate-based chain supporting `ZK-SNARKs` verification.

## Download substrate-zk
```
git clone --recurse-submodules https://github.com/Zkvers/substrate-zk
```

## Environment
Rust version:   
```
1.65.0-aarch64-apple-darwin (default)  
rustc 1.65.0 (897e37553 2022-11-02)
```
## Zero knowledge proof and verification on Substrate-based chain
In this project, we have theory and practice in Zero knowledge proof, Mainly description exists in `proof system` directory and [README](https://github.com/Zkvers/substrate-zk/blob/master/zk-tutorials/ZKSNARKS.md) of `zk-tutorial` dir and the [README](https://github.com/Zkvers/snarkjs-bellman-adapter/blob/main/README.md) of `snarkjs-bellman-adapter`
- **groth16**: refer to `substrate-zk/zk-tutorials/proof-system/theory_to_practice` and you can directly operate with `practice1` section where you can build the full process which using `bellman` to verify the proof generated by `snarkjs`
- **plonk**: coming soon...