use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "exponent",
            vec![
                "ExponentnaRg3CQbW6dqQNZKXp7gtZ9DGMp1cwC4HAS7",
                "XPK1ndTK1xrgRg99ifvdPP1exrx8D1mRXTuxBkkroCx",
                "XPMfipyhcbq3DBvgvxkbZY7GekwmGNJLMD3wdiCkBc7",
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
