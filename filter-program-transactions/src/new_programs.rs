use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "skytrade",
            vec![
                "HAS8wvo7CPg7nmzbQoxJ4KDEqLyskxDG2p9HGHZ9ub77", 
                "HmqstutaEpbddgt5hjhJAsnrXhTUfQpveCpyWEvEdnWM",
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
