use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "jupiter",
            vec![
                "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
                "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow",
                "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8",
                "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB",
                "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph",
                "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo",
                "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk",
                "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu",
                "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu",
                "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M", 
                "j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X",
                "VALaaymxQh2mNy2trH9jUqHT1mTow76wpTcGmSWSwJe",
                "LocpQgucEQHbqNABEYvBvwoxCPsSbG91A1QaQhQQqjn",
                "61DFfeTKM7trxYcPQCM78bJ794ddZprZpAwAnLiwTpYH"
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
