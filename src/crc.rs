pub struct CRC32 {
    /// Table of CRCs of all 8-bit messages.
    crc_table: [u32; 256],

    /// Flag: has the table been computed? Initially false.
    crc_table_computed: bool,
}

// CRC32 implementation from https://tools.ietf.org/html/rfc1952.html#page-11
impl CRC32 {
    pub fn new() -> CRC32 {
        CRC32 {
            crc_table: [0; 256],
            crc_table_computed: false,
        }
    }

    /// Make the table for a fast CRC.
    fn make_crc_table(&mut self) {
        for n in 0..256 {
            let mut c: u32 = n as u32;
            for _ in 0..8 {
                if c & 1 != 0 {
                    c = 0xedb88320 ^ (c >> 1);
                } else {
                    c = c >> 1;
                }
            }
            self.crc_table[n] = c;
        }
        self.crc_table_computed = true;
    }

    /*
       Update a running crc with the bytes buf[0..len-1] and return
     the updated crc. The crc should be initialized to zero. Pre- and
     post-conditioning (one's complement) is performed within this
     function so it shouldn't be done by the caller. Usage example:

       unsigned long crc = 0L;

       while (read_buffer(buffer, length) != EOF) {
         crc = update_crc(crc, buffer, length);
       }
       if (crc != original_crc) error();
    */
    pub fn update_crc(&mut self, crc: u32, buf: &[u8]) -> u32 {
        let mut c: u32 = crc ^ 0xffffffff;

        if !self.crc_table_computed {
            self.make_crc_table();
        }
        for n in 0..buf.len() {
            let xored = c ^ (buf[n] as u32);
            let byte = (xored & 0xff) as u8;
            c = self.crc_table[byte as usize] ^ (c >> 8);
        }
        return c ^ 0xffffffff;
    }

    /* Return the CRC of the bytes buf[0..len-1]. */
    pub fn crc(&mut self, buf: &[u8]) -> u32 {
        return self.update_crc(0u32, buf);
    }
}
