use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            simplified: "我",
            traditional: "我",
            pinyin: "wǒ",
            english: "I, me",
            english_alternate_choices: vec!["She", "It", "They", "We, us", "Him"],
        },
        Item {
            simplified: "我们",
            traditional: "我們",
            pinyin: "wǒmen",
            english: "We, us",
            english_alternate_choices: vec!["You", "They", "Him/Her", "I, me", "It"],
        },
        Item {
            simplified: "你",
            traditional: "你",
            pinyin: "nǐ",
            english: "You (sg.)",
            english_alternate_choices: vec!["He, him", "They", "We, us", "I, me"],
        },
        Item {
            simplified: "你们",
            traditional: "你們",
            pinyin: "nǐmen",
            english: "You (pl.)",
            english_alternate_choices: vec!["I, me", "We, us", "They", "He, him"],
        },
        Item {
            simplified: "他",
            traditional: "他",
            pinyin: "tā",
            english: "He, him",
            english_alternate_choices: vec!["They", "We, us", "I, me", "She, her"],
        },
        Item {
            simplified: "她",
            traditional: "她",
            pinyin: "tā",
            english: "She, her",
            english_alternate_choices: vec!["They", "We, us", "I, me", "She, her"],
        },
        Item {
            simplified: "他们",
            traditional: "他們",
            pinyin: "tāmen",
            english: "They (male)",
            english_alternate_choices: vec!["I, me", "We, us", "He, him", "She, her"],
        },
        Item {
            simplified: "她们",
            traditional: "她們",
            pinyin: "tāmen",
            english: "They (female)",
            english_alternate_choices: vec!["I, me", "We, us", "She, her", "She, her"],
        },
    ]
}
