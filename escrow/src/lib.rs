use solana_program::{declare_id, entrypoint};

mod processor;
pub use processor::*;

mod instructions;
pub use instructions::*;

mod state;
pub use state::*;

mod tests;

declare_id!("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);
