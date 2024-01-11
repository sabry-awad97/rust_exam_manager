use std::{collections::HashSet, io::Write};

// Struct to represent grades
struct Grade {
    total_marks: f32,
    subject_marks: f32,
}

// Struct to represent a student
struct Student {
    id: String,
    name: String,
    math: Grade,
    english: Grade,
}

const ENGLISH_QUESTIONS: [(&str, &str); 3] = [
    ("How old ------ you? ", "are"),
    ("shut ------ ? ", "up"),
    ("see ------ latter? ", "you"),
];

const MATH_QUESTIONS: [(&str, i32); 5] = [
    ("1 + 2 = ------ ", 3),
    ("5 + 2 = ------ ", 7),
    ("10 + 2 = ------ ", 12),
    ("5 - 2 = ------ ", 3),
    ("10 - 2 = ------ ", 8),
];

// Function to conduct the English exam
fn exam_english() -> Grade {
    let mut total = 0.0;
    let mut subject = 0.0;

    for (question, answer) in ENGLISH_QUESTIONS {
        print!("{}", question);
        std::io::stdout().flush().expect("Failed to flush output");
        let mut user_answer = String::new();
        std::io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");

        if user_answer.trim().eq_ignore_ascii_case(answer) {
            subject += 30.0;
        }
        total += 30.0;
    }

    Grade {
        total_marks: total,
        subject_marks: subject,
    }
}

// Function to conduct the Math exam
fn exam_math() -> Grade {
    let mut total = 0.0;
    let mut subject = 0.0;

    for (question, correct_answer) in MATH_QUESTIONS {
        print!("{}", question);
        std::io::stdout().flush().expect("Failed to flush output");
        let mut user_answer = String::new();
        std::io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");

        if let Ok(answer) = user_answer.trim().parse::<i32>() {
            if answer == correct_answer {
                subject += 20.0;
            }
        }
        total += 20.0;
    }

    Grade {
        total_marks: total,
        subject_marks: subject,
    }
}

fn main() {
    println!("Enter the number of students:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number_of_students: usize = input.trim().parse().expect("Invalid number");

    let mut students: Vec<Student> = Vec::with_capacity(number_of_students);
    let mut id_set: HashSet<String> = HashSet::new();

    // Input data for each student
    for i in 0..number_of_students {
        if number_of_students != 1 {
            println!("For student number: {}", i + 1);
        }

        println!("Enter the name of the student:");
        let mut name = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        println!("Enter the ID of the student (4 digits):");
        let mut id = String::new();
        loop {
            std::io::stdin()
                .read_line(&mut id)
                .expect("Failed to read line");

            if id_set.contains(&id.trim().to_string()) {
                println!("Repeated ID. Please enter a different ID.");
                id.clear();
            } else {
                break;
            }
        }
        id_set.insert(id.trim().to_string());

        let math = exam_math();
        let english = exam_english();

        students.push(Student {
            name,
            id,
            math,
            english,
        });
    }

    // Print results for each student
    for (i, student) in students.iter().enumerate() {
        println!("----------------------------------------------------");
        println!("Student {}", i + 1);
        println!("Name: {:<20}   ID: {}", student.name, student.id);
        print_subject_info("Math", &student.math);
        print_subject_info("English", &student.english);
        print_final_info(student);
        println!("----------------------------------------------------");
    }

    // Find and print the student with the highest grade
    if let Some(max_student) = students.iter().max_by(|a, b| {
        (a.math.subject_marks + a.english.subject_marks)
            .partial_cmp(&(b.math.subject_marks + b.english.subject_marks))
            .unwrap()
    }) {
        println!("===================================================");
        println!("=               The Best Student                =");
        print_final_info(max_student);
        println!("===================================================");
    }

    // Search for a student by ID
    println!("Enter your ID to search for your result:");
    let mut search_id = String::new();
    std::io::stdin()
        .read_line(&mut search_id)
        .expect("Failed to read line");

    if let Some(student) = students.iter().find(|s| s.id.trim() == search_id.trim()) {
        println!("----------------------------------------------------");
        println!("Search Result");
        print_final_info(student);
        println!("----------------------------------------------------");
    } else {
        println!("Student with ID {} not found.", search_id.trim());
    }
}

// Helper function to print subject information
fn print_subject_info(subject_name: &str, grade: &Grade) {
    println!(
        "{:<10}:  Grade = {:.2}   Total = {}",
        subject_name, grade.subject_marks, grade.total_marks
    );
}

// Helper function to print final information
fn print_final_info(student: &Student) {
    let total_grades = student.math.total_marks + student.english.total_marks;
    let subject_grades = student.math.subject_marks + student.english.subject_marks;
    println!(
        "Total Grade = {:<4}   Your Grade = {:<5}   Final GBA = {}",
        total_grades,
        subject_grades,
        match subject_grades {
            grade if grade < 0.5 * total_grades => 'F',
            grade if grade < 0.6 * total_grades => 'D',
            grade if grade < 0.7 * total_grades => 'C',
            grade if grade < 0.85 * total_grades => 'B',
            _ => 'A',
        }
    );
}
