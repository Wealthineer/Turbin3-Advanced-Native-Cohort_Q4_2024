use solana_program::{declare_id, entrypoint};

mod processor;
pub use processor::*;

mod instructions;
pub use instructions::*;

mod state;
pub use state::*;

declare_id!("11111111111111111111111111111111");

entrypoint!(process_instruction);
