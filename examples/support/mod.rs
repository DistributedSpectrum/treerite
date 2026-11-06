use std::cmp::Ordering;
use treerite::{Builder, Model};

pub fn build_model() -> Model {
    let model_metadata = r#"{
              "threshold_type": "float32",
              "leaf_output_type": "float32",
              "metadata": {
                "num_feature": 2,
                "task_type": "kRegressor",
                "average_tree_output": false,
                "num_target": 1,
                "num_class": [1],
                "leaf_vector_shape": [1, 1]
              },
              "tree_annotation": {
                "num_tree": 1,
                "target_id": [0],
                "class_id": [0]
              },
              "postprocessor": {
                "name": "identity"
              },
              "base_scores": [0.0]
            }"#;
    let mut builder = Builder::new(model_metadata).unwrap();
    builder.start_tree().unwrap();
    builder.start_node(0).unwrap();
    builder
        .numerical_test(0, 0.0, 0, Ordering::Less, 1, 2)
        .unwrap();
    builder.end_node().unwrap();
    builder.start_node(1).unwrap();
    builder.leaf_scalar(-1.0).unwrap();
    builder.end_node().unwrap();
    builder.start_node(2).unwrap();
    builder.leaf_scalar(1.0).unwrap();
    builder.end_node().unwrap();
    builder.end_tree().unwrap();

    let model = builder.commit_model().unwrap();
    model
}

