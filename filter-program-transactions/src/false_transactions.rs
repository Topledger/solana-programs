use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "flash",
            vec![
                "FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn",
                "FSWAPViR8ny5K96hezav8jynVubP2dJ2L7SbKzds2hwm",
            ],
        ),
        (
            "pyth",
            vec![
                "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ",
                "pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT",
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
