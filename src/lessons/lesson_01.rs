use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    Lesson {
        list: "1-2",
        content: vec![
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
            Item {
                simplified: "一",
                traditional: "一",
                pinyin: "yī",
                english: "one",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "二",
                traditional: "二",
                pinyin: "èr",
                english: "two",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "三",
                traditional: "三",
                pinyin: "sān",
                english: "three",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "四",
                traditional: "四",
                pinyin: "sì",
                english: "four",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "五",
                traditional: "五",
                pinyin: "wǔ",
                english: "five",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "六",
                traditional: "六",
                pinyin: "liù",
                english: "six",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "七",
                traditional: "七",
                pinyin: "qī",
                english: "seven",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "八",
                traditional: "八",
                pinyin: "bā",
                english: "eight",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "九",
                traditional: "九",
                pinyin: "jiǔ",
                english: "nine",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "十",
                traditional: "十",
                pinyin: "shí",
                english: "ten",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "零",
                traditional: "零",
                pinyin: "líng",
                english: "zero",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "两",
                traditional: "两",
                pinyin: "liǎng",
                english: "two",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "百",
                traditional: "百",
                pinyin: "bǎi",
                english: "hundred",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "千",
                traditional: "千",
                pinyin: "qiān",
                english: "thousand",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "第一",
                traditional: "第一",
                pinyin: "dìyī",
                english: "first",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "个",
                traditional: "个",
                pinyin: "gè",
                english: "one,",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "岁",
                traditional: "岁",
                pinyin: "suì",
                english: "year",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "本",
                traditional: "本",
                pinyin: "běn",
                english: "volume",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "些",
                traditional: "些",
                pinyin: "xiē",
                english: "some",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "块",
                traditional: "块",
                pinyin: "kuài",
                english: "piece",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "次",
                traditional: "次",
                pinyin: "cì",
                english: "number",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "公斤",
                traditional: "公斤",
                pinyin: "gōngjīn",
                english: "kilogram",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "元",
                traditional: "元",
                pinyin: "yuán",
                english: "yuan",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "件",
                traditional: "件",
                pinyin: "jiàn",
                english: "piece",
                english_alternate_choices: vec![""],
            },
            Item {
                simplified: "张",
                traditional: "张",
                pinyin: "zhāng",
                english: "sheet",
                english_alternate_choices: vec![""],
            },
        ],
    }
}
