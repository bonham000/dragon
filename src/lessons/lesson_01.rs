use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            traditional: "我",
            simplified: "我",
            pinyin: "wǒ",
            english: "I, me",
            english_alternate_choices: vec!["She", "It", "They", "We", "Us", "Him"],
        },
        Item {
            traditional: "我們",
            simplified: "我们",
            pinyin: "wǒmen",
            english: "We, us",
            english_alternate_choices: vec!["You", "They", "Him/Her", "I, me"],
        },
        Item {
            traditional: "你",
            simplified: "你",
            pinyin: "nǐ",
            english: "You",
            english_alternate_choices: vec!["Him/Her", "They", "Us", "I, me"],
        },
        Item {
            traditional: "你們",
            simplified: "你们",
            pinyin: "nǐmen",
            english: "You",
            english_alternate_choices: vec!["I, me", "Us", "They", "Him/Her"],
        },
        Item {
            traditional: "他",
            simplified: "他",
            pinyin: "tā",
            english: "He, him",
            english_alternate_choices: vec!["They", "Us", "I, me", "We"],
        },
        Item {
            traditional: "她",
            simplified: "她",
            pinyin: "tā",
            english: "She, her",
            english_alternate_choices: vec!["They", "Us", "I, me", "We"],
        },
        Item {
            traditional: "他們",
            simplified: "他们",
            pinyin: "tāmen",
            english: "They",
            english_alternate_choices: vec!["I, me", "Us", "Him/Her", "We"],
        },
        Item {
            traditional: "她們",
            simplified: "她们",
            pinyin: "tāmen",
            english: "They",
            english_alternate_choices: vec!["I, me", "Us", "Him/Her", "We"],
        },
    ]
}
