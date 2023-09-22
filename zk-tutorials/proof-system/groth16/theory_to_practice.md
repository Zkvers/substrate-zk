# Groth16: Theory to Practice
## Theory
The [Groth16](https://eprint.iacr.org/2016/260.pdf) algorithm is a non-interactive proof system used for zero-knowledge proofs, proposed by Jens Groth in 2016. Groth16 is one of the most widely used zero-knowledge proof systems and is built on previous work, primarily referencing protocols such as [Pinocchio](https://eprint.iacr.org/2013/279.pdf), [GGPR13](https://eprint.iacr.org/2012/215.pdf), and [BCTV14](https://eprint.iacr.org/2013/879.pdf).

Groth16 solves the problem of how to perform zero-knowledge proofs for complex computations. In zero-knowledge proof, the prover wishes to prove a statement about private inputs to the verifier, without revealing any information about the inputs themselves. This input is usually referred to as witness in ZkSnark. It is a solution satisfying certain conditions. The Groth16 proof system allows the prover to create a proof that they know a solution without revealing the actual solution to the verifier. As we previously introduced the basic concept of ZKSNARK(zero-knowledge succinct non-interactive argument of knowledge), it is a specific algorithm implementation of ZKSNARK.

### Features:
- **Non-interactive**: Groth16 is a non-interactive proof system, meaning that interaction is not needed between the prover and the verifier.
- **Short proofs**: Groth16 proofs are short, with a constant size of a few hundred bytes, which remains independent of the complexity of the proof.
- **Fast verification**: Groth16 proofs can be verified efficiently in polynomial time.
- **Trusted setup**: Groth16 requires a trusted setup phase for generating common parameters.
- **Applicable to any NP problem**: Groth16 proof system can construct zero-knowledge proofs for any NP problem.

## Protocol flow
Groth16 protocol is based on bilinear mapping and elliptic curve cryptography. The protocol consists of four phases: setup, proving, verification, and an optional preprocessing phase.

* **setup**, a set of public parameters is generated, including generators on an elliptic curve and some random numbers. Normally, a trusted third party is required to generate these parameters. However, it is considered insecure since the random numbers used to generate the CRS can be easily leaked. In this case, the attacker who gains access to these random numbers can forge fraudulent proofs. In practice, to avoid this issue, multiparty computation (MPC) can be used to generate this parameters in a more decentralized way, such as the [Powers of Tau ceremony](https://eprint.iacr.org/2022/1592.pdf).

* **proving**, the prover uses the public parameters and private input to generate the proof. The proof will contain some commitments about the computation and points on the elliptic curve to prove that the prover knows a solution that satisfies certain conditions while maintaining the privacy of the solution.

* **verification**, the verifier uses the public parameters and the proof provided by the prover to verify the truthfulness of the statement,i.e. the prover knows a solution that satisfies certain conditions, without knowing the prover's private input.

* **preprocessing**（optional）, some computations related to the public parameters can be preprocessed to improve verification efficiency and speed up the process, especially for large computations and multiple proof verification scenarios.

The core of the Groth16 protocol is to use bilinear mapping to transform the problem from the elliptic curve group to a scalar problem. In this way, the prover can provide a zero-knowledge proof about the computation, and the verifier only needs to perform simple bilinear mapping calculations to verify the correctness of the proof.

## Principle of the Protocol
> This part will involve some theoretical mathematical calculations and formulas. We hope you can patiently read through them and also present to you in a relatively concise form. Of course, if you find it difficult to continue, you can directly skip to the practical part below. After completing all the practices, you can come back to better understand the theoretical part.

Let's review the construction steps of zk-SNARKs as introduced in the ZKSANRKS [preview article](https://github.com/Zkvers/substrate-zk/blob/master/zk-tutorials/ZKSNARKS.md). Here, we will take a quick review of these steps. A zk-SNARKs protocol framework consists of the following steps:

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
$$(\beta, \delta, 1, \tau, \tau^2, \tau^3, \dots, \tau^{n-1})_{2}$$


Here G1，G2 are two groups in elliptic curve bilinear pairing and values in the parentheses are actually group elements scalar multiplied by the group generator and subscripts of these values indicate which group they are from.

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

* $B2$ is a point in G2:

$B2 = \beta_2 + w_0B_0(\tau)_2 + w_1B_1(\tau)_2 + w_2B_2(\tau)_2 + w_3B_3(\tau)_2 + \dots + w_mB_m(\tau)_2 + s\delta_2$


* $C1$ is a point in G1:

$C1 = w_{l+1}\cdot(L_{l+1}(\tau)/\delta)_1 + \dots + w_m\cdot(L_m(\tau)/\delta)_1 + H(\tau)\cdot(Z(\tau)/\delta)_1 + s\cdot A_1 + r\cdot B_1 - r\cdot s\cdot \delta_1$

$H(\tau)\cdot Z(\tau)/\delta \cdot g_1$ can be calculated from the coefficients of $H(x)$ by multiplying each coefficient with the corresponding term $(Z(\tau)/\delta) \cdot g_1, (\tau\cdot Z(\tau)/\delta) \cdot g_1,(\tau^2\cdot Z(\tau)/\delta) \cdot g_1, \ldots$, which are points made available by the CRS (common reference string).

The quotient polynomial $H(x)$ can be calculated by applying vector dot product operations between witness vector $w$ and polynomial vectors $A(x)$, $B(x)$, and $C(x)$. Then we have $H(x) = [w\cdot A(x) \cdot w\cdot B(x) - w\cdot C(x)] / Z(x)$.


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


As you can see,  the terms followed after $A(\tau) \cdot B(\tau)$ in $A \cdot B$ are identical to terms followed after  $C(\tau) + H(\tau) \cdot Z(\tau)$ in the right side of the equation. Thereby, in this equation, they can be canceled out. If the above equality holds, it means that:

$A(\tau) \cdot B(\tau) = C(\tau) + H(\tau) \cdot Z(\tau)$

Ok, verification succeeds. The prover does have a valid witness $w$.