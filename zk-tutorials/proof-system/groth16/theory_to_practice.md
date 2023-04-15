# Groth16: Theory to Practice
## Theory
The Groth16 algorithm is a non-interactive proof system used for zero-knowledge proofs, proposed by Jens Groth in 2016. Groth16 is one of the most widely used zero-knowledge proof systems and is built on previous work, primarily referencing protocols such as Pinocchio, GGPR13, and BCTV14.

Groth16 solves the problem of how to perform zero-knowledge proofs for complex computations. In zero-knowledge proof, the prover wishes to prove a statement about private inputs to the verifier, without revealing any information about the inputs themselves. The Groth16 proof system allows the prover to create a proof that they know a solution satisfying certain conditions, without revealing the actual solution to the verifier. As we previously introduced the basic concept of ZKSNARK, it is a specific algorithm implementation of ZKSNARK.

### Features:
- **Non-interactive**: Groth16 is a non-interactive proof system, meaning that only one communication is needed between the prover and the verifier.
- **Short proofs**: Groth16 proofs are short, with a size of only O(1), independent of the complexity of the proof.
- **Fast verification**: Groth16 proofs can be verified in polynomial time.
- **Trusted setup**: Groth16 requires a trusted setup phase for generating common parameters.
- **Applicable to any NP problem**: Groth16 proof system can construct zero-knowledge proofs for any NP problem.

## Protocol flow
Groth16 protocol is based on bilinear mapping and elliptic curve cryptography. The protocol consists of four phases: setup phase, proving phase, verification phase, and an optional preprocessing phase.

* setup phase, a set of public parameters is generated, including generators on an elliptic curve and some random numbers. A trusted third party is required to generate these parameters to prevent potential tampering. To avoid this issue, multiparty computation (MPC) can be used to generate the parameters, such as the Powers of Tau ceremony.

* proving phase, the prover uses the public parameters and private input to generate the proof. The proof will contain some commitments about the computation and points on the elliptic curve to prove that the prover knows a solution that satisfies certain conditions while maintaining the privacy of the input.

* verification phase, the verifier uses the public parameters and the proof provided by the prover to verify the truthfulness of the statement, without knowing the prover's private input.

* optional preprocessing phase, some computations related to the public parameters can be preprocessed to improve verification efficiency and speed up the process, especially for large computations and multiple proof verification scenarios.

The core of the Groth16 protocol is to use bilinear mapping to transform the problem from the elliptic curve group to a scalar problem. In this way, the prover can provide a zero-knowledge proof about the computation, and the verifier only needs to perform simple bilinear mapping calculations to verify the correctness of the proof.

## Principle of the Protocol
> This part will involve some theoretical mathematical calculations and formulas. We hope you can patiently read through them and also present to you in a relatively concise form. Of course, if you find it difficult to continue, you can directly skip to the practical part below. After completing all the practices, you can come back to better understand the theoretical part.

Let's review the construction steps of zk-SNARKs as introduced in the ZKSANRKS Interview. Here, we will summarize them in more detail. A zk-SNARKs protocol framework consists of the following steps:

1. Generate CRS (Common Reference String) that is independent of the circuit using secure multiparty computation.
2. Prover 𝓟 proves that they have witness 𝒘𝒊𝒕𝒏𝒆𝒔𝒔 that satisfies an arbitrary computation relation 𝑹.
3. Prover 𝓟 proves that they have witness 𝒘𝒊𝒕𝒏𝒆𝒔𝒔 that satisfies any polynomial-time algorithm.
4. Prover 𝓟 proves that they have witness 𝒘𝒊𝒕𝒏𝒆𝒔𝒔 that satisfies R1CS constraints.
5. The inner product of vector $\vec{s}$ and matrices $U$, $V$, $W$: $\vec{s} \cdot W = \vec{s} \cdot U * \vec{s} \cdot V$.
6. The vector $\vec{s}$ combines polynomials as follows: $\vec{s} \cdot W(x) - \vec{s} \cdot U(x) * \vec{s} \cdot V(x) = 0$.
7. The target polynomial $z(x)$ divides the QAP polynomial $z(x) | [\vec{s} \cdot W(x) - \vec{s} \cdot U(x) * \vec{s} \cdot V(x)]$ forming an NP problem.
8. Generate the CRS based on circuit polynomials 𝑾(𝒙), 𝑼(𝒙), 𝑽(𝒙).
9. Prover puts QAP polynomial, target polynomial, and quotient polynomial on elliptic curve discrete logarithm point.
10. Verifier 𝓥 reconstructs the division relation on elliptic curve discrete logarithm points, verifies the correctness of vector $\vec{s}$, but does not know vector $\vec{s}$.


