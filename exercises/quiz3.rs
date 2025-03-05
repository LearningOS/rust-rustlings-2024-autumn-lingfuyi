// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.
/*
Trait 多态：

通过 Grade trait 将不同类型（f32 和 String）的行为统一为 to_letter_grade 方法。
动态分发（Box<dyn Grade>）允许运行时根据具体类型调用方法。
代码复用与扩展性：

ReportCard 结构体无需关心成绩的具体类型，只需依赖 Grade trait 的约定。
未来新增成绩类型（如 GradeLevel 枚举）时，只需实现 Grade trait 即可无缝集成。
也可以引入Display trait 统一输出格式。
Display 是 Rust 的官方字符串格式化 trait，所有数值型（如 f32）和字符串类型（如 String）均已默认实现。直接使用可避免重复劳动。
*/

pub trait Grade {
    fn to_letter_grade(&self) -> String;
}
impl Grade for f32 {
    fn to_letter_grade(&self) -> String {
        self.to_string()
    }
}
impl Grade for String {
    fn to_letter_grade(&self) -> String {
        self.clone()
    }
}
pub struct ReportCard {
    pub grade: Box<dyn Grade>,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            &self.grade.to_letter_grade()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Box::new(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Box::new("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
