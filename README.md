# Recursive Zkp For Factorization
a demo for recursive zero-knowledge proof with underlying proof of integer multiplication and factorization
## Proof logic
implementing a ZKP for integer factorization: proving knowledge of two integers *p* and *q* such that *p × q = N*, without revealing *p* and *q*.

**Recursive zk-Proofs** take this a step further by allowing a proof to verify another proof within itself. This enables scalable and efficient proof systems, as multiple proofs can be aggregated into a single proof.

## Overview of Tools and Libraries
Utilize the following tools and libraries:

* Circom: A circuit compiler for zk-SNARKs, allowing you to define arithmetic circuits.
* SnarkJS: A JavaScript library for generating and verifying zk-SNARK proofs.
* ZoKrates: A toolbox for zkSNARKs on Ethereum, facilitating zk-proof generation.
* Noir: A domain-specific language for writing zero-knowledge proofs.
* Bellman: A Rust-based library for building zk-SNARK circuits.


* Circom + SnarkJS: Ideal for defining custom circuits and generating proofs with JavaScript.
* ZoKrates: Provides an end-to-end toolkit with a higher-level abstraction.
* Noir: Offers a Rust-like language for writing ZK circuits, focusing on usability.
* Bellman: Suitable for those comfortable with Rust and seeking performance.

## Reference
* Circom Documentation: [https://docs.circom.io/]
* SnarkJS Documentation: [https://github.com/iden3/snarkjs]
* ZoKrates Documentation: [https://zokrates.github.io/introduction.html]
* Noir Documentation: [https://noir-lang.org/docs]
* Bellman Repository: [https://github.com/zkcrypto/bellman]
