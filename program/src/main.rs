//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use reth_primitives::{Signature, B256};

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let signature = sp1_zkvm::io::read::<Signature>();

    signature.recover_signer_unchecked(B256::random()).unwrap();
}
