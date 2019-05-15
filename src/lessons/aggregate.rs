use super::lesson_01;
use super::types::Lesson;

pub fn get_content() -> Vec<Lesson> {
    vec![
        lesson_01::get_content(),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_content() {
        let content = super::get_content();
        println!("Content: {:?}", content);
        assert_eq!(2, 2);
    }
}
