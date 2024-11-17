## First Iteration of optimizations
- Swap out `solana_program` for `pinocchio`
- Implement own data struct without bytemuck
- Removing Borsh serialization/deserialization

### CU usage
- make: 8100
- take: 16596
- refund: 10538

### Program size
- 48k

## Baseline
See  [`escrow`](../escrow) for baseline code this started with.

### CU usage
- make: 14773
- take: 24626
- refund: 15140

### Program size
- 124k