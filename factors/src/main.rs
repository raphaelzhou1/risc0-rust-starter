// TODO: Update the name of the method loaded by the prover. E.g., if the method is `multiply`, replace `METHOD_NAME_ID` with `MULTIPLY_ID` and replace `METHOD_NAME_PATH` with `MULTIPLY_PATH`
use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let a: u64 = 17;
    let b: u64 = 23;

    // Make the prover.
    let mut prover = Prover::new(MULTIPLY_ELF, MULTIPLY_ID).unwrap();

    // TODO: Implement communication with the guest here

    prover.add_input_u32_slice(&to_vec(&a).unwrap());
    prover.add_input_u32_slice(&to_vec(&b).unwrap());

    // Run prover & generate receipt
    let receipt = prover.run().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(MULTIPLY_ID).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here


}
