//! Huffman

use std::collections::binary_heap::BinaryHeap;
use std::collections::btree_map::BTreeMap;
use std::iter::FromIterator;

use crate::counter::Counter;
use std::cmp::Ordering;
use std::rc::Rc;

type HuffmanRcNode = Option<Rc<HuffmanNode>>;

#[derive(Debug, Eq)]
struct HuffmanNode {
    frequency: u128,
    character: char,
    left: HuffmanRcNode,
    right: HuffmanRcNode,
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character && self.frequency == other.frequency
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl HuffmanNode {
    fn new(
        frequency: u128,
        character: char,
        left: HuffmanRcNode,
        right: HuffmanRcNode,
    ) -> HuffmanNode {
        HuffmanNode {
            frequency,
            character,
            left,
            right,
        }
    }
}

#[derive(Debug)]
struct HuffmanTree {
    tree: BinaryHeap<HuffmanNode>,
}

impl HuffmanTree {
    fn new() -> Self {
        HuffmanTree {
            tree: BinaryHeap::new(),
        }
    }

    fn encode(&mut self, stream: &str) {
        let counter: Counter<char> = Counter::from_iter(stream.chars());

        // Step 1.
        // Build a min heap that contains 6 nodes
        // where each node represents root of a tree with single node.
        for (character, frequency) in &counter {
            self.tree
                .push(HuffmanNode::new(*frequency, *character, None, None));
        }

        // repeat steps #2 and #3
        while self.tree.len() > 1 {
            // Step 2.
            // Extract two nodes with the minimum frequency from the min heap.
            let first_node = match self.tree.pop() {
                Some(node) => node,
                None => break,
            };
            let second_node = match self.tree.pop() {
                Some(node) => node,
                None => break,
            };

            // Step 3.
            // Create a new internal node with a frequency equal to
            // the sum of the two nodes frequencies.
            // Make the first extracted node as its left child
            // and the other extracted node as its right child.
            // Add this node to the min heap.
            let new_node: HuffmanNode;
            if first_node.frequency == second_node.frequency {
                if first_node.character < second_node.character {
                    new_node = HuffmanNode::new(
                        first_node.frequency + second_node.frequency,
                        '$',
                        Some(Rc::new(first_node)),
                        Some(Rc::new(second_node)),
                    );
                } else {
                    new_node = HuffmanNode::new(
                        first_node.frequency + second_node.frequency,
                        '$',
                        Some(Rc::new(second_node)),
                        Some(Rc::new(first_node)),
                    );
                }
            } else {
                new_node = HuffmanNode::new(
                    first_node.frequency + second_node.frequency,
                    '$',
                    Some(Rc::new(first_node)),
                    Some(Rc::new(second_node)),
                );
            }
            self.tree.push(new_node);
        }
    }

    fn explore_tree(&self, node: &HuffmanNode, buffer: String, output: &mut Vec<(char, String)>) {
        if let Some(rc_node) = &node.left {
            self.explore_tree(rc_node.as_ref(), format!("{}0", buffer), output);
        }
        if let Some(rc_node) = &node.right {
            self.explore_tree(rc_node.as_ref(), format!("{}1", buffer), output);
        }
        if node.character != '$' {
            output.push((node.character, buffer));
        }
    }

    fn extract_encoding(&mut self) -> Vec<(char, String)> {
        if let Some(node) = self.tree.peek() {
            let mut output_vec: Vec<(char, String)> = Vec::new();
            self.explore_tree(node, String::new(), &mut output_vec);
            output_vec
        } else {
            panic!("No Huffman tree!")
        }
    }
}

/// Huffman `Encoder`
#[derive(Debug)]
pub struct Huffman {
    encoding: BTreeMap<char, String>,
    decoding: BTreeMap<String, char>,
    huffman_tree: HuffmanTree,
}

impl Huffman {
    pub fn new() -> Self {
        Huffman {
            encoding: BTreeMap::new(),
            decoding: BTreeMap::new(),
            huffman_tree: HuffmanTree::new(),
        }
    }

    // from: https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/
    pub fn huffman_encode(&mut self, stream: &str) {
        self.huffman_tree.encode(stream);
        let encoding_data = self.huffman_tree.extract_encoding();
        for (character, encoding) in encoding_data {
            self.update(character, encoding);
        }
    }

    fn update<S>(&mut self, character: char, encoding: S)
    where
        S: ToString + Sized,
    {
        self.encoding.insert(character, encoding.to_string());
        self.decoding.insert(encoding.to_string(), character);
    }

    pub fn encode<S>(&mut self, stream: &S) -> String
    where
        S: ToString + Sized,
    {
        let stream_string = stream.to_string();
        self.huffman_encode(&stream_string);
        stream_string
            .chars()
            .map(|c| {
                if !self.encoding.contains_key(&c) {
                    panic!("No encoding for character {}", c);
                }
                String::from(&self.encoding[&c])
            })
            .collect::<String>()
    }

    pub fn decode<S>(&mut self, stream: &S) -> String
    where
        S: ToString + Sized,
    {
        let stream_string = stream.to_string();
        let mut buffer = String::new();
        let mut decoded = String::new();
        for char in stream_string.chars() {
            buffer.push(char);
            match self.decoding.get(buffer.as_str()) {
                Some(character) => {
                    decoded.push(*character);
                    buffer.clear();
                }
                None => continue,
            };
        }
        decoded
    }
}
