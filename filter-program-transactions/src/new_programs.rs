use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "meteora",
            vec![
                "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
                "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
                "FEESngU3neckdwib9X3KWqdL7Mjmqk9XNp3uh5JbP4KP",
                "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi",
                "FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1",
                "vaU6kP7iNEGkbmPkLmZfGwiGxd4Mob24QQCie5R9kd2",
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
