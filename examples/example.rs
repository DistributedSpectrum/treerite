/*!
 * This is the rust version of treelite/tests/example_app/example.c
 * Copyright (c) 2023 by Contributors
 * \file example.c
 * \brief Test using Treelite as a C++ library
 */

mod support;
use support::build_model;

use ndarray::array;
use treerite::GTIConfig;


fn main() {
    let model = build_model();
    let input = array![
        [-2.0f64, 0.0f64],
        [-1.0f64, 0.0f64],
        [0.0f64, 0.0f64],
        [1.0f64, 0.0f64],
        [2.0f64, 0.0f64]
    ];

    let config = GTIConfig::parse(
        r#"{
          "predict_type": "default",
          "nthread": 2
        }"#,
    )
    .unwrap();

    let y = model.predict(&config, &input, None).unwrap();

    println!("{:?}", y);
}
