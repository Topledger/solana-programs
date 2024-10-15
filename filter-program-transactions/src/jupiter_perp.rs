use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "jupiter-perp",
            vec![
                "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu"
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
