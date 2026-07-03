use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Teacher {
    name: String,
    students: Vec<Student>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Student {
    name: String,
    active: bool,
}