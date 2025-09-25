use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
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
            "hivemapper",
            vec![
                "BNH1dUp3ExFbgo3YctSqQbJXRFn3ffkwbcmSas8azfaW"
              ,"6563bNZpM7bn63rJYzdqP7pN6ndqkDvzgSZXbHtaDgWn"
              ,"AFMLFi8Mh1yehDsE1gTQs5xcJcMN39cvzSorpU152HaE" 
              ,"EEjwuvCMVYjgHUeX1BM9qmUog59Pft88c3jbt2ATwcJw" 
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
