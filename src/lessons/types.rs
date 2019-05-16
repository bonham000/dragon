pub type Alternates = Vec<&'static str>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub simplified: &'static str,
    pub traditional: &'static str,
    pub pinyin: &'static str,
    pub english: &'static str,
    pub usage_notes: &'static str,
    pub part_of_speech: &'static str,
    pub english_alternate_choices: Alternates,
}

pub type Lesson = Vec<Item>;

pub type LessonSet = Vec<Lesson>;
