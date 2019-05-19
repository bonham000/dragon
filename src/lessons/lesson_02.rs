use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            simplified: "这",
            traditional: "這",
            pinyin: "zhè",
            english: "Here, this",
            english_alternate_choices: vec!["Where", "Who", "How", "When"],
        },
        Item {
            simplified: "那",
            traditional: "那",
            pinyin: "nà",
            english: "There, that",
            english_alternate_choices: vec!["Where", "Who", "How", "When"],
        },
        Item {
            simplified: "哪",
            traditional: "哪",
            pinyin: "nǎ",
            english: "Where",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "谁",
            traditional: "誰",
            pinyin: "shuí",
            english: "Who",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "什么",
            traditional: "什麼",
            pinyin: "shénme",
            english: "What, why",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "多少",
            traditional: "多少",
            pinyin: "duōshǎo",
            english: "A few, how many",
            english_alternate_choices: vec![""],
        },
        Item {
            simplified: "怎么",
            traditional: "怎麼",
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
