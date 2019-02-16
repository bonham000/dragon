#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub characters: &'static str,
    pub phonetic: &'static str,
    pub english: &'static str,
}

pub type Lesson = Vec<Item>;

pub type LessonSet = Vec<Lesson>;