### Initialization of CRS

* Random $\tau$ chosen during trusted setup as well as $\alpha$, $\beta$
* $n$ is number of gates.
* $u_i$, $v_i$ and $w_i$ are the QAP polynomials created from the R1CS circuit

$CRS: \{g_1^\alpha\, g_1^\beta, g_1^\delta, \{\{ g_1^{\tau^i}\}, \{g_1^{\alpha \tau^i}\}, \{g_1^{\beta \tau^i}\}\}_{i=0}^{n-1}, \{g_1^{\beta u_i(\tau) + \alpha v_i(\tau) + w_i(\tau)}\}_{i=0}^n\}, \{g_1^{\frac{\tau^it(x)}{\delta}}\}_{i=0}^{n-1},\\g_2^\beta, g_2^\delta, \{g_2^{\tau ^i}\}_{i=0}^{n-1}\}$
> $g_1$ is the generator of $G_1$, $g_2$ is the generator of $G_2$

The CRS (Common Reference String) is divided into two sets. The part needed for proving is denoted as `PK (Proving Key)`, and the part needed for verification is denoted as `VK (Verification Key)`.

### Prover
**Settings**:
* Regular proof is $\pi = (A, B, C)$. 
* $l+1$ is number of public inputs (+1 as first public input is "1" in R1CS)
* Vector of `public inputs` $\phi = (a_0, \dots, a_{l})$

**Extension**: 
Prover computes the regular proof and computes the following additionally 
* $S = g_1^{\sum_{i=0}^{l} a_i (\beta u_i(\tau) + \alpha v_i(\tau) + w_i(\tau))}$ 
> Construct linear combination relationships, and place the coefficients of the combined polynomial on the elliptic curve discrete logarithm points, forming a discrete logarithm problem that is difficult to solve.
* Derive random challenge $z = H(\phi, \pi, S)$
* Compute the three following elements.
    * $U = g_2^{\sum_{i=0}^l a_i ( \frac{u_i(\tau) - u_i(z)}{\tau - z})}$
    * $V = g_2^{\sum_{i=0}^l a_i ( \frac{v_i(\tau) - v_i(z)}{\tau - z})}$
    * $W = g_2^{\sum_{i=0}^l a_i ( \frac{w_i(\tau) - w_i(z)}{\tau - z})}$

* Proof is $(\pi, S, U , V , W)$

### Verifier
* Compute the following three elements $u_z, v_z, w_z \in \mathbb{F_r}$:
    * $u_z = \sum_{i=0}^l u_i(z)$
    * $v_z = \sum_{i=0}^l v_i(z)$
    * $w_z = \sum_{i=0}^l w_i(z)$
* Check **as usual**
    * $e(A,B) = e(g_1^\alpha, g_2^\beta)e(S,g2)e(C,g2^\delta)$
* Check public input construction
    * $e(Sg_1^{-\beta u_z -\alpha v_z -w_z},g_2) = e(g_1^{\beta \tau - \beta z}, U)e(g_1^{\alpha \tau - \alpha z},V)e(g_1^{\tau - z},W)$
    * These computations are done using the CRS

The CRS of Groth16 consists of very specific polynomials 𝑊(𝑥), 𝑈(𝑥), 𝑉(𝑥) that are equivalent to the R1CS constraints. Advantages: The prover 𝒫 can generate the proof using the polynomials 𝑊(𝑥), 𝑈(𝑥), 𝑉(𝑥) directly from the CRS, resulting in a fast verification. Disadvantages: The polynomials 𝑊(𝑥), 𝑈(𝑥), 𝑉(𝑥) in this CRS are derived from R1CS constraints and are fixed, which means that they can only express a unique circuit and their expressive power is relatively limited to other circuits.




## Practice1: Multilication





## Practice2: Merkle Tree
