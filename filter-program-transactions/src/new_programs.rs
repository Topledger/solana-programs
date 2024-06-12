use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "switchboard",
            vec![
                "sbattyXrzedoNATfc4L31wC9Mhxsi1BmFhTiN8gDshx"
                ,"SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f"

            ],
        ),
        (
            "candy_guard_fork",
            vec![
                "TFCMaAnX3cbvHVQUsnJmUah49vJJ5gABbiV43qfGmsd"
                ,"TFCGs5oBAZmVNTsGWPMquE8uqXUtZt6zRY1LBxQs35G"

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
