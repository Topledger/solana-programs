use std::collections::HashMap;

/// Generic program configuration that can be easily modified for different programs
pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "solana_native_staking",
            vec![
                "Stake11111111111111111111111111111111111111"
            ],
        )
        // Add more programs here as needed:
        // (
        //     "jupiter_perp",
        //     vec![
        //         "PERPHjuBqRHArX4DySjwM6UJHiqqXhU7zUyoQCBZP9cJ"
        //     ],
        // ),
    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
