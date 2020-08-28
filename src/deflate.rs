use crate::huffman::Huffman;
use crate::lz77::LZ77;

#[derive(Debug)]
pub struct Deflate {
    lz77: LZ77,
}

impl Deflate {
    pub fn new(window_size: usize, dictionary_size: usize) -> Deflate {
        Deflate {
            lz77: LZ77::new(window_size, dictionary_size),
        }
    }

    pub fn compress<S>(&mut self, data: S) -> String
    where
        S: Sized + ToString,
    {
        self.lz77.encode(data);
        let mut huffman = Huffman::new();
        let lz77_vec: Vec<u8> = self.lz77.iter().map(|n| n.to_vec_u8()).flatten().collect();
        let lz77_string: String = lz77_vec.iter().map(|v| *v as char).collect();
        huffman.encode(&lz77_string)
    }

    pub fn decompress<S>(&self, data: S) -> String
    where
        S: Sized + ToString,
    {
        let mut huffman = Huffman::new();
        let _huffman_decoded = huffman.decode(&data);
        self.lz77.decode()
    }
}
