use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            traditional: "這",
            simplified: "这",
            pinyin: "zhè",
            english: "Here, this",
            english_alternate_choices: vec!["Where", "Who", "How", "When"],
        },
        Item {
            traditional: "那",
            simplified: "那",
            pinyin: "nà",
            english: "There, that",
            english_alternate_choices: vec!["Where", "Who", "How", "When"],
        },
        Item {
            traditional: "哪",
            simplified: "哪",
            pinyin: "nǎ",
            english: "Where",
            english_alternate_choices: vec![""],
        },
        Item {
            traditional: "誰",
            simplified: "谁",
            pinyin: "shuí",
            english: "Who",
            english_alternate_choices: vec![""],
        },
        Item {
            traditional: "什麼",
            simplified: "什么",
            pinyin: "shénme",
            english: "What, why",
            english_alternate_choices: vec![""],
        },
        Item {
            traditional: "多少",
            simplified: "多少",
            pinyin: "duōshǎo",
            english: "A few, how many",
            english_alternate_choices: vec![""],
        },
        Item {
            traditional: "怎麼",
            simplified: "怎么",
            pinyin: "zěnme",
            english: "How",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "怎么样",
            traditional: "怎麼樣",
            pinyin: "zěnmeyàng",
            english: "How about",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "您",
            traditional: "您",
            pinyin: "nín",
            english: "You",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "它",
            traditional: "它",
            pinyin: "tā",
            english: "It",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "大家",
            traditional: "大家",
            pinyin: "dàjiā",
            english: "Everyone",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "每",
            traditional: "每",
            pinyin: "měi",
            english: "Every",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "为什么",
            traditional: "為什麼",
            pinyin: "wèishénme",
            english: "Why",
            english_alternate_choices: vec![""],
        },
    ]
}
