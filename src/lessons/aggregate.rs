use super::list_01_02;
use super::list_03;
use super::types::Lesson;

pub fn get_content() -> Vec<Lesson> {
    vec![list_01_02::get_content(), list_03::get_content()]
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
