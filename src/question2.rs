// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。

struct Student {
    name: String,
    age: u8,
    score: f32,
}

impl Student {
    fn new(name: &str, age: u8, score: f32) -> Student {
        Student {
            name: String::from(name),
            age,
            score,
        }
    }

    fn show(&self) {
        println!("Name: {}, Age: {}, Score: {}", self.name, self.age, self.score);
    }

    fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question2() {
        let student = Student::new("Alice", 18, 95.5);
        student.show();
        assert_eq!(student.is_passed(), true);

        let student2 = Student::new("Bob", 20, 55.0);
        student2.show();
        assert_eq!(student2.is_passed(), false);
    }

    #[test]
    fn test_student_boundary_score() {
        let student = Student::new("Charlie", 19, 60.0);
        assert_eq!(student.is_passed(), true);

        let student2 = Student::new("Diana", 21, 59.9);
        assert_eq!(student2.is_passed(), false);
    }

    #[test]
    fn test_student_empty_name() {
        let student = Student::new("", 18, 75.0);
        assert_eq!(student.name, "");
        assert_eq!(student.is_passed(), true);
    }

    #[test]
    fn test_student_zero_age() {
        let student = Student::new("Eve", 0, 85.0);
        assert_eq!(student.age, 0);
        assert_eq!(student.is_passed(), true);
    }

    #[test]
    fn test_student_negative_score() {
        let student = Student::new("Frank", 22, -10.0);
        assert_eq!(student.is_passed(), false);
    }
}