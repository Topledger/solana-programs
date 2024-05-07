use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "token22",
            vec![
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            ],
        ),
        (
            "allbridge",
            vec![
                "BrdgN2RPzEMWF96ZbnnJaUtQDQx7VRXYaHHbYCBvceWB",
                "2bbbTeKEBNK6xamKrY51byxrYiHksPo6vSkivw2zJFSQ",
                "BBbD1WSjbHKfyE3TSFWF6vx1JV51c8msKSQy4ess6pXp"
                ],
        ),
        (
            "circuit-trading",
            vec![
                "vAuLTsyrvSfZRuRB3XgvkPwNGgYSs9YRYymVebLKoxR",
                ],

        ),
        (
            "sanctum",
            vec![
                "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
                "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
                "5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx",
                "unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ"
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
