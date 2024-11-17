## First Iteration of optimizations
- Swap out `solana_program` for `pinocchio`
- Implement own data struct without bytemuck
- Removing Borsh serialization/deserialization

### CU usage

- make: 
- take: 
- refund: 

## Baseline
See `escrow` program in this repo for baseline code this started with.

### CU usage
- make: 14773
- take: 24626
- refund: 15140

### Program size
- 124k