use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "staratlas",
            vec![
                "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg",
                "FLEET1qqzpexyaDpqb2DGsSzE2sDCizewCg9WjrA6DBW",
                "ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc",
                "STAKEr4Bh8sbBMoAVmTDBRqouPzgdocVrvtjmhJhd65",
                "FACTNmq2FhA2QNTnGM2aWJH3i7zT3cND5CgvjYTjyVYe",
                "TESTWCwvEv2idx6eZVQrFFdvEJqGHfVA1soApk2NFKQ",
                "gateVwTnKyFrE8nxUUgfzoZTPKgJQZUbLsEidpG4Dp2",
                "Lock7kBijGCQLEFAmXcengzXKA88iDNQPriQ7TbgeyG",
                "snapNQkxsiqDWdbNfz8KVB7e3NPzLwtHHA6WV8kKgUc",
                "SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6",
                "Craftf1EGzEoPFJ1rpaTSQG1F6hhRRBAf4gRo9hdSZjR",
                "Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH",
                "pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9",
                "pv1ttom8tbyh83C1AVh6QH2naGRdVQUVt3HY1Yst5sv",
                "pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq",
                "APR1MEny25pKupwn72oVqMH4qpDouArsX8zX4VwwfoXD",
            ],
        ),
        (
            "phoenix",
            vec!["PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY"],
        ),
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
            ],
        ),
        (
            "magiceden",
            vec![
                "M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K",
                "MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8",
                "mmm3XBJg5gk8XJxEKBvdgptZz6SgK4tXvn36sodowMc",
                "CMX5tvuWs2rBUL3vqVWiARfcDoCKjdeSinCsZdxJmYoF",
                "CMY8R8yghKfFnHKCWjzrArUpYH4PbJ56aWBr4kCP4DMk",
                "CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb",
                "ocp4vWUzA2z2XMYJ3QhM9vWdyoyoQwAFJhRdVTbvo9E",
                "M3mxk5W2tt27WGT7THox7PmgRDp4m6NEhL5xvxrBfS1",
            ],
        ),
        (
            "marinade",
            vec![
                "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD",
                "tokdh9ZbWPxkFzqsKqeAwLDk6J6a8NBZtQanVuuENxa",
                "tovt1VkTE2T4caWoeFP6a2xSFoew5mNpd7FWidyyMuk",
                "va12L6Z9fa5aGJ7gxtJuQZ928nySAk5UetjcGPve3Nu",
                "LigadctxNRkZied3WuhX525vUhDkuhXNK5DyeijeDnh",
                "MR2LqxoSbw831bNy68utpu5n4YqBH3AzDmddkgk9LQv",
                "dstK1PDHNoKN9MdmftRzsEbXP5T1FTBiQBm1Ee3meVd",
                "VoteMBhDCqGLRgYpp9o7DGyq81KNmwjXQRAHStjtJsS",
                "mnspJQyF1KdDEs5c6YJPocYdY1esBgVQFufM2dY9oDk",
            ],
        ),
        ("robox", vec!["rbxKHT1j2EnkfvVgANWFXdcV5Sm8EkdZLLDjJ5R1Lwa"]),
        (
            "sharky",
            vec!["SHARKobtfF1bHhxD2eqftjHBdVSCbKo9JtgK71FhELP"],
        ),
        (
            "light_protocol",
            vec!["2c54pLrGpQdGxJWUAoME6CReBrtDbsx5Tqx4nLZZo6av"],
        ),
        (
            "elusiv_privacy",
            vec!["4CgyHKuP6yi1vbmcdsArngEKcETR4nZupqYEMk2hEoQd"],
        ),
        (
            "alldomains",
            vec![
                "TLDHkysf5pCnKsVA4gXpNvmy7psXLPEu4LAdDJthT9S",
                "ALTNSZ46uaAUU7XUV6awvdorLGqAsPwa9shm7h4uP2FK",
                "TLDM2itbqSCEGnCBEF5ExqbePWQGJpkAW2t5noVqHu9",
                "NH3uX6FtVE2fNREAioP7hm5RaozotZxeL6khU1EHx51",
                "APooLSMjJV9kkqEWX2QvDyStkdPesVugMALsMvPLbsnx",
            ],
        ),
        (
            "native_stake",
            vec!["Stake11111111111111111111111111111111111111"],
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
