use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature (pub String, pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub typeis: String,
    pub features: Vec<Feature>,
    pub value: String,
    pub subtree: Vec<Node>
}