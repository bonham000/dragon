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
        Item {
            characters: "You have two linked lists of integers representing numbers in reverse order. Add them in linear time.",
            english: "Iterate both lists adding the value at each node and carrying anything greater than 10, adding a final node if there is a carry at the end.",
            phonetic: "",
        },
        Item {
            characters: "Given a string find all permutations.",
            english: "Iterate through the string and recursively find all permutations for each substring, excluding the character at that position. Add all results to a set.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree determine if it is balanced.",
            english: "Find the max and min heights of the tree and verify they differ by no more than 1. Can do this by traversing the subtrees and tracking the height, comparing at each leaf node.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree return if it is a valid binary search tree.",
            english: "You have to recursively traverse both subtrees, keeping track of a minimum value for the left subtree and a maximum value for the right subtree and verifying all child nodes satisfy the properties of a binary search tree.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree write a function to serialize and deserialize this tree.",
            english: "You can perform a breadth first search (level order traversal) and serialize the result as a comma separate string. Then, you can deserialize this string by splitting it at the comma values and then re-building the tree from the root -> left -> right until all values are added back.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree, converting it to a doubly linked list.",
            english: "We could use an inorder traversal (left, node, right), or any other traversal, which keeps track of a previous node, to build a linked list. At each node, we assign a new list node and connect it to the previously built node until we traverse the entire tree.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree print the nodes in columns from left to right, top to bottom.",
            english: "Traverse the tree in a depth first search, tracking the column position (root is zero). For each node, add it to a results hash map, adding an array of values for each column. Then iterate the hash map from the min to max column values, pushing all the result values from each column to a final results array.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree perform preorder traversal without using recursion!",
            english: "Push root to a stack, pop from stack and push to results array. Push right, then left subtree to the stack. Repeat until stack is empty.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree perform inorder traversal without using recursion!",
            english: "If the node is not null, push it to a stack, push all left nodes first. Until the stack is empty, pop the top node, push value to a results array, and set the node to the right child.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree perform postorder traversal without using recursion!",
            english: "Use two stacks. Pop values from the first stack and push them to the second stack. For each popped node, push the left and right child to the first stack. Pop the second stack and print the results.",
            phonetic: "",
        },
        Item {
            characters: "Given a 2D grid of 0s and 1s, with rectangles of 0s, determine all coordinates and dimensions of rectangles.",
            english: "Iterate through the grid, tracking each rectangle position and recording the results.",
            phonetic: "",
        },
        Item {
            characters: "Flatten an array iteratively.",
            english: "If the first element is an array, spread the contents to the start of the array. Otherwise, shift the first element and push it to a results array.",
            phonetic: "",
        },
        Item {
            characters: "Given a series of houses with money, determine the maximum amount of money you can steal if you cannot rob adjacent houses.",
            english: "Use dynamic programming: iterate through the array and track the maximum value possible at each position. Each position is the maximum of the i - 1 house maximum plus the current, or the maximum at i - 1. Return the final value in the maximum array.",
            phonetic: "",
        },
        Item {
            characters: "Find the inorder successor of a value in a binary tree.",
            english: "Perform inorder traversal and track a flag for finding the target value. Once it is found, return the next node value the traversal encounters.",
            phonetic: "",
        },
        Item {
            characters: "Given two sorted lists join them.",
            english: "Iterate through the lists for each element add the least of them to a results list. Once one list is exhausted, add all the elements of the remaining list to the results list.",
            phonetic: "",
        },
        Item {
            characters: "Given a binary tree perform level order traversal.",
            english: "Create a queue and enqueue the root node. Dequeue and push the node value to a results array. Enqueue the left and right subtrees. Continue until the queue is empty.",
            phonetic: "",
        },
        Item {
            characters: "Given a linked list determine if there is a cycle.",
            english: "Traverse the list to the end keeping a cache of all visiting nodes. If you encountered a cached value, there is a cycle.",
            phonetic: "",
        },
        Item {
            characters: "Given K sorted lists, merge them.",
            english: "Recursively merge the first two lists until all K lists are merge. Use a helper function to merge two lists, which iterates both lists rearranging the references to each next greatest item until the ends of the list are reached.",
            phonetic: "",
        },
        Item {
            characters: "Given a 2D matrix of values, rotate is 90 degrees.",
            english: "Create a helper to rotate a value around the grid for a given coordinate position. Iterate through the grid with good bounds checking and run this helper for every value.",
            phonetic: "",
        },
        Item {
            characters: "Given an array of integer heights, return the maximum area of trapped rainwater.",
            english: "Iterate from the initial and end position of the array, setting the maximum area at each position. Only advance the lesser of the heights at each position.",
            phonetic: "",
        },
        Item {
            characters: "Given an array of integers representing heights, return the total amount of rainwater trapped within the vacant spaces.",
            english: "Iterate inward from the start and end of the array, tracking the maximum heights on the left and right. For each space, if it's possible to trap water add that trapped water to a running total.",
            phonetic: "",
        },
        Item {
            characters: "Given a string and mappings between encoded and decoded values, return all the ways the string can be decoded.",
            english: "Use recursion to decode each encoding variation, i.e. encode single or double digit values. Empty string is the base case of 1. Then, use memoization to cache results of each calculation to run in linear time.",
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
