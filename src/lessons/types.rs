#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub simplified: &'static str,
    pub traditional: &'static str,
    pub phonetic: &'static str,
    pub english: &'static str,
}

pub type Lesson = Vec<Item>;

pub type LessonSet = Vec<Lesson>;
