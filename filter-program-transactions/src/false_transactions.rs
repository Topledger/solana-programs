use std::collections::HashMap;

pub fn create_programs_map() -> HashMap<&'static str, &'static str> {
    let program_data = vec![
        (
            "flash",
            vec![
                "FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn",
                "FSWAPViR8ny5K96hezav8jynVubP2dJ2L7SbKzds2hwm",
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
       
    ];

    let mut program_map: HashMap<&str, &str> = HashMap::new();

    for (program_name, program_ids) in program_data {
        for program_id in program_ids {
            program_map.insert(program_id, program_name);
        }
    }

    program_map
}
