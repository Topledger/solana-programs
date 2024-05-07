use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "armada",
            vec![
                "ArmN3Av2boBg8pkkeCK9UuCN9zSUVc2UQg1qR2sKwm8d",
                "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB",
                "STAKEGztX7S1MUHxcQHieZhELCntb9Ys9BgUbeEtMu1",
                "LBCZU6Nogrx2oAAt3uiuzB4zzuYzLovMXPJ5RcZfJ8U",
                "exch6P2DC2UrU91PfbU72Ch6q1gRy5bDyyTJMqAQhKM"
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
