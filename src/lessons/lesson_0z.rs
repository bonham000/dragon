use super::types::{Item, Lesson};

pub fn get_content() -> Lesson {
    vec![
        Item {
            characters: "Given a binary tree and a sum, return all the paths in the tree that add to the sum.",
            english: "Traverse from each parent adding child nodes until leaf nodes. Whenever the current sum equals the target, a path has been found. Repeat the process for each child node as you traverse down the tree.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree return if a path exists to a sum (root to leaf).",
            english: "Start traversing all paths in the tree to leaf nodes, building a sum as you go. If you reach a leaf node and the sum matches, return true.",
            phonetic: "",
        },
        Item {
            characters: "Invert a binary tree.",
            english: "Write a recursive function which swaps the left and right children references for each parent. Then call the same function for each child subtree until all nodes are processed.",
            phonetic: "",
        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
//        Item {
//            characters: "",
//            english: "",
//            phonetic: "",
//        },
    ]
}
