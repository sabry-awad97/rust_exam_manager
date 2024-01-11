# Student Exam System

This Rust project is a simple student exam system that allows users to input information about students, conduct exams in math and English, and view and analyze the results.

## Features

- Input details for multiple students, including their names, IDs, and exam scores.
- Conduct exams in both math and English.
- Calculate and display individual subject grades, total grades, and final Grade Point Average (GBA).
- Identify the student with the highest overall grade.
- Search for a student's result by their ID.

Here's a brief overview:

1. **Structs:**

   - `Grade`: Represents grades for a subject, storing both the total marks and the subject-specific marks.
   - `Student`: Represents student information, including name, ID, and grades for both math and English.

2. **Functions:**

   - `exam_english()`: Conducts the English exam, prompting the user for answers and calculating grades.
   - `exam_math()`: Conducts the math exam, prompting the user for answers and calculating grades.

3. **Main Function:**

   - Prompts the user to enter the number of students.
   - Utilizes a vector of `Student` structs to store information for each student.
   - Ensures unique IDs for each student using a `HashSet`.
   - Conducts math and English exams for each student and calculates total scores.
   - Determines the final grade (GBA) for each student based on their total marks.
   - Prints the details of each student, including exam grades and final GBA.
   - Identifies and prints the student with the highest GBA.
   - Allows the user to search for their results using their ID.

4. **Highlights:**

   - Utilizes Rust's ownership system for memory safety and prevention of common programming errors.
   - Implements functions for modular code organization.
   - Incorporates Rust's pattern matching and functional programming features for concise and readable code.

5. **Note:**

   - The program assumes valid user inputs and does minimal error handling for simplicity.
   - It calculates the GBA based on a predefined grade scale and prints the final results for each student.

## Usage

1. **Compile and Run:**

   - Ensure you have Rust installed on your system.
   - Clone this repository.
   - Navigate to the project directory in the terminal.
   - Run the project using `cargo run`.

2. **Input Student Data:**

   - Enter the number of students.
   - For each student, input their name, ID, and exam scores.

3. **View Results:**

   - Results for each student, including subject grades, total grades, and GBA, will be displayed.
   - The student with the highest grade is highlighted.

4. **Search for Results:**
   - Enter your ID to search for your result.

## Contributing

Contributions are welcome! If you have any suggestions, improvements, or bug fixes, feel free to open an issue or submit a pull request.
