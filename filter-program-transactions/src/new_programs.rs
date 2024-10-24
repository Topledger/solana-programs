use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "solayer",
            vec![
                "sSo1iU21jBrU9VaJ8PJib1MtorefUV4fzC9GURa2KNn",
                "endoLNCKTqDn8gSVnN2hDdpgACUPWHZTwoYnnMybpAT",
            ],
        ),

    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
