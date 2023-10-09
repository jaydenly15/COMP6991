use std::error::Error;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::Deserialize;

const ENROLMENTS_PATH: &str = "enrolments.psv";

#[derive(Deserialize, Debug)]
struct Student {
    subject: String,
    id: String, 
    name: String,
    dummy1: String, 
    dummy2: String, 
    wam: f64,
}
fn main() {
    let mut data = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(ENROLMENTS_PATH).expect("psv file cannot be read");

    // count number of students (rows)
    let mut students = HashSet::new();
    let mut subject_frequency = HashMap::new();
    let mut sum_wam = 0.0;
    for row in data.records() {
        let student: Student = row.unwrap().deserialize(None).unwrap();
        if students.insert(student.id) {
            sum_wam += student.wam;
        }
        *subject_frequency.entry(student.subject).or_insert(0) += 1;
    }
    let most_common = Some(subject_frequency
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap());
    let least_common = Some(subject_frequency
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap());
    let num_students = students.len();
    println!("Number of students: {}", num_students);
    if let Some(most_common) = most_common {
        println!("Most common course: {} with {} students", most_common.0, most_common.1);
    }
    if let Some(least_common) = least_common {
        println!("Least common course: {} with {} students", least_common.0, least_common.1);
    }
    println!("Average WAM: {:.2}", sum_wam / (num_students as f64));
   
}
