use student_system;
use teacher_system;

fn main() {
    let teacher_id = 1;
    let student_id = 2;

    // let teacher_courses = get_teacher_courses(teacher_id);
    // let student_courses = get_enrolled_courses(student_id);

    let teacher_courses = teacher_id;
    let student_courses = student_id;

    println!("Cursos del profesor: {:?}", teacher_courses);
    println!("Cursos del estudiante: {:?}", student_courses);
}
