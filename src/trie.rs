use std::{collections::HashMap, iter::Iterator};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Trie {
    root: TrieNode,
}

pub struct TrieSearcher<'a> {
    trie_node: &'a TrieNode,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct TrieNode {
    val: String,
    full: bool,
    children: HashMap<char, TrieNode>,
}

impl Trie {
    pub fn new<'a, I: Iterator<Item = &'a str>>(values: I) -> Trie {
        let mut trie = Trie::default();
        for value in values {
            trie.add(value);
        }
        trie
    }

    pub fn add(&mut self, value: &str) {
        self.root.add(String::new(), value);
    }

    pub fn get_searcher<'a>(&'a self) -> TrieSearcher<'a> {
        TrieSearcher {
            trie_node: &self.root,
        }
    }
}

impl TrieNode {
    // This function adds a value to the node and recursively populates it
    fn add(&mut self, curr: String, value: &str) {
        self.val = curr.clone();
        if let Some(next_char) = value.chars().nth(0) {
            if let Some(node) = self.children.get_mut(&next_char) {
                node.add(curr + &value[..1], &value[1..]);
            }
        } else {
            self.full = true;
        }
    }

    fn get_next(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }

    fn get_value(&self) -> &str {
        &self.val
    }

    fn is_full(&self) -> bool {
        self.full
    }
}

impl<'a> TrieSearcher<'a> {
    pub fn advance(&mut self, c: char) -> Option<&str> {
        if let Some(node) = self.trie_node.get_next(c) {
            self.trie_node = node;

            if self.trie_node.full {
                return Some(self.trie_node.get_value());
            }
        }

        None
    }
}
