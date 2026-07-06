use serde::{Serialize, Deserialize};
// Storing relations between a teacher and their students.

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
