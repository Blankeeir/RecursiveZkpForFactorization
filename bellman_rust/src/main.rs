// src/main.rs
use bellman::{
    gadgets::multipack,
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use pairing::bn256::{Bn256, Fr};
use pairing::Engine;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

mod verifier_circuit;

#[derive(Clone)]
struct FactorizationCircuit {
    // Public input: N
    n: Option<Fr>,
    // Private inputs: p and q
    p: Option<Fr>,
    q: Option<Fr>,
}

impl<E: Engine> Circuit<E> for FactorizationCircuit {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate public input: N
        let n_var = cs.alloc_input(
            || "N",
            || self.n.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Allocate private input: p
        let p_var = cs.alloc(
            || "p",
            || self.p.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Allocate private input: q
        let q_var = cs.alloc(
            || "q",
            || self.q.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Enforce p * q = N
        cs.enforce(
            || "p * q = N",
            |lc| lc + p_var,
            |lc| lc + q_var,
            |lc| lc + n_var,
        );

        Ok(())
    }
}


#[derive(Serialize, Deserialize)]
struct ProofJson {
    pi_a: [String; 2],
    pi_b: [[String; 2]; 2],
    pi_c: [String; 2],
}

#[derive(Serialize, Deserialize)]
struct PublicJson {
    N: String,
}

fn main() {

    let rng = &mut OsRng;

    // TODO: Example values: : should change later according to user input
    let p = 17u64;
    let q = 23u64;
    let n = p * q;


    // Convert to Fr
    let p_fr = Fr::from_str(&p.to_string()).expect("Invalid Fr");
    let q_fr = Fr::from_str(&q.to_string()).expect("Invalid Fr");
    let n_fr = Fr::from_str(&n.to_string()).expect("Invalid Fr");

    let circuit = FactorizationCircuit {
        n: Some(n_fr),
        p: Some(p_fr),
        q: Some(q_fr),
    };

    println!("Generating parameters for Factorization Circuit...");

    // create random parameters for the circuit
    let params = {
        let empty_circuit = FactorizationCircuit {
            n: None,
            p: None,
            q: None,
            // set them to None first
        };

        generate_random_parameters::<Bn256, _, _>(empty_circuit, rng).expect("Parameter generation failed")
    };

    let pvk = prepare_verifying_key(&params.vk);


    println!("Creating proof for Factorization Circuit...");
    let proof = create_random_proof(circuit, &params, rng).expect("Proof generation failed");


    let proof_json = ProofJson {
        pi_a: [
            format!("{:?}", proof.pi_a.0),
            format!("{:?}", proof.pi_a.1),
        ],
        pi_b: [
            [
                format!("{:?}", proof.pi_b.0 .0),
                format!("{:?}", proof.pi_b.0 .1),
            ],
            [
                format!("{:?}", proof.pi_b.1 .0),
                format!("{:?}", proof.pi_b.1 .1),
            ],
        ],
        pi_c: [
            format!("{:?}", proof.pi_c.0),
            format!("{:?}", proof.pi_c.1),
        ],
    };

    let public_json = PublicJson {
        N: format!("{:?}", n_fr),
    };

    // Save proof.json
    let proof_file = File::create("proof.json").expect("Unable to create proof.json");
    serde_json::to_writer_pretty(pretty_print_writer(proof_file), &proof_json).expect("Unable to write proof");

    // Save public.json
    let public_file = File::create("public.json").expect("Unable to create public.json");
    serde_json::to_writer_pretty(pretty_print_writer(public_file), &public_json).expect("Unable to write public inputs");

    println!("Proof and public inputs saved to proof.json and public.json");

    // Verify the proof
    println!("Verifying proof...");
    let n_fr_verify = Fr::from_str(&n.to_string()).expect("Invalid Fr for verification");
    let is_valid = verify_proof(&pvk, &proof, &[n_fr_verify]).expect("Verification failed");

    println!("Proof is valid: {}", is_valid);

    // Now, create a recursive proof that verifies the above proof
    // For demonstration, we'll use the VerifierCircuit which always returns true
    // In a real scenario, you would implement the Groth16 verification within the circuit

    // Create the VerifierCircuit instance
    let verifier_circuit_instance = verifier_circuit::VerifierCircuit::<Bn256> {
        proof: Some(proof.clone()),
        public_input: Some(n_fr_verify),
    };

    // Generate parameters for VerifierCircuit
    println!("Generating parameters for Verifier Circuit...");
    let verifier_params = {
        let empty_verifier_circuit = verifier_circuit::VerifierCircuit::<Bn256> {
            proof: None,
            public_input: None,
        };
        generate_random_parameters::<Bn256, _, _>(empty_verifier_circuit, rng).expect("Verifier parameter generation failed")
    };

    // Prepare the verifier's verification key
    let verifier_pvk = prepare_verifying_key(&verifier_params.vk);

    // Create the recursive proof
    println!("Creating recursive proof...");
    let recursive_proof = create_random_proof(verifier_circuit_instance, &verifier_params, rng).expect("Recursive proof generation failed");

    // Serialize recursive proof to JSON
    let recursive_proof_json = ProofJson {
        pi_a: [
            format!("{:?}", recursive_proof.pi_a.0),
            format!("{:?}", recursive_proof.pi_a.1),
        ],
        pi_b: [
            [
                format!("{:?}", recursive_proof.pi_b.0 .0),
                format!("{:?}", recursive_proof.pi_b.0 .1),
            ],
            [
                format!("{:?}", recursive_proof.pi_b.1 .0),
                format!("{:?}", recursive_proof.pi_b.1 .1),
            ],
        ],
        pi_c: [
            format!("{:?}", recursive_proof.pi_c.0),
            format!("{:?}", recursive_proof.pi_c.1),
        ],
    };

    let recursive_public_json = PublicJson {
        N: format!("{:?}", Fr::one()), // As per the placeholder constraint in VerifierCircuit
    };

    // Save recursive_proof.json
    let recursive_proof_file = File::create("recursive_proof.json").expect("Unable to create recursive_proof.json");
    serde_json::to_writer_pretty(pretty_print_writer(recursive_proof_file), &recursive_proof_json).expect("Unable to write recursive proof");

    // Save recursive_public.json
    let recursive_public_file = File::create("recursive_public.json").expect("Unable to create recursive_public.json");
    serde_json::to_writer_pretty(pretty_print_writer(recursive_public_file), &recursive_public_json).expect("Unable to write recursive public inputs");

    println!("Recursive proof and public inputs saved to recursive_proof.json and recursive_public.json");

    // Verify the recursive proof
    println!("Verifying recursive proof...");
    let is_recursive_valid = verify_proof(&verifier_pvk, &recursive_proof, &[Fr::one()]).expect("Recursive verification failed");

    println!("Recursive proof is valid: {}", is_recursive_valid);
}

// Helper function to pretty print JSON
fn pretty_print_writer(file: File) -> impl Write {
    serde_json::to_writer_pretty(file)
}
