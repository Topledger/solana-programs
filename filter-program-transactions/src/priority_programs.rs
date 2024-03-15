use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "tensor",
            vec![
                "TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN",
                "TL1ST2iRBzuGTqLn1KXnGdSnEow62BzPnGiqyRXhWtW",
                "TB1Dqt8JeKQh7RLDzfYDJsq8KS4fS2yt87avRjyRxMv",
                "TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp",
                "TSTKEiz9sqJRypokAkRhaW29rnDYDSxqWxmdv9brkp2",
                "SWPhxKY7ponWjkfYCnvWypX8pEMe9hvQHhKo9wS7vim",
                "TRoLL7U1qTaqv2FFQ4jneZx5SetannKmrYCR778AkQZ",
                "TLoCKic2wGJm7VhZKumih4Lc35fUhYqVMgA4j389Buk",
                "TDRoPy8i5G8AMzuaGPb98fxDRevS81kfhVeaipyWGbN",
                "TGARDaEzs7px1tEUssCCZ9ewhTW7oCA1MnY5y7rQk9n",
                "TPA1R3GSAgUcZRcJXz5EU8Z7Y7w9XxoXz5fguY3anvM",
            ],
        ),
        (
            "zeta",
            vec![
                "ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD",
                "zDEXqXEG7gAyxb1Kg9mK5fPnUdENCGKzWrM21RMdWRq",
            ],
        ),
        (
            "solend",
            vec!["So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"],
        ),
        (
            "drift",
            vec![
                "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
                "DraWMeQX9LfzQQSYoeBwHAgM5JcqFkgrX7GbTfjzVMVL",
            ],
        ),
        (
            "marginfi",
            vec![
                "LipsxuAkFkwa4RKNzn51wAsW7Dedzt1RNHMkTkDEZUW",
                "MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA",
            ],
        ),
        (
            "wormhole",
            vec![
                "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth",
                "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb",
                "WnFt12ZrnzZrFZkt2xsNsaNWoQribnuQ5B5FrDbwDhD",
            ],
        ),
        (
            "mayan_swap",
            vec!["8LPjGDbxhW4G2Q8S6FvdvUdfGWssgtqmvsc63bwNFA7E"],
        ),
        (
            "debridge",
            vec![
                "dst5MGcFPoBeREFAA5E3tU5ij8m5uVYwkzkSAbsLbNo",
                "src5qyZHqTqecJV4aY6Cb6zDZLMDzrDKKezs22MPHr4",
                "DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh",
                "DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft",
            ],
        ),
        (
            "allbridge",
            vec!["BrdgN2RPzEMWF96ZbnnJaUtQDQx7VRXYaHHbYCBvceWB"],
        ),
        (
            "hubble",
            vec![
                "HubbLeXBb7qyLHt3x7gvYaRrxQmmgExb7fCJgDqFuB6T",
                "6LtLpnUFNByNXLyCoK9wA2MykKAmQNZKBdY8s47dehDc",
                "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD",
            ],
        ),
        (
            "sphere_pay",
            vec!["AYGdvqsQruZoaJPWsViLqUgtbfXGRnxzgxzW4zmbbckL"],
        ),
        (
            "helio",
            vec![
                "ENicYBBNZQ91toN7ggmTxnDGZW14uv9UkumN7XBGeYJ4",
                "Gfz4VD7NmjyxeQexzLtwqpxUVkXHGQ61BTD6XUB5j55x",
                "3KPRuKWxV6PtneZXbokMBwdF4T9brCFx7FcmKJ2tPqqt",
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
