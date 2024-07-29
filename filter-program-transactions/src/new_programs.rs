use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "staratlas",
            vec![
                "Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk", 
                "CRAFT2RPXPJWCEix4WpJST3E7NLf79GTqZUL75wngXo5",
                "Point2iBvz7j5TMVef8nEgpmz4pDr7tU7v3RjAfkQbM",
                "PsToRxhEPScGt1Bxpm7zNDRzaMk31t8Aox7fyewoVse",
                "SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE",
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
                "MPL4o4wMzndgh8T1NVDxELQCj5UQfYTYEkabX3wNKtb"
            ],
        ),
        (
            "bpf_program",
            vec![
                "BPFLoaderUpgradeab1e11111111111111111111111",
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

    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
