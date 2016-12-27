// TODO: Color Emphasis
// TODO: Sprite overflow
// TODO: Sprite #0 Hit
// TODO: Sprites (at all)

// Generate memory Controller trait for PPU
make_controller!();

#[derive(Default)]
pub struct PPU {
    /// [PPUCTRL:7] NMI Enable
    nmi_enable: bool,

    /// [PPUCTRL:5] Sprite 8x16 (1) or 8x8 (0)
    sprite_16: bool,

    /// [PPUCTRL:4] Background Pattern Table Select (0 = $0000, 1 = $1000)
    background_pattern_table_select: bool,

    /// [PPUCTRL:3] Sprite Pattern Table Select (0 = $0000, 1 = $1000)
    sprite_pattern_table_select: bool,

    /// [PPUCTRL:2] RAM Address Increment (0 = +1, 1 = +32)
    ram_address_increment: bool,

    /// [PPUCTRL:0..1] Base Nametable Address (2-bit)
    base_nametable_address: u8,

    /// [PPUMASK:0] Monochrome
    monochrome: bool,

    /// [PPUMASK:1] Show Background in leftmost 8 pixels
    background_leftmost_enable: bool,

    /// [PPUMASK:2] Show Sprites in leftmost 8 pixels
    sprite_leftmost_enable: bool,

    /// [PPUMASK:3] Show Background
    background_enable: bool,

    /// [PPUMASK:4] Show Sprites
    sprite_enable: bool,

    // TODO: Change name when we have another one
    /// Address latch for PPUSCROLL and PPUADDR
    latch: bool,

    /// In V-Blank (outwords facing flag)
    vblank: bool,

    /// Current scanline being rendered (261 = -1)
    line: u16,

    /// Even/Odd frame (toggled each frame; regardless if rendering is enabled or not)
    frame_odd: bool,

    /// Dot counter for the current scanline
    dots: u16,

    /// Current V-RAM address (15 bits)
    v: u16,

    /// Current Fine X Scroll value
    x: u8,

    /// Temporary V-RAM address (15 bits); more directly controlled by PPU registers
    t: u16,
}

impl PPU {
    pub fn reset(&mut self) {
        self.background_pattern_table_select = false;
        self.base_nametable_address = 0;
        self.sprite_16 = false;
        self.sprite_pattern_table_select = false;
        self.ram_address_increment = false;
        self.nmi_enable = false;

        self.vblank = false;

        self.latch = false;

        self.line = 261;  // -1 (pre-render scanline)
        self.frame_odd = false;
        self.dots = 0;
        self.v = 0;
        self.t = 0;
        self.x = 0;
    }

    pub fn step(&mut self, c: &mut Controller) {
        if self.line == 261 {
            // Pre-render (-1 / 261)
            if self.dots == 1 {
                // TODO: Clear: V-Blank
                // TODO: Clear: Sprite Overflow
                // TODO: Clear: Sprite 0 Hit
            } else if self.dots >= 280 && self.dots <= 304 {
                // TODO: Initilize: Vy to Ty
            }
        } else if self.line >= 240 && self.line <= 260 {
            // In V-Blank; do mostly nothing
            if self.line == 241 && self.dots == 1 {
                // Set V-Blank
                self.vblank = true;

                // TODO: Raise NMI (if enabled)
                // TODO: Signal screen refresh (to front-end)
            }
        }

        if (self.line == 261 || self.line <= 239) && self.dots <= 257 && self.dots > 0 {
            // Visible (0 ... 239) and Pre-render (-1 / 261)

            // Fetch: Nametable (NT) at dot 1,9,17,...,249

            // TODO: Fetch: Attribute (AT) at dot 3,11,19,...,251
            // TODO: Fetch: Background Tile Low at dot 5,13,21,...,253
            // TODO: Fetch: Background Tile High at dot 7,15,23,...,255

            // TODO: Increment: Vx at dots 8,16,...,64,...,248
            // TODO: Increment: Vy at dot 256
            // TODO: Initilize: Vx to Tx at dot 257
        }

        // Increment dot counter
        self.dots += 1;

        // Increment line counter; handle end-of-*
        if self.dots >= 341 {
            // End of scanline
            self.line += 1;
            self.dots = 0;

            if self.line >= 262 {
                // End of screen
                self.line = 0;
                self.frame_odd = !self.frame_odd;

                // If this next frame is an _odd_ frame (and rendering is enabled);
                // skip the idle dot of the first scanline
                // TODO: IFF rendering enabled
                if self.frame_odd {
                    self.dots += 1;
                }
            }
        }
    }

    pub fn read(&mut self, c: &mut Controller, address: u16) -> u8 {
        match address {
            // TODO: Least significant bits previously written into a PPU register (0..3)
            0x2002 => {
                let r = (self.vblank as u8) << 7;

                // Reading the status register will clear `vblank` and also
                // the address latch used by PPUSCROLL and PPUADDR.
                self.vblank = false;
                self.latch = false;

                r
            }

            _ => {
                panic!("PPU::read received unmapped address: ${:04X}", address);
            }
        }
    }

    pub fn write(&mut self, c: &mut Controller, address: u16, value: u8) {
        match address {
            0x2000 => {
                self.nmi_enable = value & 0x80 != 0;
                self.sprite_16 = value & 0x20 != 0;
                self.background_pattern_table_select = value & 0x10 != 0;
                self.sprite_pattern_table_select = value & 0x08 != 0;
                self.ram_address_increment = value & 0x04 != 0;
                self.base_nametable_address = value & 0x03;

                if value & 0x40 != 0 {
                    // Entering slave mode .. the hell?
                    panic!("unimplemented: NES PPU slave mode (this shorts the EXT circuit on a \
                            real NES so what do you want us to do here)");
                }
            }

            0x2001 => {
                self.sprite_enable = value & 0x10 != 0;
                self.background_enable = value & 0x08 != 0;
                self.sprite_leftmost_enable = value & 0x04 != 0;
                self.background_leftmost_enable = value & 0x02 != 0;
                self.monochrome = value & 0x01 != 0;
            }

            0x2005 => {
                if self.latch {
                    self.t &= !0x73E0;
                } else {
                    self.t &= !0x1F;
                    // 3..7 -> t:0..4
                    self.t |= ((value >> 3) & 0x1F) as u16;
                    // 0..2 -> x
                    self.x = value & 0x7;
                }

                self.latch = !self.latch;
            }

            0x2006 => {
                if self.latch {
                    self.t &= !0xFF;
                    self.t |= value as u16;

                    self.v = self.t;
                } else {
                    self.t &= !0xFF00;
                    self.t |= ((value as u16) << 8) & 0x3F00;
                }

                self.latch = !self.latch;
            }

            0x2007 => {
                // Write VRAM
                c.write(self.v, value);

                // Increment "Current" VRAM Address
                // TODO: Should T change at all here?
                // TODO: Check through math and make sure this does indeed handling all the
                //       weirdness that is V
                if self.ram_address_increment {
                    self.v += 32;
                    self.t += 32;
                } else {
                    self.v += 1;
                    self.t += 1;
                }
            }

            _ => {
                panic!("PPU::write received unmapped address: ${:04X} with value ${:02X}",
                       address,
                       value);
            }
        }
    }
}
