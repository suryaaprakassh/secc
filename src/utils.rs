use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_at_end: bool,
    children: HashMap<char, TrieNode>,
}

pub struct KeyWordManager {
    root: TrieNode,
}

impl KeyWordManager {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn populate(&mut self, words: &[&str]) {
        for word in words {
            self.insert(word)
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_at_end = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_at_end
    }
}
