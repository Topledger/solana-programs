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
            "native_stake",
            vec!["Stake11111111111111111111111111111111111111"],
        ),
        (
            "openbook",
            vec![
                "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
                "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb",
            ],
        ),
        ("saber", vec!["SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ"]),
        (
            "stake_pool",
            vec!["SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy"],
        ),
        (
            "spl_governance",
            vec!["GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw"],
        ),
        (
            "lifinity",
            vec![
                "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S",
                "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c",
            ],
        ),
        
        (
            "fluxbeam",
            vec!["FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X"],
        ),
        (
            "parcl",
            vec!["3parcLrT7WnXAcyPfkCz49oofuuf2guUKkjuFkAhZW8Y"],
        ),
        (
            "ionet",
            vec!["AYGdvqsQruZoaJPWsViLqUgtbfXGRnxzgxzW4zmbbckL"],
        ),
        (
            "whales_market",
            vec!["stPdYNaJNsV3ytS9Xtx4GXXXRcVqVS6x66ZFa26K39S"],
        ),
        (
            "access_protocol",
            vec!["6HW8dXjtiTGkD4jzXs7igdFmZExPpmwUrRN5195xGup"],
        ),
        (
            "dispatch_unknown",
            vec!["GLsSp8Dr9EAe5UL67XmmjA3c8qqwYgNeD63pLZEcVGCw"],
        ),
        (
            "raindrops",
            vec![
                "itEm2FtqJWqUmMTYrUxoFcmFtBxEpd68VTmxJamQXA3",
                "AvAtARWmYZLbUFfoQc3RzT7zR5zLRs92VSMm8CsCadYN",
            ],
        ),
        (
            "underdog",
            vec!["updg8JyjrmFE2h3d71p71zRXDR8q4C6Up8dDoeq3LTM"],
        ),
        (
            "xnft_backpack",
            vec!["xnft5aaToUM4UFETUQfj7NUDUBdvYHTVhNFThEYTm55"],
        ),
        (
            "sgraph_protocol",
            vec!["graph8zS8zjLVJHdiSvP7S9PP7hNJpnHdbnJLR81FMg"],
        ),
        (
            "baze_one",
            vec!["Human1nfyFpJsPU3BBKqWPwD9FeaZgdPYzDVrBj32Xj"],
        ),
        (
            "code_canvas",
            vec!["CoCaSGpuNso2yQP3oqi1tXt82wBp3y78SJDwLCboc8WS"],
        ),
        (
            "nina_protocol",
            vec!["ninaN2tm9vUkxoanvGcNApEeWiidLMM2TdBX8HoJuL4"],
        ),
        (
            "mallow",
            vec![
                "MAUsg1KhgYQV2Kxr9ccAkv7bUod88Qi3AKe5nUN41oe",
                "MMA7VebX8Pi5JrrvaTBBm7nW81sfCww7ZtLBBT1YCy8"
                ],
        ),
        (
            "five_dollar_nft",
            vec!["7WUKntragGhmc2gGt7q6pAhg4txu9vLmfMi28fAcnUf9"],
        ),
        (
            "only_one",
            vec![
                "mkt59DL1S91CcGPNhgPT2pq7fR26LcbXBZtdDFQhYQX",
                "pssZn3LTGJpknPtSDiqLXZq98VwfYVicy6zDfPu82Cs",
            ],
        ),
        (
            "hub_three",
            vec!["2pi53pUUC5S4zyUU6Wrbe6EfYXS9LNcpikpwPFahtQQw"],
        ),
        (
            "vault_music",
            vec!["Drop9NhG6Mm6oQmPnzr8FQP6rWaPiwFkin59SeAKApLc"],
        ),
        (
            "bonfida",
            vec![
                "jCebN34bUfdeUYJT13J1yG16XWQpt5PDx6Mse9GUqhR",
                "nftD3vbNkNqfj2Sd3HZwbpw4BxxKWr4AjGb9X38JeZk",
                "AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT",
                "85iDfUvr3HJyLM2zcq5BXSiDvUWfw6cSE1FfNBo8Ap29",
            ],
        ),
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
        (
            "mango_markets",
            vec![
                "4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg",
                "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68",
                "GqTPL6qRf5aUuqscLh8Rg2HTxPUXfhhAXDptTLhp1t2J",
                "JD3bq9hGdy38PuWQ4h2YJpELmHVGPPfFSuFkpzAd9zfu",
                "5fNfvyp5czQVX77yoACa3JJVEhdRaWjPuazuWgjhTqEH",
            ],
        ),
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
                "unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ",
                "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq"
            ],
        ),
        (
            "moonwalk",
            vec![
                "FitAFk15vtx2PBjfr7QTnefaHRx6HwajRiZMt1DdSSKU",
                "FitAF2RmjKDiTnmhChJfiSo3fNjk7apRHz5FBdpenGQ1"
            ],
        ),
        (
            "nosana",
            vec![
                "nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM",
                "nosRB8DUV67oLNrL45bo2pFLrmsWPiewe2Lk2DRNYCp",
                "nosScmHY2uR24Zh751PmGj9ww9QRNHewh9H59AfrTJE",
                "nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD"
                ],
        ),
        (
            "bpf_program",
            vec![
                "BPFLoaderUpgradeab1e11111111111111111111111",
                ],

        ),
        (
            "pump",
            vec![
                "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
            ],
        ),
        (
            "Honeycomb",
            vec![
                "HivezrprVqHR6APKKQkkLHmUG8waZorXexEBRZWh5LRm"
                ,"ChRCtrG7X5kb9YncA4wuyD68DXXL8Szt3zBCCGiioBTg"
                ,"CrncyaGmZfWvpxRcpHEkSrqeeyQsdn4MAedo9KuARAc4"
                ,"Pay9ZxrVRXjt9Da8qpwqq4yBRvvrfx3STWnKK4FstPr"
                ,"Assetw8uxLogzVXic5P8wGYpVdesS1oZHfSnBFHAu42s"
                ,"HuntaX1CmUt5EByyFPE8pMf13SpvezybmMTtjmpmGmfj"
                ,"MiNESdRXUSmWY7NkAKdW9nMkjJZCaucguY3MDvkSmr6"
            ],
        ),
        (
            "cctp",
            vec![
                "CCTPmbSD7gX1bxKPAmg77w8oFzNFpaQiQUWD43TKaecd"
                ,"CCTPiPYPc6AsJuwueEnWgSgucamXDZwBd53dQ11YiKX3"
                ],
        ),
        (
            "moonwalk",
            vec![
                "FitAFk15vtx2PBjfr7QTnefaHRx6HwajRiZMt1DdSSKU",
                "FitAF2RmjKDiTnmhChJfiSo3fNjk7apRHz5FBdpenGQ1"
            ],
        ),
        (
            "switchboard",
            vec![
                "sbattyXrzedoNATfc4L31wC9Mhxsi1BmFhTiN8gDshx"
                ,"SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f"

            ],
        ),
        (
            "hivemapper",
            vec![
                "BNH1dUp3ExFbgo3YctSqQbJXRFn3ffkwbcmSas8azfaW"
              ,"6563bNZpM7bn63rJYzdqP7pN6ndqkDvzgSZXbHtaDgWn"
              ,"AFMLFi8Mh1yehDsE1gTQs5xcJcMN39cvzSorpU152HaE" 
              ,"EEjwuvCMVYjgHUeX1BM9qmUog59Pft88c3jbt2ATwcJw" 
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
