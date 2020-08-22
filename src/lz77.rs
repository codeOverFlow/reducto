use std::cmp::{min, Ordering};
use std::slice::Iter;

#[derive(Debug, Default, Eq)]
pub struct LZ77Match {
    index: usize,
    match_data: Vec<u8>,
}

impl LZ77Match {
    pub fn len(&self) -> usize {
        self.match_data.len()
    }
}

impl From<(Vec<u8>, usize)> for LZ77Match {
    fn from(data: (Vec<u8>, usize)) -> Self {
        let (match_data, index) = data;
        LZ77Match { index, match_data }
    }
}

impl PartialEq for LZ77Match {
    fn eq(&self, other: &Self) -> bool {
        self.match_data == other.match_data && self.index == other.index
    }
}

impl PartialOrd for LZ77Match {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LZ77Match {
    fn cmp(&self, other: &Self) -> Ordering {
        self.match_data.len().cmp(&other.match_data.len())
    }
}

#[derive(Debug)]
pub enum LZ77NodeMatch {
    NextMatch(u8),
    EndOfData,
}

impl PartialEq for LZ77NodeMatch {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LZ77NodeMatch::NextMatch(value) => match other {
                LZ77NodeMatch::NextMatch(other_value) => value == other_value,
                LZ77NodeMatch::EndOfData => true,
            },
            Self::EndOfData => match other {
                LZ77NodeMatch::NextMatch(_) => false,
                LZ77NodeMatch::EndOfData => true,
            },
        }
    }
}

impl Eq for LZ77NodeMatch {}

#[derive(Debug, Eq)]
pub struct LZ77Node {
    offset: usize,
    length: usize,
    next_match: LZ77NodeMatch,
}

impl LZ77Node {
    fn new(offset: usize, length: usize, next_match: LZ77NodeMatch) -> LZ77Node {
        LZ77Node {
            offset,
            length,
            next_match,
        }
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.push(self.offset as u8);
        res.push(self.length as u8);
        res.push(self.offset as u8);
        if let LZ77NodeMatch::NextMatch(data) = self.next_match {
            res.push(data);
        }
        res
    }
}

impl PartialEq for LZ77Node {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.offset == other.offset
            && self.next_match == other.next_match
    }
}

#[derive(Debug)]
pub struct LZ77 {
    window_size: usize,
    dictionary_size: usize,
    position: usize,
    nodes: Vec<LZ77Node>,
}

fn find_subvec_in_vec(source: &[u8], target: &[u8]) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    if target.len() == 0 || target.len() > source.len() {
        // return 0
        return res;
    }

    for (index, byte) in source.iter().enumerate().rev() {
        if *byte == target[0] {
            let res_index = index;
            let mut res_size: usize = 0;
            for i in 0..min(source.len(), target.len()) {
                if index + i < source.len() {
                    if target[i] == source[index + i] {
                        res_size += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            res.push((res_index, res_size));
        }
    }
    res
}

impl LZ77 {
    pub fn new(window_size: usize, dictionary_size: usize) -> LZ77 {
        assert_ne!(window_size, 0);
        assert_ne!(dictionary_size, 0);
        assert!(window_size <= u8::MAX as usize);
        assert!(dictionary_size <= u8::MAX as usize);
        LZ77 {
            window_size,
            dictionary_size,
            position: 0,
            nodes: Vec::new(),
        }
    }

    fn find_longest_match(&self, data: &[u8]) -> LZ77Match {
        let mut res: LZ77Match = Default::default();
        if self.position > 0 {
            let dictionary_index_start =
                self.position.checked_sub(self.dictionary_size).unwrap_or(0);
            let data_available_index_end = min(self.position, data.len());
            let data_available = &data[dictionary_index_start..data_available_index_end];
            let data_window_index_end = min(self.position + self.window_size, data.len());
            let data_window = &data[self.position..data_window_index_end];

            for i in 0..data_window.len() {
                let substring = &data_window[..=i];
                let matches = find_subvec_in_vec(data_available, substring);

                for (index, size) in matches {
                    let index_start = dictionary_index_start + index;

                    let index_end = match index + size < data_available.len() {
                        true => index + size,
                        false => data_available.len(),
                    };

                    let match_data: Vec<u8> = Vec::from(&data_available[index..index_end]);
                    let new_match = LZ77Match::from((match_data, index_start));

                    if new_match > res {
                        res = new_match;
                    }
                }
            }
        }
        res
    }

    pub fn encode<S>(&mut self, data: S)
    where
        S: Sized + ToString,
    {
        let data: Vec<u8> = Vec::from(data.to_string().as_bytes());
        let data_length = data.len();

        while self.position < data_length {
            // 1. get the longest match
            let longest_match = self.find_longest_match(&data);

            // get the window after the match
            let next_match_value: u8;
            let longest_match_size = longest_match.len();
            let offset = match longest_match_size {
                0 => 0,
                _ => self.position.checked_sub(longest_match.index).unwrap_or(0),
            };
            let next_match_position_start = self.position + longest_match_size;
            if next_match_position_start < data_length {
                next_match_value = match data.iter().nth(next_match_position_start) {
                    Some(character) => *character,
                    None => panic!("Error: Index {} out of bound", next_match_position_start),
                };
            } else {
                let node = LZ77Node::new(offset, longest_match_size, LZ77NodeMatch::EndOfData);
                self.nodes.push(node);
                break;
            }

            let node = LZ77Node::new(
                offset,
                longest_match_size,
                LZ77NodeMatch::NextMatch(next_match_value),
            );

            self.nodes.push(node);

            // 3. update position
            self.position += match longest_match_size {
                0 => 1,
                _ => longest_match_size + 1, // skip longest_match and the next char that we already encoded
            };
        }
    }

    pub fn decode(&self) -> String {
        let mut decoded_string: Vec<u8> = Vec::new();

        for lz77_node in &self.nodes {
            let length = lz77_node.length;
            let lz77_match = &lz77_node.next_match;

            if length == 0 {
                match lz77_match {
                    LZ77NodeMatch::NextMatch(character) => decoded_string.push(*character),
                    LZ77NodeMatch::EndOfData => break,
                }
            } else {
                // Read the stored sequence
                let offset = lz77_node.offset;

                let index_start = decoded_string // start = string.len() - offset
                    .len()
                    .checked_sub(offset)
                    .unwrap_or(0);

                let index_end = index_start // end = start + (len || (string.len - 1))
                    .checked_add(length)
                    .unwrap_or(decoded_string.len().checked_sub(1).unwrap_or(0));

                let substring: Vec<u8> = Vec::from(&decoded_string[index_start..index_end]);
                decoded_string.extend(substring.iter());

                // push the next symbol
                match lz77_match {
                    LZ77NodeMatch::NextMatch(character) => decoded_string.push(*character),
                    LZ77NodeMatch::EndOfData => break,
                }
            }
        }

        match String::from_utf8(decoded_string) {
            Ok(string) => string,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn iter(&self) -> Iter<'_, LZ77Node> {
        self.nodes.iter()
    }
}
