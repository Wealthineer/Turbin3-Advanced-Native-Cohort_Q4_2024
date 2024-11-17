use pinocchio::{entrypoint, pubkey::Pubkey};

mod processor;
pub use processor::*;

mod instructions;
pub use instructions::*;

mod state;
pub use state::*;

#[cfg(test)]
mod tests;

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);
