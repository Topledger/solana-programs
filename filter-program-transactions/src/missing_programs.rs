use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "drift",
            vec![
                "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
                "DraWMeQX9LfzQQSYoeBwHAgM5JcqFkgrX7GbTfjzVMVL",
            ],
        )
       
    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
