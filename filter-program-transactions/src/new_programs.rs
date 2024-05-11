use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "moonwalk",
            vec![
                "FitAFk15vtx2PBjfr7QTnefaHRx6HwajRiZMt1DdSSKU"
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

    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
