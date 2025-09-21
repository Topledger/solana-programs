# Generic Program Transaction Filter

This is a generic Substreams package for filtering Solana program transactions. It's designed to be easily configurable for any Solana program.

## Current Configuration
- **Program**: Solana Native Staking
- **Program ID**: `Stake11111111111111111111111111111111111111`

## How to Change Programs

To filter transactions for a different program, simply modify `src/program_config.rs`:

```rust
pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "your_program_name",
            vec![
                "YourProgramID1111111111111111111111111111"
            ],
        )
        // Add multiple programs if needed
    ];
    // ... rest of the function
}
```

## Building and Packaging

1. **Build**: `make build`
2. **Package**: `make package`
3. **Stream**: `make stream`

## Package Output

The package will be generated as `solana_native_staking_v1_0_0.spkg` (or update the name in `substreams.yaml` for different programs).

## Easy Customization

- Change program IDs in `src/program_config.rs`
- Update package name/version in `substreams.yaml`
- Rebuild with `make package`
