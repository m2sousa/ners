pub(super) struct PatternTables {
    left_table: [u8; 16384],
    right_table: [u8; 16384],
}

impl PatternTables {
    const PATTERN_DATA_SIZE: usize = 4096;

    pub fn new(data: &[u8]) -> Self {
        let left_table = Self::get_table_from_data(&data[0..Self::PATTERN_DATA_SIZE]);
        let right_table =
            Self::get_table_from_data(&data[Self::PATTERN_DATA_SIZE..2 * Self::PATTERN_DATA_SIZE]);

        PatternTables {
            left_table,
            right_table,
        }
    }

    fn get_table_from_data(data: &[u8]) -> [u8; 16384] {
        let mut pattern_table: [u8; 16384] = [0; 16384];

        for tile_idx in 0..256 {
            let tile_data = &data[tile_idx * 16..(tile_idx + 1) * 16];

            for row in 0..8 {
                let low_byte = tile_data[row];
                let high_byte = tile_data[row + 8];

                for col in 0..8 {
                    let bit_pos = 7 - col;

                    let low_bit = (low_byte >> bit_pos) & 1;
                    let high_bit = (high_byte >> bit_pos) & 1;

                    let pixel = (high_bit << 1) | low_bit;
                    let output_idx = tile_idx * 64 + row * 8 + col;

                    pattern_table[output_idx] = pixel;
                }
            }
        }

        pattern_table
    }
}

impl std::ops::Index<usize> for PatternTables {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.left_table,
            1 => &self.right_table,
            _ => panic!(
                "[ERR] Pattern table index {} out of bounds (must be 0 or 1).",
                index
            ),
        }
    }
}
