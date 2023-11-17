# Groth16: Theory to Practice
## Theory
The [Groth16](https://eprint.iacr.org/2016/260.pdf) algorithm is a non-interactive proof system used for zero-knowledge proofs, proposed by Jens Groth in 2016. Groth16 is one of the most widely used zero-knowledge proof systems and is built on previous work, primarily referencing protocols such as [Pinocchio](https://eprint.iacr.org/2013/279.pdf), [GGPR13](https://eprint.iacr.org/2012/215.pdf), and [BCTV14](https://eprint.iacr.org/2013/879.pdf).

Groth16 solves the problem of how to perform zero-knowledge proofs for complex computations. In zero-knowledge proof, the prover wishes to prove a statement about private inputs to the verifier, without revealing any information about the inputs themselves. This input is usually referred to as witness in ZkSnark. It is a solution satisfying certain conditions. The Groth16 proof system allows the prover to create a proof that they know a solution without revealing the actual solution to the verifier. As we previously introduced the basic concept of ZKSNARK(zero-knowledge succinct non-interactive argument of knowledge), it is a specific algorithm implementation of ZKSNARK.

### Features:
- **Non-interactive**: Groth16 is a non-interactive proof system, meaning that only one communication is needed between the prover and the verifier.
- **Short proofs**: Groth16 proofs are short, with a size of only O(1), independent of the complexity of the proof.
- **Fast verification**: Groth16 proofs can be verified in polynomial time.
- **Trusted setup**: Groth16 requires a trusted setup phase for generating common parameters.
- **Applicable to any NP problem**: Groth16 proof system can construct zero-knowledge proofs for any NP problem.

## Protocol flow
Groth16 protocol is based on bilinear mapping and elliptic curve cryptography. The protocol consists of four phases: setup, proving, verification, and an optional preprocessing phase.

* **setup**, a set of public parameters is generated, including generators on an elliptic curve and some random numbers. Normally, a trusted third party is required to generate these parameters. However, it is considered insecure since the random numbers used to generate the CRS can be easily leaked. In this case, the attacker who gains access to these random numbers can forge fraudulent proofs. In practice, to avoid this issue, multiparty computation (MPC) can be used to generate this parameters in a more decentralized way, such as the [Powers of Tau ceremony](https://eprint.iacr.org/2022/1592.pdf).

* **proving**, the prover uses the public parameters and private input to generate the proof. The proof will contain some commitments about the computation and points on the elliptic curve to prove that the prover knows a solution that satisfies certain conditions while maintaining the privacy of the solution.

* **verification**, the verifier uses the public parameters and the proof provided by the prover to verify the truthfulness of the statement,i.e. the prover knows a solution that satisfies certain conditions, without knowing the prover's private input.

* **preprocessing**（optional）, some computations related to the public parameters can be preprocessed to improve verification efficiency and speed up the process, especially for large computations and multiple proof verification scenarios.

The core of the Groth16 protocol is to use bilinear mapping to transform the problem from the elliptic curve group to a scalar problem and reduce the number of pairing. In this way, the prover can provide a zero-knowledge proof about the computation, and the verifier only needs to perform simple pairing (bilinear mapping) calculations in a finite field to verify the correctness of the proof.


## Principle of the Protocol
> This part will involve some theoretical mathematical calculations and formulas. We hope you can patiently read through them and also present to you in a relatively concise form. Of course, if you find it difficult to continue, you can directly skip to the practical part below. After completing all the practices, you can come back to better understand the theoretical part.

Let's review the construction steps of zk-SNARKs as introduced in the ZKSANRKS [preview article](https://github.com/Zkvers/substrate-zk/blob/master/zk-tutorials/ZKSNARKS.md). Here, we will summarize them in more detail. A zk-SNARKs protocol framework consists of the following steps:

1. Prover 𝓟 wants to prove that he has witness $w$ that satisfies a computation relation 𝑹.
2. Prover 𝓟 wants to prove that he has witness $w$ that satisfies 𝑹's corresponding arithmetic circuit.
This arithmetic circuit is an interpretation of 𝑹. It only consists of addition and multiplication gates. The circuit is a Directed Acyclic Graph on which at each node of the graph an arithmetic operation is performed.
3. Prover 𝓟 wants to prove that he has witness $w$ that satisfies R1CS constraints.
R1CS further interprets the arithmetic circuits as a set of rank-1 quadratic constraints. To represent each gate circuit, it representes it as a relation between vectors. To be more specific, the inner product of witness vector $w$ and matrices $A$, $B$, $C$ satisfies: $w \cdot C = w \cdot A * w \cdot B$ in which matrices $A$, $B$, $C$ consist of vectors which are used to represent all the constraints in R1CS.
4. Converting R1CS to QAP. QAP implements the exact same logic except using polynomials instead of dot products between vectors. 
By this step, the problem is further transformed into the prover proving he knows a witness $w$ that satisfies: 
$w \cdot C(x) - w \cdot A(x) * w \cdot B(x) = 0$.
when evaluat this polynomial at each constraint's corresponding $x$. e.g. if $x=1$ represents the first constraint, then this polynomial equals 0 and $C(x), A(x), B(x)$ (they are in the form of vector of polynomials) equals to this constraint's vector in R1CS when $x$ is equal to 1.  
We can construct $C(x), A(x), B(x)$ using the common Lagrange interpolation.
By converting the problem into a polynomial from, the verifier can check all constraints simultaneously which makes the verification process very efficient.
5. According to the previous step, we have a target polynomial $Z(x)$ divides the QAP polynomial, i.e., $Z(x) | [w \cdot C(x) - w \cdot A(x) * w \cdot B(x)]$.
A target polynomial is a polynomial whose roots are all constraint’s corresponding $x$. e.g. if the problem has 4 constraints with correspongding $x=1,2,3,4$, then $Z(x)=(x-1)(x-2)(x-3)(x-4)$.
6. Generate the CRS based on circuit polynomials $A(x), B(x), C(x)$.
7. Using CRS and scalar multiplication， the prover can evaluate QAP polynomial, target polynomial, and quotient polynomial (the result of dividing QAP polynomial by the target polynomial) on elliptic curve.
8. Verifier 𝓥 reconstructs the division relation by bilinear pairing on elliptic curves, verifies the correctness of vector $w$ without knowing vector $w$.


## Initialization of CRS

In this section, we talk about how to generate CRS based on $A(x), B(x), C(x)$. 

During the setup phase, we randomly select the following field elements（finite field of order $p$） (all of which are toxic waste and must be destroyed later):$\alpha$, $\beta$, $\gamma$, $\delta$, $\tau$.

Given:
* A computation with $m$ variables, where $l$ of them are public inputs.
* $n$ constraints and their corresponding $x=1,2,3,...,n$.
* According to the QAP definition, $Z(x)=(x-1)\cdot(x-2)\cdot(x-3)\cdot\dots\cdot(x-n)$.
* $L_i(x) = \beta\cdot A_i(x) + \alpha\cdot B_i(x) + C_i(x)$ for $i=0,1,...,m$.


Here, $A_i(x), B_i(x), C_i(x)$ are univariate polynomials.  They are elements of polynomial vectors $A(x), B(x), C(x)$ respectively. Since there are $m$ variables and the first element of the witness vector is always 1, these polynomial vectors are of length $m+1$.

The CRS (Common Reference String) is divided into two sets. The part needed for proving is denoted as `PK (Proving Key)`, and the part needed for verification is denoted as `VK (Verification Key)`.

### Proving key
* Values related to G1:
$$(\alpha, \delta, 1, \tau, \tau^2, \tau^3, \dots, \tau^{n-1}, L_{l+1}(\tau)/\delta, L_{l+2}(\tau)/\delta, \dots, L_m(\tau)/\delta, Z(\tau)/\delta, \tau\cdot Z(\tau)/\delta, \tau^2\cdot Z(\tau)/\delta, \dots, \tau^{n-2}\cdot Z(\tau)/\delta)_{1}$$

* Values related to G2:
$$(\beta, \delta, 1, \tau, \tau^2, \tau^3, \dots, \tau^{n-1})_{2}$$ [skalman](# "this is the first time you talk about G1 and G2, please explain")

>Here G1，G2 are two groups in elliptic curve bilinear pairing and values in the parentheses are actually group elements scalar multiplied by the group generator and subscripts of these values indicate which group they are from.

Elliptic cure pairing are used in Groth16 to check  equations on multiplications without revealing the hidden values. Assume the generators of G1, G2 are $g1, g2$ respectively and function $e$ is the bilinear mapping that maps two group elements, one in G1 and one in G2, to an element in a new group Gt. For example, if you want to check $a \times b =c$ without revealing $a,b,c$, you can simply check $e(a \cdot g1, b \cdot g2) = e(c \cdot g1, g2)$. More explainations about [bilinear pairing](https://alinush.github.io/2022/12/31/pairings-or-bilinear-maps.html). 


As the circuit is known to the prover, the proving key will also include the coefficients of the polynomials $A(x) = (A_0(x)$, $A_1(x)$, $\dots$, , $A_m(x))$, $B(x) = (B_0(x)$, $B_1(x)$, $\dots$, $B_m(x))$ and $C(x) = (C_0(x)$, $C_1(x)$, $\dots$, $C_m(x))$, as well as the polynomial $Z(x)$.

### Verification key
* Values related to G1: $(1, L_0(\tau)/\gamma, L_1(\tau)/\gamma, L_2(\tau)/\gamma, \dots, L_l(\tau)/\gamma)_{1}$

* Values related to G2: $(1, \gamma, \delta)_{2}$

* Values related to Gt: $(\alpha_{1}\cdot\beta_{2})^{t}$



## Proof generation

Given a witness vector $w = (1, w_1, w_2, w_3, …, w_m )$, and two random field elements r and s, a **proof** is made of two points in G1 and one point in G2:
* $A1$ is a point in G1:

$A1 = \alpha_1 + w_0A_0(\tau)_1 + w_1A_1(\tau)_1 + w_2A_2(\tau)_1 + w_3A_3(\tau)_1 + \dots + w_mA_m(\tau)_1 + r\delta_1$

>All elements in this equation are elements of G1(The subscript number in the lower right corner is used for group identification), eg:$\alpha_1 = \alpha \cdot g_1$, $A_0(\tau)_1 = A_0(\tau) \cdot g_1$ , $A_1(\tau)_1 = A_1(\tau) \cdot g_1$ and so on.

$A_i(\tau)\cdot g_1$ can be calculated from the coefficients of $A_i(x)$ by multiplying each coefficient with the corresponding term $g_1, \tau\cdot g_1, \tau^2\cdot g_1, \ldots$, which are points made available by the CRS (common reference string).

* $B2$ is a point in G2: [skalman](# "your definition here is recursive you have B_2 on both side, you really need a better notation")

$B2 = \beta_2 + w_0B_0(\tau)_2 + w_1B_1(\tau)_2 + w_2B_2(\tau)_2 + w_3B_3(\tau)_2 + \dots + w_mB_m(\tau)_2 + s\delta_2$


* $C1$ is a point in G1:

$C1 = w_{l+1}\cdot(L_{l+1}(\tau)/\delta)_1 + \dots + w_m\cdot(L_m(\tau)/\delta)_1 + H(\tau)\cdot(Z(\tau)/\delta)_1 + s\cdot A_1 + r\cdot B_1 - r\cdot s\cdot \delta_1$

$H(\tau)\cdot Z(\tau)/\delta \cdot g_1$ can be calculated from the coefficients of $H(x)$ by multiplying each coefficient with the corresponding term $(Z(\tau)/\delta) \cdot g_1, (\tau\cdot Z(\tau)/\delta) \cdot g_1,(\tau^2\cdot Z(\tau)/\delta) \cdot g_1, \ldots$, which are points made available by the CRS (common reference string).

The quotient polynomial $H(x)$ can be calculated by applying vector dot product operations between witness vector $w$ and polynomial vectors $A(x)$, $B(x)$, and $C(x)$. Then we have $H(x) = [w\cdot A(x) \cdot w\cdot B(x) - w\cdot C(x)] / Z(x)$.


We also need to calculate $B1$ in $G1$: $B_1 = \beta_1 + w_0B_0(\tau)_1 + w_1B_1(\tau)_1 + \dots + w_mB_m(\tau)_1 + s\delta_1$



## Proof Verification

To verify a proof, we just need to use bilinear pairing to check the following equality:

$A1 \cdot B2 = \alpha_1 \cdot \beta_2 + (w_0L_0(\tau)/y + w_1L_1(\tau)/y + \dots + w_lL_l(\tau)/y)_1 \cdot y_2 + C1 \cdot \delta_2$

In this equation, each multiplication requires one pairing and addition only requires multiplication in Gt. Thereby, this process requires only three pairings, considering that $\alpha_1 \cdot \beta_2$ is already available in the verifying key.

* the left term $A1 \cdot B2$:

$[\alpha + A(\tau) + r\delta] \cdot [\beta + B(\tau) + s\delta]$

$= \alpha \cdot \beta + \alpha \cdot B(\tau) + s\alpha\delta + A(\tau) \cdot \beta + A(\tau) \cdot B(\tau) + sA(\tau)\delta + r\delta \cdot \beta + r\delta \cdot B(\tau) + s \cdot r\delta\delta$

$= A(\tau) \cdot B(\tau) + \alpha \cdot \beta + \alpha \cdot B(\tau) + \beta \cdot A(\tau) + s\alpha\delta + sA(\tau)\delta + r\beta\delta + rB(\tau)\delta + s \cdot r\delta\delta$

* the right term:

$\alpha \cdot \beta + L(\tau) + H(\tau) \cdot Z(\tau) + s\alpha\delta + sA(\tau)\delta + s \cdot r\delta\delta + r\beta\delta + rB(\tau) \cdot \delta + s \cdot r\delta\delta - r \cdot s\delta\delta$

$= \alpha \cdot \beta + \beta \cdot A(\tau) + \alpha \cdot B(\tau) + C(\tau) + H(\tau) \cdot Z(\tau) + s\alpha\delta + sA(\tau)\delta + s \cdot r\delta\delta + r\beta\delta + rB(\tau) \cdot \delta$

$= C(\tau) + H(\tau) \cdot Z(\tau) + \alpha \cdot \beta + \alpha \cdot B(\tau) + \beta \cdot A(\tau) + s\alpha\delta + sA(\tau)\delta + r\beta\delta + rB(\tau) \cdot \delta + s \cdot r\delta\delta$


As you can see,  the terms followed after $A(\tau) \cdot B(\tau)$ in $A \cdot B$ are identical to terms followed after  $C(\tau) + H(\tau) \cdot Z(\tau)$ in the right side of the equation. Thereby, in this equation, they can be canceled out. If the above equality holds, it means that [skalman](# "you need to define the pairing operation here and show when and why the verifier needs to apply the pairing"):

$A(\tau) \cdot B(\tau) = C(\tau) + H(\tau) \cdot Z(\tau)$

As you can see, Groth16 does not use the “knowledge of coefficient” (that requires in the proof two group elements for each polynomial) , but uses the secret field elements $\alpha$, $\beta$ to force $A1$, $B2$ and $C1$ to use the same vector $w$. The other two secret field elements $γ$, $δ$ are used to make the public input independent from the other witness components. So that is why the verifier can apply the pairing to verify the proof.

Ok, if the verification succeeds, the prover does have a valid witness $w$.


## Practice1: Multilication 
In the first example, we start with the simplest `multiplication` case "a * b = c". The public input is `c`, and the prover needs to prove to someone else that he know two values `a` and `b`, whose product is c, without revealing the values of a and b. You can go to directory `substrate-zk/snarkjs-bellman-adapter` to refer to its readme, complete `Pre-requirements` and `Use the adapter` section, and then come back here. I will explain to you what happens in the process and then show you how to verify the proof generated by snarkjs on our substrate-based chain with bellman.

### Generate a proof by snarkjs 
In the previous practice, you have run `./start.sh Multiplication` command. We will delve into the details of `start.sh` and explain in detail what it does.

1. start a new powers of tou ceremony(bls12_381)
```
snarkjs powersoftau new bls12_381 12 pot12_0000.ptau -v
```
The first parameter after new refers to the type of curve you wish to use. At the moment, we use `bls12-381` and in this way, we can adapt to Bellman very well which also using this curve.
The second parameter, in this case 12, is the power of two of the maximum number of constraints that the ceremony can accept: in this case, the number of constraints is 2 ^ 12 = 4096(Because the number of constraints we are testing here is not large, we chose 12. If your number of constraints is larger, you can choose a larger value.). The maximum value supported here is 28, which means you can use snarkjs to securely generate zk-snark parameters for circuits with up to 2 ^ 28 (≈268 million) constraints.


[skalman](# "could you explain what is 12 and why you have chosen to stop at power 12?")
2. contribute to the `powers of tau ceremony` and prepare circuit
```shell
# contribute to the ceremony
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
snarkjs powersoftau contribute pot12_0001.ptau pot12_0002.ptau --name="Second contribution" -v -e="some random text"
snarkjs powersoftau export challenge pot12_0002.ptau challenge_0003
```

```shell
snarkjs powersoftau challenge contribute bls12_381 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot12_0002.ptau response_0003 pot12_0003.ptau -n="Third contribution name"
```
Here, we export the challenge for the third contributor and it allows you to use different types of software in a single ceremony which will be more secure.

# verify the ptau
```
snarkjs powersoftau verify pot12_0003.ptau
snarkjs powersoftau beacon pot12_0003.ptau pot12_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
```
The `beacon` command creates a `ptau` file with a contribution applied in the form of a random beacon.

We need to apply a random beacon in order to finalise the first phase of trusted setup.

>To paraphrase Sean Bowe and Ariel Gabizon, a random beacon is a source of public randomness that is not available before a fixed time. The beacon itself can be a delayed hash function (e.g. 2^40 iterations of SHA256) evaluated on some high entropy and publicly available data. Possible sources of data include: the closing value of the stock market on a certain date in the future, the output of a selected set of national lotteries, or the value of a block at a particular height in one or more blockchains. E.g. the hash of the 11 millionth Ethereum block (which as of this writing is some 3 months in the future). See [here](https://eprint.iacr.org/2017/1050.pdf) for more on the importance of a random beacon.

For the purposes of this tutorial, the beacon is essentially a delayed hash function evaluated on `0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` (in practice this value will be some form of high entropy and publicly available data of your choice). The next input -- in our case `10` -- just tells snarkjs to perform `2 ^ 10` iterations of this hash function.

Note that security holds even if an adversary has limited influence on the beacon.

```shell
# prepare the phase 2 (ceremony with circuit) 
snarkjs powersoftau prepare phase2 pot12_beacon.ptau pot12_final.ptau -v
```
We're now ready to prepare phase 2 of the setup (the circuit-specific phase).

Under the hood, the `prepare phase2` command calculates the encrypted evaluation of the Lagrange polynomials at tau for `tau`, `alpha*tau` and `beta*tau`. It takes the beacon `ptau` file we generated in the previous step, and outputs a final `ptau` file which will be used to generate the circuit proving and verification keys.

```shell
snarkjs powersoftau verify pot12_final.ptau
```

The `verify` command verifies a powers of tau file. Before we go ahead and create the circuit, we perform a final check and verify the final protocol transcript. Notice there is no longer a warning informing you that the file does not contain phase 2 precalculated values.

3. compile the cicuit and run circuit ceremony

The circom command takes one input (the circuit to compile, in our case circuit.circom) and three options:
- `r1cs`: generates circuit.r1cs (the r1cs constraint system of the circuit in binary format).
- `wasm`: generates circuit.wasm (the wasm code to generate the witness – more on that later).
- `sym`: generates circuit.sym (a symbols file required for debugging and printing the constraint system in an annotated mode).
```shell
  circom circuit.circom --r1cs --wasm --sym -p bls12381
```
`-p` params is the target curve to generate r1cs constrains, it tells circom which fields the polynomials are interpolated in.

print some information about the circuit and create the `witness` (values of all the wires) for our inputs.
```shell
  snarkjs info -r circuit.r1cs
  snarkjs r1cs print circuit.r1cs circuit.sym

  #export the r1cs to json
  snarkjs r1cs export json circuit.r1cs circuit.r1cs.json

  cd circuit_js
  node generate_witness.js circuit.wasm ../input.json ../witness.wtns
  cd ..
```

`Groth16` requires `a trusted ceremony for each specific circuit`. So we need the follow command to setup a ceremony

```shell
  #setup the phase 2 circuit ceremony
  snarkjs groth16 setup circuit.r1cs pot12_final.ptau circuit_0000.zkey

  # first contribution
  snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="1st Contributor Name" -v

  # second contribution
  snarkjs zkey contribute circuit_0001.zkey circuit_0002.zkey --name="Second contribution Name" -v -e="Another random entropy"

  # Third contribution
  snarkjs zkey export bellman circuit_0002.zkey  challenge_phase2_0003
  snarkjs zkey bellman contribute bls12_381 challenge_phase2_0003 response_phase2_0003 -e="some random text"
  snarkjs zkey import bellman circuit_0002.zkey response_phase2_0003 circuit_0003.zkey -n="Third contribution name"

  # verify the latest key
  snarkjs zkey verify circuit.r1cs pot12_final.ptau circuit_0003.zkey
  snarkjs zkey beacon circuit_0003.zkey circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
```

Before we go ahead and export the verification key as a json, we perform a final check and verify the final protocol transcript (zkey). Last, We can export the verification key from `circuit_final.zkey` into `verification_key.json`.

```shell
  snarkjs zkey verify circuit.r1cs pot12_final.ptau circuit_final.zkey
  snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
```

We can create the proof and this command will generate the files:
- proof.json: contains the actual proof
- public.json contains the values of the public inputs and output.

```shell
snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json
```
> Do you remember what we wanted to prove? We wanted to prove that we know the values of `a` and `b`, but here we didn't directly prove that we know a and b by presenting their values `1` and `33` as proof. Instead, we used ZKP to convert them into a special representation, which is the `proof.json` in our case. Anyone who knows the value of proof.json cannot deduce what the values of a and b are. However, for a verifier, it is very fast to verify that we do know the values of a and b by the proof.json. This is the magic of ZKP!

In the last, we use the snarkjs to verify the proof
```shell
snarkjs groth16 verify verification_key.json public.json proof.json
```

Okay, so far, we have learned about the process of generating a proof with snarkjs and verifying the proof with bellman. The next step is to use bellman to verify the proof generated by snarkjs on the substrate-based chain, which is our ultimate goal.

### Verify the proof by bellman on substrate
1. start the substrate chain
go to the dir `~/..../substrate-zk`
```
./target/release/node-template --dev --tmp
```

2. get the proof and verification key
If you have run all the practice before, go to dir `snarkjs-bellman-adapter/circuit/Multiplication`, you can find two file `proof_hex.json` and `vkey_hex.json` which we will use in the next step.

For example, My proof_hex.json is:
```json
{
  "pi_a": "0x002f6a48c20c57574a73ddc00a78610a1de3598667d1f6809b530c5372ddb31b4b083b51554b64caae036d9ba1f6c293153d93a707bae6c379e91e672e1f3a44f9890e9df715b6d10c59882a986eb99c56bf7762f2578779f39d64ec4822be2a",
  "pi_b": "0x13c87e31c7ccf1fc55ba6ba1c9763c45453960c3eed28ca2a4ecf26c7f26d68a765f769880a05d46491be2906d497eff19ac2a6a2f8a0b3963aed11f1a97174978c34328d9a1092767a6d3f709cf3b025c01f40f305e2c7a1c41cb536b9f5145061e05534b0acd28330f758b043966f590f627e6cf05088e548f2b5553e392870b4a1cbfb1740b2a6e0951601b143d12190570b9b9e79a2ce12c071543669395a0a2f1d2c689e08e3511bbc944eb682aeaba1e2e6a69acda45dfca4abd5258c4",
  "pi_c": "0x074aa9b1a5e8326c7650aa682122032172acb27c50c363d9108d42d20a47fde2b8914d05234127c99d59f5dff1508e4e089451dcc616d0d0c70dfbcd885c06c181520fb30c626bc9d48aa7c8e732468a66dee719f98247dc9de06334fad7f254"
}
```
vkey_hex.json is:
```json
{
  "alpha_1": "0x0e67daf9a85092df9874ca7f5d7da340fc2a5aa1dcd9b05141372f70151e4b04088f9d27c750145b224c00af9010f4fa075ac903ddc76ad4fba5d5fc4eacacee84505f4196ba6c8336eb7618a327344a5e911fddbae06400aa77a4b72ac6e8dd",
  "beta_2": "0x110747d3880e11d9a2343c43901c8912570145d429653bc134d12c289c6b4445703f6a27a6ecba5ba059f7700a5a9102173fd13bd10dc2eb2d4f7fe321af2de11b3f7aedc51568b20268f368a733cdccb93166982578654e203c75f3b265e39100ab9eac7af6237c6b1e57c2eb6e47edbe73fd50c9ed4ba7690ef7adddcb3465924ca9dfa446d5ee27025eb86ff3d70812a299ee8e95ae1392ea883e4a26a90b9aa08ff0e441ef7e83a9b6a4c005197803e3e1dd0b2146213a5bce849cb2433f",
  "gamma_2": "0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801",
  "delta_2": "0x0af4427a2ce60c02cb6426d650d03ad5e1059bff94d21e5a83d6a06b6327c48dbcfeb66ff0e8ecc265f1bb8c53681ca3132850056c7e7d35670927278fcd51c3d68c829d86520fc78dc32901f51ba9bd6db780b7b7d4ecc812060da20b2912c606e645b538ca41ce11acb642d49192f4105fd16aa1710a18505be03c558a9a1c7c1a5ee73195f73afed91e5743600387006ee1188191e9cab2061ea2dd8216ec23da8b64708aa6e4ec98ec8cd595c38105ac0a1b2347ec4c5c42d3ed78e6bb06",
  "ic0": "0x0d65c0b02cb1875e44296b4fa12bed262e6fa486989698cc5b8a8c1960e6fc9f6eecc19fc275958a036bd8ac4a1ba70c156c1738cd8975e0b4b3060b5e0b04e5944b3f058d946b2bfe278c304f39fdf1860f568696b07ebc4590bc2f7fc2bc86",
  "ic1": "0x07f6584cf982f7d8c2f825325de03779cc9badb4f8e04f3f3326c686f83f931d6b41b61271bec2b180c1f5df8cfa7779109e4f1e010838aac80adf5fd12540984750347d0bd8445faec70e1c895cc6805cfcf8306e642acd5e3ca74a724661f5"
}

```
> Of course, the generated proof is different each time. You can follow the steps below based on your actual file content.

3. set zk parameter on substrate-based chain
We can open the PolkadotJS browser, and then navigate to the `verifier` pallet, and select the method `setZkKeys`, like this:
![set zk parameter](../../img/set_zk_parameters.jpg)

We assume Alice is the verifier, who is the creator of the multiplication zk-SNARK circuit, where the public input is 33. Next are the parameters of the verification key, which are the contents of the vkey_hex.json file we mentioned earlier.

If you pass the parameter correctly, it will be store in the chain, like this event:
![set zk event](../../img/set_zk_event.jpg)

4. prover set proof and check the result

Now, Bob sees the ZKP problem set by Alice and wants to prove that he knows the result of a*b. Bob uploads his proof, which is the content of proof_hex.json. We select the `verify` method as Bob.
![set proof](../../img/set_proof.jpg)
Once we execute this method, the bellman on the chain will verify the correctness of this proof of snarkjs.
![verification passed](../../img/verification_passed.jpg)
Great! If you see the above fig, which means your proof passed the verification!

## Practice2: Merkle Tree
After running the previous example, we now have a basic understanding of the zk-SNARK execution process. 

In this example, we will introduce a more advanced example: `verify the correctness of a Merkle tree using zk-SNARKs`. This is the simplest example of implementing `rollup`. The specific circuit can be found in `substrate-zk/snarkjs-bellman-adaper/circuit/MerkleTree`, which provides a detailed introduction to the circuit. If you want to run the example, the method is the same as the first Multiplication example.


- First, generate `input.json` for the Merkle tree
  go into the dir `snarkjs-bellman-adapter/circuit/MerkleTree` and run with: 
  ```shell
  npm install && node generate_merkle_root_input.js
  ```

- Generate proof and verification key with `circuit.circom` and `inputs.json` in the dir `circuit` of this project by `start.sh`. And then generate the the uncompressed hex format proof and verification key.
```
cd ../../ && ./start.sh MerkleTree
cd prove && npm install
cd src && node adapter.js MerkleTree
cd ../../verify/src/adapter
CIRCUIT_DIR_NAME=MerkleTree PUBLIC_INPUT=0 cargo test snark_proof_bellman_verify -- --nocapture
``````

In this case, out public input is "0". If you see the below output, which means the verification with bellman passed.

```
running 1 test
THE CIRCUIT YOU ARE TESTING IS : MerkleTree
PUBLIC INPUT is : 0
>>>>start encode the uncompressed data to Affine<<<<<
>>>>end verification<<<<<<<
test adapter::snark_proof_bellman_verify ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

If you want to learn more about circuit implementation, you can refer to the documentation in `snarkjs-bellman-adapter/circuit/MerkleTree/README.md`, which provides detailed explanations.

If you want to verify this proof on the chain, the process is the same as the Multiplication circuit above.
