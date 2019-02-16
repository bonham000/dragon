
#[derive(Debug)]
pub struct Item {
    pub characters: &'static str,
    pub phonetic: &'static str,
    pub english: &'static str,
}

pub type Lesson = Vec<Item>;