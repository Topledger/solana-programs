use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "drift",
            vec![
                "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
                "DraWMeQX9LfzQQSYoeBwHAgM5JcqFkgrX7GbTfjzVMVL",
            ],
        ),
        (
            "exchange_art",
            vec![
                "exAuvFHqXXbiLrM4ce9m1icwuSyXytRnfBkajukDFuB",
                "exofLDXJoFji4Qyf9jSAH59J4pp82UT5pmGgR6iT24Z",
                "AmK5g2XcyptVLCFESBCJqoSfwV3znGoVYQnqEnaAZKWn",
                "EXBuYPNgBUXMTsjCbezENRUtFQzjUNZxvPGTd11Pznk5",
            ],
        ),
        (
            "flash",
            vec![
                "FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn",
                "FSWAPViR8ny5K96hezav8jynVubP2dJ2L7SbKzds2hwm",
            ],
        ),
        (
            "libreplex",
            vec![
                "LibrQsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p",
                "rndbQFmFfiQ4tTG9QPhfJeq1J9fLCSBMAPj3EbLQ7Co",
                "inscokhJarcjaEs59QbQ7hYjrKz25LEPRfCbP8EmdUp",
                "78deTr7qycJ6498vSd3pNMhdCKKWxMipniitVHQcM8RM",
                "ListjawGEdhxuAErSyYwcTEGWQswFoi6FScnGG1RKSB",
                "G9whLiLT9nSkxwWzWvbiKKrTL6yWxvzh2UXqNht5VXqV",
                "9SXDHUdtfvBGT3H2uPCNEkxmWREoqdeS1qdBudLDD6KX",
                "Leg1xVbrpq5gY6mprak3Ud4q4mBwcJi5C9ZruYjWv7n",
            ],
        ),
        (
            "metaplex",
            vec![
                "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
                "hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk",
                "cndyAnrLdpjq1Ssp1z8xxDsB8dxe7u4HL5Nxi2K5WXZ",
                "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ",
                "CndyV3LdqHUfDLmE5naZjVN8rBZz4tqhdefbAnjHG3JR",
                "Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g",
                "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY",
                "hyDQ4Nz1eYyegS6JfenyKwKzYxRsCWCriYSAjtzP4Vg",
                "trifMWutwBxkSuatmpPVnEe7NoE3BJKgjVi8sSyoXWX",
                "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
                "MPL4o4wMzndgh8T1NVDxELQCj5UQfYTYEkabX3wNKtb",
                "CMACYFENjoBMHzapRXyo1JZkVS6EtaDDzkjMrmQLvr4J"
            ],
        ),
        (
            "orca",
            vec![
                "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
                "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP",
                "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1",
            ],
        ),
        (
            "squads",
            vec![
                "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf",
                "SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu",
                "SMPL5bz5ERMdweouWrXtk3jmb6FnjZkWf7pHDsE6Zwz",
                "SMPLbiNbsa19gf9jz8x9uHSvSn9VLFJB38dGy46kqJ7",
                "SMPLKTQhrgo22hFCVq2VGX1KAktTWjeizkhrdB1eauK",
                "SMPLVC8MxZ5Bf5EfF7PaMiTCxoBAcmkbM2vkrvMK8ho",
            ],
        ),
        (
            "armada",
            vec![
                "ArmN3Av2boBg8pkkeCK9UuCN9zSUVc2UQg1qR2sKwm8d",
                "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB",
                "STAKEGztX7S1MUHxcQHieZhELCntb9Ys9BgUbeEtMu1",
                "LBCZU6Nogrx2oAAt3uiuzB4zzuYzLovMXPJ5RcZfJ8U",
                "exch6P2DC2UrU91PfbU72Ch6q1gRy5bDyyTJMqAQhKM",
            ],
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
            "meteora",
            vec![
                "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
                "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi",
                "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
            ],
        ),
        (
            "helium",
            vec![
                "circAbx64bbsscPbQzZAUvuXpHqrCe6fLMzc2uKXz9g",
                "credMBJhYFzfn7NxBMdU4aUqFggAjgztaCcv2Fo6fPT",
                "hemjuPXBpNvggtaUnN1MwT3wrdhttKEfosTcc2P9Pg8",
                "hdaoVTCqhfHHo75XdAMxBKdUqvq1i5bF23sisBqVgGR",
                "1azyuavdMyvsivtNxPoz6SucD18eDHeXzFCUPq5XU7w",
                "1atrmQs3eq1N2FEYWu6tyTXbCjP4uQwExpjtnhXtS8h",
                "treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5",
                "hvsrNC3NKbcryqDs2DocYHZ9yPKEVzdSjQG6RVtK1s8",
                "iotEVVZLEywoTn1QdwNPddxPWszn3zFhEot3MfL9fns",
                "mb1eu7TzEc71KxDpsmsKoucSSuuoGLv1drys1oP2jh6",
            ],
        ),
        (
            "frakt",
            vec![
                "A66HabVL3DzNzeJgcHYtRRNW1ZRMKwBfrdSR4kLsZ9DJ",
                "regNrR9XpXkg6VCZXEyTwCGVETwKpZMtQxYx3zResJh",
                "raffyhEJMx59iDx778Y9gdaVcP2XDeusPcPBxwr3qAH",
                "4tdmkuY6EStxbS6Y8s5ueznL3VPMSugrvQuDeAHGZhSt",
            ],
        ),
        (
            "hadeswap",
            vec!["hadeK9DLv9eA7ya5KCTqSvSvRZeJC3JgD5a9Y3CNbvu"],
        ),
        (
            "psyoptions",
            vec![
                "R2y9ip6mxmWUj4pt54jP2hz2dgvMozy9VTSwMWE7evs",
                "FASQhaZQT53W9eT9wWnPoBFw8xzZDey9TbMmJj6jCQTs",
                "PSYFiYqguvMXwpDooGdYV6mju92YEbFobbvW617VNcq",
                "pSystkitWgLkzprdAvraP8DSBiXwee715wiSXGJe8yr"

            ],
        ),
        (
            "pyth",
            vec![
                "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ",
                "pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT",
            ],

        ),
        (
            "exponent",
            vec![
                "ExponentnaRg3CQbW6dqQNZKXp7gtZ9DGMp1cwC4HAS7",
                "XPK1ndTK1xrgRg99ifvdPP1exrx8D1mRXTuxBkkroCx",
                "XPMfipyhcbq3DBvgvxkbZY7GekwmGNJLMD3wdiCkBc7",
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
