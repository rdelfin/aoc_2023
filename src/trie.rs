use std::{collections::HashMap, iter::Iterator};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Trie {
    root: TrieNode,
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
            processed_chars: 0,
        }
    }
}

impl std::fmt::Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct TrieNode {
    val: String,
    full: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    // This function adds a value to the node and recursively populates it
    fn add(&mut self, curr: String, value: &str) {
        self.val = curr.clone();
        if let Some(next_char) = value.chars().nth(0) {
            if let Some(node) = self.children.get_mut(&next_char) {
                node.add(curr + &value[..1], &value[1..]);
            } else {
                let mut node = TrieNode::default();
                node.add(curr + &value[..1], &value[1..]);
                self.children.insert(next_char, node);
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

impl std::fmt::Display for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leaf = if self.is_full() { " 🍃" } else { "" };
        let children_str = self.children.iter().fold(String::new(), |acc, (_, child)| {
            if acc.is_empty() {
                format!("{child}")
            } else {
                format!("{acc}, {child}")
            }
        });
        let children_str = if self.children.is_empty() {
            String::new()
        } else {
            format!(": [{children_str}]")
        };
        write!(f, "(\"{}\"{leaf}){children_str}", self.val)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("there was no match for this character")]
    NoMatch,
}

pub struct TrieSearcher<'a> {
    trie_node: &'a TrieNode,
    processed_chars: usize,
}

impl<'a> TrieSearcher<'a> {
    pub fn advance(&mut self, c: char) -> Result<Option<String>, SearchError> {
        if let Some(node) = self.trie_node.get_next(c) {
            self.trie_node = node;
            self.processed_chars += 1;

            if self.trie_node.is_full() {
                Ok(Some(self.trie_node.get_value().into()))
            } else {
                Ok(None)
            }
        } else {
            Err(SearchError::NoMatch)
        }
    }

    pub fn len(&self) -> usize {
        self.processed_chars
    }
}
