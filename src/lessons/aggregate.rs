use super::lesson_00;
use super::lesson_01;
use super::lesson_02;
use super::lesson_03;
use super::lesson_04;
use super::lesson_05;
use super::lesson_06;
use super::lesson_07;
use super::lesson_08;
use super::lesson_09;
use super::lesson_10;
use super::lesson_11;
use super::types::Lesson;

pub fn get_content() -> Vec<Lesson> {
    vec![
        lesson_00::get_content(),
        lesson_01::get_content(),
        lesson_02::get_content(),
        lesson_03::get_content(),
        lesson_04::get_content(),
        lesson_05::get_content(),
        lesson_06::get_content(),
        lesson_07::get_content(),
        lesson_08::get_content(),
        lesson_09::get_content(),
        lesson_10::get_content(),
        lesson_11::get_content(),
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
