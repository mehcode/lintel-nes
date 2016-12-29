#[derive(Default)]
pub struct APU {
    ch1_length: u8,
    cycles: u32,
}

impl APU {
    pub fn reset(&mut self) {
        self.ch1_length = 0;
    }

    pub fn step(&mut self) {
        self.cycles += 1;

        if self.cycles == 14913 {
            self.step_length();
        } else if self.cycles == 29829 {
            self.step_length();
        } else if self.cycles == 29830 {
            self.cycles = 0;
        }
    }

    pub fn step_length(&mut self) {
        if self.ch1_length > 0 {
            self.ch1_length -= 1;
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x4015 => (if self.ch1_length > 0 { 1 } else { 0 }),

            _ => {
                // warn!("unhandled read at ${:04X}", address);

                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // 0x4003 => {
            //     let length_i = value >> 3;
            //     if length_i != 3 && length_i != 0 {
            //         panic!("unhandled length index: {}", length_i);
            //     }
            //
            //     if length_i == 0 {
            //         self.ch1_length = 10;
            //     } else if length_i == 3 {
            //         self.ch1_length = 2;
            //     }
            // }
            _ => {
                // warn!("unhandled write at ${:04X} with ${:02X} ({})",
                //       address,
                //       value,
                //       value);
            }
        }
    }
}
