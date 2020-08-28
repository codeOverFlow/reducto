use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::BitAnd;
use std::path::Path;

#[derive(Debug)]
pub struct Bits {
    bits: String,
    delimiter: char,
}

impl Bits {
    pub fn new(data: Vec<u8>) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:08b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
        }
    }

    pub fn next_data_as_u8(&mut self, size_to_read: usize) -> u8 {
        assert!(size_to_read <= 8);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u8::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_reversed_as_u8(&mut self, size_to_read: usize) -> u8 {
        assert!(size_to_read <= 8);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u8::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_as_u16(&mut self, size_to_read: usize) -> u16 {
        assert!(size_to_read <= 16);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u16::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_reversed_as_u16(&mut self, size_to_read: usize) -> u16 {
        assert!(size_to_read <= 16);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u16::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_as_u32(&mut self, size_to_read: usize) -> u32 {
        assert!(size_to_read <= 32);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u32::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_reversed_as_u32(&mut self, size_to_read: usize) -> u32 {
        assert!(size_to_read <= 32);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u32::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_as_u64(&mut self, size_to_read: usize) -> u64 {
        assert!(size_to_read <= 64);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u64::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_reversed_as_u64(&mut self, size_to_read: usize) -> u64 {
        assert!(size_to_read <= 64);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u64::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_as_u128(&mut self, size_to_read: usize) -> u128 {
        assert!(size_to_read <= 128);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u128::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_data_reversed_as_u128(&mut self, size_to_read: usize) -> u128 {
        assert!(size_to_read <= 128);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        let res = match u128::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(size_to_read);
        res
    }

    pub fn next_byte(&mut self) -> u8 {
        let expected_delimiter = 1;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            idx += 1;
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }
        }
        let res = match u8::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(8);
        res
    }

    pub fn next_byte_reversed(&mut self) -> u8 {
        let expected_delimiter = 1;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            idx += 1;
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }
        }
        let res = match u8::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(8);
        res
    }

    pub fn next_word(&mut self) -> u16 {
        let expected_delimiter = 2;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u16::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(16);
        res
    }

    pub fn next_word_reversed(&mut self) -> u16 {
        let expected_delimiter = 2;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u16::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(16);
        res
    }

    pub fn next_double_word(&mut self) -> u32 {
        let expected_delimiter = 4;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u32::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(32);
        res
    }

    pub fn next_double_word_reversed(&mut self) -> u32 {
        let expected_delimiter = 4;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u32::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(32);
        res
    }

    pub fn next_64_bits(&mut self) -> u64 {
        let expected_delimiter = 8;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u64::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(64);
        res
    }

    pub fn next_64_bits_reversed(&mut self) -> u64 {
        let expected_delimiter = 8;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u64::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(64);
        res
    }

    pub fn next_128_bits(&mut self) -> u128 {
        let expected_delimiter = 16;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u128::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(128);
        res
    }

    pub fn next_128_bits_reversed(&mut self) -> u128 {
        let expected_delimiter = 16;
        let mut found_delimiter = 0;
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while found_delimiter != expected_delimiter {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            } else {
                found_delimiter += 1;
            }

            idx += 1;
        }
        let res = match u128::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        };
        self.move_n_bits(128);
        res
    }

    pub fn peek_next_data_as_u8(&mut self, size_to_read: usize) -> u8 {
        assert!(size_to_read <= 8);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u8::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u8_reversed(&mut self, size_to_read: usize) -> u8 {
        assert!(size_to_read <= 8);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u8::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u16(&mut self, size_to_read: usize) -> u16 {
        assert!(size_to_read <= 16);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u16::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u16_reversed(&mut self, size_to_read: usize) -> u16 {
        assert!(size_to_read <= 16);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u16::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u32(&mut self, size_to_read: usize) -> u32 {
        assert!(size_to_read <= 32);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u32::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u32_reversed(&mut self, size_to_read: usize) -> u32 {
        assert!(size_to_read <= 32);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u32::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u64(&mut self, size_to_read: usize) -> u64 {
        assert!(size_to_read <= 64);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u64::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u64_reversed(&mut self, size_to_read: usize) -> u64 {
        assert!(size_to_read <= 64);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u64::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u128(&mut self, size_to_read: usize) -> u128 {
        assert!(size_to_read <= 128);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u128::from_str_radix(&slice.iter().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_u128_reversed(&mut self, size_to_read: usize) -> u128 {
        assert!(size_to_read <= 128);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        match u128::from_str_radix(&slice.iter().rev().collect::<String>(), 2) {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason),
        }
    }

    pub fn peek_next_data_as_string(&mut self, size_to_read: usize) -> String {
        assert!(size_to_read <= 16);
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        slice.iter().collect::<String>()
    }

    pub fn move_n_bits(&mut self, n: usize) {
        assert!(n < self.bits.len());
        let x = &self.bits[..=n];
        let nb_delim = x.chars().filter(|c| *c == self.delimiter).count();
        self.bits = String::from(&self.bits[n + nb_delim..]);
    }

    pub fn display(&self) {
        println!("BITS: {}", self.bits)
    }
}

#[derive(Debug)]
pub enum GZipFlag {
    FTEXT = 0b0000_0001,
    FHCRC = 0b0000_0010,
    FEXTRA = 0b0000_0100,
    FNAME = 0b0000_1000,
    FCOMMENT = 0b0001_0000,
}

impl BitAnd<u8> for GZipFlag {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        (self as u8) & rhs
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CompressionType {
    NoCompression = 0,
    FixedCompression = 1,
    DynamicCompression = 2,
    Error = 3,
}

impl PartialEq<u8> for CompressionType {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    NotFinal = 0,
    Final = 1,
}

impl PartialEq<u8> for BlockType {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }
}

const MAX_BITS_IN_CODE: u16 = 15; // maximum bits in a code
const MAX_LITERAL_LENGTH_CODES: u16 = 286; // maximum number of literal/length codes
const NB_DISTANCE_CODES: u16 = 30; // maximum number of distance codes
const MAX_CODES_LENGTH_TO_READ: u16 = MAX_LITERAL_LENGTH_CODES + NB_DISTANCE_CODES; // maximum codes lengths to read
const NB_LITERAL_LENGTH_CODES: u16 = 288; // number of fixed literal/length codes

const LENGTHS_SIZE_BASE: [u16; 29] = [
    // Size base for length codes 257..285
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];

const LENGTHS_EXTRA_BITS: [u16; 29] = [
    // Extra bits for length codes 257..285
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

const DISTANCE_OFFSET_BASE: [u16; 30] = [
    // Offset base for distance codes 0..29
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049,
    3073, 4097, 6145, 8193, 12289, 16385, 24577,
];

const DISTANCE_EXTRA_BITS: [u16; 30] = [
    // Extra bits for distance codes 0..29
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

#[derive(Debug)]
pub struct GZip {}

impl GZip {
    pub fn decompress_file(path_to_gz: &str) {
        let path = Path::new(path_to_gz);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(reason) => panic!("{}", reason),
        };
        let mut bytes: Vec<u8> = Vec::new();
        if let Err(reason) = file.read_to_end(&mut bytes) {
            panic!("{}", reason);
        }

        // reverse it so I can use pop()
        let mut bits: Bits = Bits::new(bytes);
        bits.display();

        // header
        let id1 = bits.next_byte_reversed();
        let id2 = bits.next_byte_reversed();
        let cm = bits.next_byte_reversed();
        let flags = bits.next_byte_reversed();
        println!("ID1     : {:02x}", id1);
        println!("ID2     : {:02x}", id2);
        println!("CM      : {:02x}", cm);
        println!("flags   : {:08b}", flags);

        if GZipFlag::FTEXT & flags != 0 {
            // text
        }

        let mut mtime: u32 = bits.next_byte_reversed() as u32;
        mtime = ((bits.next_byte_reversed() as u32) << 8) + mtime;
        mtime = ((bits.next_byte_reversed() as u32) << 16) + mtime;
        mtime = ((bits.next_byte_reversed() as u32) << 24) + mtime;

        let xfl = bits.next_byte_reversed();
        let os = bits.next_byte_reversed();

        println!("MTIME   : {:04x}", mtime);
        println!("XFL     : {:08b}", xfl);
        println!("OS      : {:02x}", os);

        if GZipFlag::FEXTRA & flags != 0 {
            // extra
            let extra_length = bits.next_byte_reversed();
            println!("XTRA LEN: {:02x}", extra_length);
            for i in 0..extra_length {
                let extra_data = bits.next_byte_reversed();
                println!("XTRA DATA {}: {:02x}", i, extra_data);
            }
        }

        let mut file_name: Option<Vec<u8>> = None;
        if GZipFlag::FNAME & flags != 0 {
            // name
            let mut buffer: Vec<u8> = Vec::new();
            loop {
                let next = bits.next_byte_reversed();
                buffer.push(next);
                if next == 0x00 {
                    file_name = Some(buffer);
                    break;
                }
            }
            if file_name.is_none() {
                panic!(".gz file is malformed !");
            }
        }
        println!("FILENAME: {:02x?}", file_name);
        if let Some(ref v) = file_name {
            let name: String = v
                .iter()
                .take_while(|b| **b != 0)
                .map(|b| *b as char)
                .collect();
            println!("FILENAME: {}", name);
        }

        if GZipFlag::FCOMMENT & flags != 0 {
            loop {
                let next = bits.next_byte_reversed();
                if next == 0x00 {
                    break;
                }
            }
        }

        if GZipFlag::FHCRC & flags != 0 {
            // crc
            let mut crc16: u16 = bits.next_byte_reversed() as u16;
            crc16 = ((bits.next_byte_reversed() as u16) << 8) + crc16;
            println!("CRC16   : {:04x}", crc16);
        }

        // compressed data
        //
        // decoding algorithm from https://tools.ietf.org/html/rfc1951#page-10
        //
        // do
        //     read block header from input stream.
        //     if stored with no compression
        //         skip any remaining bits in current partially processed byte
        //         read LEN and NLEN (see next section)
        //         copy LEN bytes of data to output
        //     otherwise
        //         if compressed with dynamic Huffman codes
        //             read representation of code trees (see subsection below)
        //         loop (until end of block code recognized)
        //             decode literal/length value from input stream
        //             if value < 256
        //                 copy value (literal byte) to output stream
        //             otherwise
        //                 if value = end of block (256)
        //                     break from loop
        //                 otherwise (value = 257..285)
        //                     decode distance from input stream
        //
        //                     move backwards distance bytes in the output
        //                     stream, and copy length bytes from this
        //                     position to the output stream.
        //         end loop
        // while not last block
        let mut fixed_tree: BTreeMap<String, u16> = BTreeMap::new();
        let mut fixed_distance_tree: BTreeMap<String, u16> = BTreeMap::new();
        let mut output: Vec<u8> = Vec::new();
        loop {
            // mask the 3 bits header
            let bfinal: u8 = bits.next_data_reversed_as_u8(1);
            println!("BFINAL: {}", bfinal);

            let btype: u8 = bits.next_data_reversed_as_u8(2);
            println!("BTYPE: {}", btype);

            if CompressionType::NoCompression == btype {
                let mut len: u16 = bits.next_byte_reversed() as u16;
                len = ((bits.next_byte_reversed() as u16) << 8) + len;

                let mut nlen: u16 = bits.next_byte_reversed() as u16;
                len = ((bits.next_byte_reversed() as u16) << 8) + nlen;

                println!("LEN: {}", len);
                println!("NLEN: {}", nlen);

                for _ in 0..len {
                    output.push(bits.next_byte_reversed());
                }
            } else if CompressionType::Error == btype {
                panic!("Error in compression !");
            } else {
                let mut tree: &BTreeMap<String, u16> = &BTreeMap::new();
                let mut distance_tree: &BTreeMap<String, u16> = &BTreeMap::new();

                if CompressionType::DynamicCompression == btype {
                    println!("Dynamic Compression");
                } else {
                    println!("Fixed Compression");
                    if fixed_tree.len() == 0 {
                        let mut lengths: BTreeMap<u16, u16> = BTreeMap::new();

                        /* literal/length table */
                        for symbol in 0..144 {
                            lengths.insert(symbol, 8);
                        }
                        for symbol in 144..256 {
                            lengths.insert(symbol, 9);
                        }
                        for symbol in 256..280 {
                            lengths.insert(symbol, 7);
                        }
                        for symbol in 280..NB_LITERAL_LENGTH_CODES {
                            lengths.insert(symbol, 8);
                        }
                        println!("LENGTHS: {:#?}", lengths);

                        let mut count: BTreeMap<u16, u16> = BTreeMap::new();
                        let mut next_code: BTreeMap<u16, u16> = BTreeMap::new();

                        // count number of codes of each length
                        for symbol in 0..NB_LITERAL_LENGTH_CODES {
                            let value = count.entry(lengths[&symbol]).or_insert(0);
                            *value += 1;
                        }

                        println!("COUNT: {:#?}", count);

                        // Find the numerical value of the smallest code for each code length
                        let mut code: u16 = 0;
                        for bits in 1..=MAX_BITS_IN_CODE {
                            code = (code + *count.entry(bits - 1).or_default()) << 1;
                            next_code.insert(bits, code);
                        }

                        /* Assign numerical values to all codes, using consecutive
                         * values for all codes of the same length with the base
                         * values determined at step 2. Codes that are never used
                         * (which have a bit length of zero) must not be assigned a value.
                         */
                        for n in 0..NB_LITERAL_LENGTH_CODES {
                            let len = lengths[&n];
                            if len != 0 {
                                match len {
                                    7 => fixed_tree.insert(
                                        format!("{:07b}", *next_code.entry(len).or_insert(0)),
                                        n,
                                    ),
                                    8 => fixed_tree.insert(
                                        format!("{:08b}", *next_code.entry(len).or_insert(0)),
                                        n,
                                    ),
                                    9 => fixed_tree.insert(
                                        format!("{:09b}", *next_code.entry(len).or_insert(0)),
                                        n,
                                    ),
                                    _ => panic!("Invalid length: {}", len),
                                };
                                let value = next_code.entry(len).or_insert(0);
                                *value += 1;
                            }
                        }

                        println!("FIXED TREE:");
                        for (k, v) in fixed_tree.iter() {
                            if *v <= 255 {
                                let vv: u8 = *v as u8;
                                println!("-> {}: {} -> {}", k, vv, vv as char);
                            } else {
                                println!("-> {}: {}", k, *v);
                            }
                        }

                        /* distance table */
                        for symbol in 0..NB_DISTANCE_CODES {
                            lengths.insert(symbol, 5);
                        }

                        code = 0;
                        next_code = BTreeMap::new();
                        for bits in 1..=MAX_BITS_IN_CODE {
                            code = (code + *count.entry(bits - 1).or_default()) << 1;
                            next_code.insert(bits, code);
                        }

                        /* Assign numerical values to all codes, using consecutive
                         * values for all codes of the same length with the base
                         * values determined at step 2. Codes that are never used
                         * (which have a bit length of zero) must not be assigned a value.
                         */
                        for n in 0..NB_DISTANCE_CODES {
                            let len = lengths[&n];
                            if len != 0 {
                                match len {
                                    5 => fixed_distance_tree.insert(
                                        format!("{:05b}", *next_code.entry(len).or_insert(0)),
                                        n,
                                    ),
                                    _ => panic!("Invalid length: {}", len),
                                };
                                let value = next_code.entry(len).or_insert(0);
                                *value += 1;
                            }
                        }

                        println!("FIXED DISTANCE TREE:");
                        for (k, v) in fixed_distance_tree.iter() {
                            println!("-> {}: {}", k, *v);
                        }
                    }
                    tree = &fixed_tree;
                    distance_tree = &fixed_distance_tree;
                }

                loop {
                    bits.display();
                    let mut decoded: u16 = 0;
                    let masked = bits.peek_next_data_as_string(9); // check if it 9 bits code
                    println!("MASKED: {}", masked);
                    if tree.contains_key(&masked) {
                        decoded = tree[&masked];
                        // remake a 8 bits data
                        bits.move_n_bits(9);
                    } else {
                        let masked = bits.peek_next_data_as_string(8); // check if it 8 bits code
                        println!("MASKED: {}", masked);
                        if tree.contains_key(&masked) {
                            decoded = tree[&masked];
                            bits.move_n_bits(8);
                        } else {
                            let masked = bits.peek_next_data_as_string(7); // check if it 7 bits code
                            println!("MASKED: {}", masked);
                            if tree.contains_key(&masked) {
                                decoded = tree[&masked];
                                bits.move_n_bits(7);
                            }
                        }
                    }
                    println!("DECODED: {}", decoded);

                    if decoded == 256 {
                        break;
                    } else if decoded <= 255 {
                        println!("DECODED: {}", (decoded as u8) as char);
                        output.push(decoded as u8);
                    } else {
                        let index: usize = (decoded - 257) as usize;
                        let extra_bits_size = LENGTHS_EXTRA_BITS[index];
                        println!("LENGTH EXTRA BITS SIZE: {}", extra_bits_size);
                        let extra_bits_value: u16 =
                            bits.peek_next_data_as_u16(extra_bits_size as usize);
                        let length = LENGTHS_SIZE_BASE[index] + extra_bits_value;
                        println!("LENGTH: {}", length);

                        /* Distance codes 0-31 are represented by (fixed-length) 5-bit
                         * codes, with possible additional bits as shown in the table
                         * shown in Paragraph 3.2.5, above.  Note that distance codes 30-
                         * 31 will never actually occur in the compressed data.
                         */
                        let distance_code = bits.peek_next_data_as_string(5);
                        println!("DISTANCE CODE: {}", distance_code);
                        let mut distance: u16 = 0;
                        if distance_tree.contains_key(&distance_code) {
                            let distance_code = distance_tree[&distance_code];
                            let distance_extra_bits_size =
                                DISTANCE_EXTRA_BITS[distance_code as usize];
                            println!("DISTANCE EXTRA BITS SIZE: {}", distance_extra_bits_size);
                            let extra_bits_value: u16 =
                                bits.next_data_as_u16(distance_extra_bits_size as usize);
                            distance =
                                DISTANCE_OFFSET_BASE[distance_code as usize] + extra_bits_value;
                            println!("DISTANCE: {}", distance);
                        }

                        let index_start = output.len() - 1 - (distance as usize);
                        let index_end = index_start + (decoded as usize);
                        let slice: Vec<u8> = Vec::from(&output[index_start..index_end]);
                        output.extend_from_slice(slice.as_slice());
                    }
                }
            }

            if BlockType::Final == bfinal {
                break;
            }
        }

        if let Some(ref name_bytes) = file_name {
            let name_string: String = name_bytes
                .iter()
                .take_while(|b| **b != 0x00)
                .map(|b| *b as char)
                .collect();
            let path = Path::new(&name_string);
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(reason) => panic!("{}", reason),
            };

            if let Err(reason) = file.write_all(output.as_slice()) {
                panic!("{}", reason);
            };
        }

        // End of block
        let mut crc32: u32 = bits.next_byte_reversed() as u32;
        crc32 = ((bits.next_byte_reversed() as u32) << 8) + crc32;
        crc32 = ((bits.next_byte_reversed() as u32) << 16) + crc32;
        crc32 = ((bits.next_byte_reversed() as u32) << 24) + crc32;
        println!("CRC32   : {:04x}", crc32);

        let mut i_size: u32 = bits.next_byte_reversed() as u32;
        i_size = ((bits.next_byte_reversed() as u32) << 8) + i_size;
        i_size = ((bits.next_byte_reversed() as u32) << 16) + i_size;
        i_size = ((bits.next_byte_reversed() as u32) << 24) + i_size;
        i_size = i_size % 0b10000000_00000000_00000000_00000000u32; // isize mod 2^32
        println!("ISIZE   : {:04x}", i_size);
    }
}
