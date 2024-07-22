use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "switchboard",
            vec![
                "sbattyXrzedoNATfc4L31wC9Mhxsi1BmFhTiN8gDshx"
                ,"SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f",
                "SBondMDrcV3K4kxZR1HNVT7osZxAHVHgYXL5Ze1oMUv",
                "SBSTk6t52R89MmCdD739Rdd97HdbTQUFHe41vCX7pTt",

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
