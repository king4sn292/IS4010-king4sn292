mod student;
use student::{CourseGrade, Grade, Student, StudentDatabase};

fn main() {
    println!("Week 11: Student management system");

    let mut db = StudentDatabase::new();

    let mut alice = Student::new(
        String::from("S001"),
        String::from("Alice Johnson"),
        String::from("alice@example.com"),
    );
    alice.add_grade(CourseGrade::new(
        String::from("IS4010"),
        String::from("App Dev with AI"),
        3,
        Grade::A,
    ));

    let mut bob = Student::new(
        String::from("S002"),
        String::from("Bob Smith"),
        String::from("bob@example.com"),
    );
    bob.add_grade(CourseGrade::new(
        String::from("IS3050"),
        String::from("Database Design"),
        3,
        Grade::B,
    ));

    db.add_student(alice).unwrap();
    db.add_student(bob).unwrap();

    println!("\nDatabase Statistics:");
    println!("Total students: {}", db.student_count());
    println!("Average GPA: {:.2}", db.average_gpa());

    if let Some(student) = db.find_student_mut("S002") {
        student.add_grade(CourseGrade::new(
            String::from("IS4000"),
            String::from("Advanced Systems"),
            3,
            Grade::A,
        ));
        println!("Updated record for {}.", student.name);
    }

    println!("\nAll Students:");
    for student in db.list_students() {
        println!(
            "  {} - {} (GPA: {:.2})",
            student.id,
            student.name,
            student.calculate_gpa(),
        );
    }

    if let Some(student) = db.find_student("S001") {
        println!("\nFound student: {}", student.name);
        println!("  Email: {}", student.email);
        println!("  Credits: {}", student.credits_earned);
        println!("  GPA: {:.2}", student.calculate_gpa());
        println!("  Transcript:");
        for course_grade in &student.grades {
            println!(
                "    {}: {} - {:?} ({:.1} quality points)",
                course_grade.course_code,
                course_grade.course_name,
                course_grade.grade,
                course_grade.quality_points(),
            );
        }
    }
}
