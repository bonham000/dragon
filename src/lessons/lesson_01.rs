use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            traditional: "我",
            simplified: "我",
            pinyin: "wǒ",
            english: "I, me",
            english_alternate_choices: vec!["She", "It", "They", "We, us", "Him"],
        },
        Item {
            traditional: "我們",
            simplified: "我们",
            pinyin: "wǒmen",
            english: "We, us",
            english_alternate_choices: vec!["You", "They", "Him/Her", "I, me", "It"],
        },
        Item {
            traditional: "你",
            simplified: "你",
            pinyin: "nǐ",
            english: "You (sg.)",
            english_alternate_choices: vec!["He, him", "They", "We, us", "I, me"],
        },
        Item {
            traditional: "你們",
            simplified: "你们",
            pinyin: "nǐmen",
            english: "You (pl.)",
            english_alternate_choices: vec!["I, me", "We, us", "They", "He, him"],
        },
        Item {
            traditional: "他",
            simplified: "他",
            pinyin: "tā",
            english: "He, him",
            english_alternate_choices: vec!["They", "We, us", "I, me", "She, her"],
        },
        Item {
            traditional: "她",
            simplified: "她",
            pinyin: "tā",
            english: "She, her",
            english_alternate_choices: vec!["They", "We, us", "I, me", "She, her"],
        },
        Item {
            traditional: "他們",
            simplified: "他们",
            pinyin: "tāmen",
            english: "They (male)",
            english_alternate_choices: vec!["I, me", "We, us", "He, him", "She, her"],
        },
        Item {
            traditional: "她們",
            simplified: "她们",
            pinyin: "tāmen",
            english: "They (female)",
            english_alternate_choices: vec!["I, me", "We, us", "She, her", "She, her"],
        },
    ]
}
