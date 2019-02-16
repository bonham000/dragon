
use super::lesson_01;
use super::types::{Lesson};

pub fn get_content() -> Vec<Lesson> {
    vec![
        lesson_01::get_content()
    ]
}