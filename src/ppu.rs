use std::vec::Vec;

// TODO: Color Emphasis
// TODO: Sprite overflow
// TODO: Sprite #0 Hit
// TODO: Sprites (at all)

// Generate memory Controller trait for PPU
make_controller!();

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

const PALETTE: [(u8, u8, u8); 0x40] = [(0x65, 0x65, 0x65),
                                       (0x00, 0x12, 0x7D),
                                       (0x18, 0x00, 0x8E),
                                       (0x36, 0x00, 0x82),
                                       (0x56, 0x00, 0x5D),
                                       (0x5A, 0x00, 0x18),
                                       (0x4F, 0x05, 0x00),
                                       (0x38, 0x19, 0x00),
                                       (0x1D, 0x31, 0x00),
                                       (0x00, 0x3D, 0x00),
                                       (0x00, 0x41, 0x00),
                                       (0x00, 0x3B, 0x17),
                                       (0x00, 0x2E, 0x55),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00),
                                       (0xAF, 0xAF, 0xAF),
                                       (0x19, 0x4E, 0xC8),
                                       (0x47, 0x2F, 0xE3),
                                       (0x6B, 0x1F, 0xD7),
                                       (0x93, 0x1B, 0xAE),
                                       (0x9E, 0x1A, 0x5E),
                                       (0x97, 0x32, 0x00),
                                       (0x7B, 0x4B, 0x00),
                                       (0x5B, 0x67, 0x00),
                                       (0x26, 0x7A, 0x00),
                                       (0x00, 0x82, 0x00),
                                       (0x00, 0x7A, 0x3E),
                                       (0x00, 0x6E, 0x8A),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00),
                                       (0xFF, 0xFF, 0xFF),
                                       (0x64, 0xA9, 0xFF),
                                       (0x8E, 0x89, 0xFF),
                                       (0xB6, 0x76, 0xFF),
                                       (0xE0, 0x6F, 0xFF),
                                       (0xEF, 0x6C, 0xC4),
                                       (0xF0, 0x80, 0x6A),
                                       (0xD8, 0x98, 0x2C),
                                       (0xB9, 0xB4, 0x0A),
                                       (0x83, 0xCB, 0x0C),
                                       (0x5B, 0xD6, 0x3F),
                                       (0x4A, 0xD1, 0x7E),
                                       (0x4D, 0xC7, 0xCB),
                                       (0x4C, 0x4C, 0x4C),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00),
                                       (0xFF, 0xFF, 0xFF),
                                       (0xC7, 0xE5, 0xFF),
                                       (0xD9, 0xD9, 0xFF),
                                       (0xE9, 0xD1, 0xFF),
                                       (0xF9, 0xCE, 0xFF),
                                       (0xFF, 0xCC, 0xF1),
                                       (0xFF, 0xD4, 0xCB),
                                       (0xF8, 0xDF, 0xB1),
                                       (0xED, 0xEA, 0xA4),
                                       (0xD6, 0xF4, 0xA4),
                                       (0xC5, 0xF8, 0xB8),
                                       (0xBE, 0xF6, 0xD3),
                                       (0xBF, 0xF1, 0xF1),
                                       (0xB9, 0xB9, 0xB9),
                                       (0x00, 0x00, 0x00),
                                       (0x00, 0x00, 0x00)];

#[derive(Default)]
pub struct PPU {
    /// FrameBuffer
    pub framebuffer: Vec<u8>,

    /// [PPUCTRL:7] NMI Enable
    pub nmi_enable: bool,

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

    /// In V-Blank (outwords facing flag)
    pub vblank: bool,

    /// Current scanline being rendered (261 = -1)
    line: u16,

    /// Even/Odd frame (toggled each frame; regardless if rendering is enabled or not)
    frame_odd: bool,

    /// Dot counter for the current scanline
    dots: u16,

    /// Current OAM address
    pub oam_address: u8,

    /// Object Attribute Memory (OAM) ~ 256 bytes
    oam: Vec<u8>,

    /// "Current" V-RAM address (15 bits)
    /// This internal register is used for everything during rendering. And during read/write
    /// of VRAM by the CPU. Don't go changing this during rendering unless you want mad fun.
    ///
    /// yyy NN YYYYY XXXXX
    /// ||| || ||||| +++++-- coarse X scroll
    /// ||| || +++++-------- coarse Y scroll
    /// ||| ++-------------- nametable select
    /// +++----------------- fine Y scroll
    ///
    v: u16,

    /// "Current" Fine X Scroll value
    x: u8,

    /// "Temporary" V-RAM address (15 bits)
    /// This internal register is effectively used for _nothing_ except for reloading v
    /// at a set interval. This is the register most directly affected by the CPU. Yes, I meant
    /// "most directly". And yes, this is somewhat convoluted. My kingdom for SCX as in Gameboy.
    t: u16,

    /// First/second write toggle used in $2005, $2006, and reset in $2002.
    w: bool,

    /// "Next" Nametable byte in use by the tile fetch routine
    nx_nametable: u8,

    /// "Next" Attribute byte (color bits 2-3 of tile data) in use by the tile fetch routine
    nx_attribute: u8,

    /// "Next" Tile Bitmap Lo (color bit 0 of tile data) in use by the tile fetch routine
    nx_tile_lo: u8,

    /// "Next" Tile Bitmap Hi (color bit 1 of tile data) in use by the tile fetch routine
    nx_tile_hi: u8,

    /// "Current" Tile Data in use by the rendering pipeline.
    cur_tile: u64,
}

impl PPU {
    pub fn reset(&mut self) {
        self.framebuffer.clear();
        self.framebuffer.resize(WIDTH * HEIGHT * 4, 0);

        self.background_pattern_table_select = false;
        self.base_nametable_address = 0;
        self.sprite_16 = false;
        self.sprite_pattern_table_select = false;
        self.ram_address_increment = false;
        self.nmi_enable = false;

        self.vblank = false;

        self.w = false;

        self.oam.clear();
        self.oam.resize(256, 0);
        self.oam_address = 0;

        self.line = 261;  // -1 (pre-render scanline)
        self.frame_odd = false;
        self.dots = 0;

        // NOTE: Short 1-letter names are used here to correspond to the names popularized
        //       by "loopy"; who was the first person to fully document the scrolling
        //       behavior of the PPU.
        self.v = 0;
        self.t = 0;
        self.x = 0;
        self.w = false;

        self.nx_nametable = 0;
        self.nx_attribute = 0;
        self.nx_tile_lo = 0;
        self.nx_tile_hi = 0;

        self.cur_tile = 0;
    }

    fn fetch_nametable(&mut self, c: &mut Controller) {
        let address = 0x2000 | (self.v & 0xFFF);
        self.nx_nametable = c.read(address);
    }

    fn fetch_attribute(&mut self, c: &mut Controller) {
        // Take "coarse" X and Y
        let coarse_x = self.v & 0x1F;
        let coarse_y = (self.v >> 5) & 0x1F;

        // Divide each by 4 (4 tiles per attribute byte)
        // Combine and copy the nametable from V
        let address = (self.v & 0x0C00) | (coarse_x >> 2) | ((coarse_y >> 2) << 3);

        // Fetch macro attribute byte
        let at = c.read(0x23C0 | address);

        // Shift to reduce scope to single 2x2 tile area
        // NOTE: This is then shifted left 2 to make OR'ing easy as what we
        //       call an "attribute" on the NES background is just bits 2-3
        //       of the palette index
        let shift = ((coarse_y & 2) << 1) | (coarse_x & 2);
        self.nx_attribute = ((at >> shift) & 3) << 2;
    }

    fn fetch_tile_lo(&mut self, c: &mut Controller) {
        // Addresses are formed as follows:

        // DCBA98 76543210
        // ---------------
        // 0HRRRR CCCCPTTT
        // |||||| |||||+++- T: Fine Y offset, the row number within a tile
        // |||||| ||||+---- P: Bit plane (0: "lower"; 1: "upper")
        // |||||| ++++----- C: Tile column
        // ||++++---------- R: Tile row
        // |+-------------- H: Half of sprite table (0: "left"; 1: "right")
        // +--------------- 0: Pattern table is at $0000-$1FFF

        let address_base = if self.background_pattern_table_select {
            0x1000
        } else {
            0x0000
        };

        let address = address_base | ((self.v >> 12) & 7) | ((self.nx_nametable as u16) << 4);
        self.nx_tile_lo = c.read(address);
    }

    fn fetch_tile_hi(&mut self, c: &mut Controller) {
        // TODO: Fix logic duplication with `fetch_tile_lo`
        let address_base = if self.background_pattern_table_select {
            0x1000
        } else {
            0x0000
        };

        let address = address_base | ((self.v >> 12) & 7) | ((self.nx_nametable as u16) << 4);
        self.nx_tile_hi = c.read(address | 8);
    }

    fn increment_horz_v(&mut self) {
        let mut coarse_x = self.v & 0x1F;

        if coarse_x == 31 {
            // Coarse X overflows by toggling the horizontal nametable
            coarse_x = 0;
            self.v ^= 0x0400;
        } else {
            coarse_x += 1;
        }

        self.v = (self.v & !0x1F) | coarse_x;
    }

    fn increment_vert_v(&mut self) {
        // TODO: Understand this
        if self.v & 0x7000 != 0x7000 {
            // Increment "fine Y"
            self.v += 0x1000;
        } else {
            // Set "fine Y" to 0
            self.v &= 0x8FFF;

            // Let y = "coarse Y"
            let mut y = (self.v & 0x3E0) >> 5;
            if y == 29 {
                // Set "coarse Y" to 0
                y = 0;
                // Switch vertical nametable
                self.v ^= 0x0800;
            } else if y == 31 {
                // Set "coarse Y" to 0; don't switch nametable
                y = 0;
            } else {
                // Increment "coarse Y"
                y += 1;
            }

            // Put "coarse Y" back into v
            self.v = (self.v & 0xFC1F) | (y << 5);
        }
    }

    fn reload_horz_v(&mut self) {
        self.v = (self.v & !0x41F) | (self.t & 0x41F);
    }

    fn reload_vert_v(&mut self) {
        self.v = (self.v & !0x7BE0) | (self.t & 0x7BE0);
    }

    fn render_pixel(&mut self, c: &mut Controller) {
        let offset = (self.line as usize * WIDTH + (self.dots as usize - 1)) * 4;
        let (mut r, mut g, mut b) = (0, 0, 0);

        if self.background_enable {
            let palette_index = ((self.cur_tile >> 32) >> ((7 - self.x) << 2)) & 0x0F;
            let color = c.read(0x3F00 + palette_index as u16);

            // TODO(rust): Is there a way to assign directly to the mutable tuple?
            let (r1, g1, b1) = PALETTE[color as usize];
            r = r1;
            g = g1;
            b = b1;
        }

        self.framebuffer[offset] = b;
        self.framebuffer[offset + 1] = g;
        self.framebuffer[offset + 2] = r;
        self.framebuffer[offset + 3] = 0xFF;
    }

    pub fn step(&mut self, c: &mut Controller) {
        let is_line_render = self.line <= 239;
        let is_line_active = self.line == 261 || is_line_render;
        let is_dot_render = self.dots >= 1 && self.dots <= 256;
        let is_dot_prefetch = self.dots >= 321 && self.dots <= 336;
        let is_dot_fetch = is_dot_render || is_dot_prefetch;
        let is_dot_shift = (self.dots >= 2 && self.dots <= 257) ||
                           (self.dots >= 322 && self.dots <= 337);


        // Render
        if is_line_render && is_dot_render {
            self.render_pixel(c);
        }

        // Background: Fetch
        if is_line_active {
            if is_dot_shift && self.background_enable {
                self.cur_tile <<= 4;
            }

            if is_dot_fetch && self.background_enable {
                // Every other dot; another step in the fetch process
                match self.dots % 8 {
                    1 => self.fetch_nametable(c),
                    3 => self.fetch_attribute(c),
                    5 => self.fetch_tile_lo(c),
                    7 => self.fetch_tile_hi(c),

                    0 => {
                        // Compute tile data
                        let mut data: u32 = 0;
                        for _ in 0..8 {
                            let p1 = (self.nx_tile_lo & 0x80) >> 7;
                            let p2 = (self.nx_tile_hi & 0x80) >> 6;

                            self.nx_tile_lo <<= 1;
                            self.nx_tile_hi <<= 1;

                            data <<= 4;
                            data |= (self.nx_attribute | p1 | p2) as u32;
                        }

                        self.cur_tile |= data as u64;

                        // Increment horz(v)
                        self.increment_horz_v();
                    }

                    _ => {
                        // Do nothing
                    }
                }
            }

            if self.dots == 256 && self.background_enable {
                // Increment vert(v)
                self.increment_vert_v();
            }

            if self.dots == 257 && self.background_enable {
                // Reload horz(v)
                self.reload_horz_v();
            }
        }

        // Pre-Render (261 / -1)
        if self.line == 261 {
            if self.dots == 1 {
                // Clear: V-Blank
                self.vblank = false;

                // TODO: Clear: Sprite Overflow
                // TODO: Clear: Sprite 0 Hit
            }

            if self.dots >= 280 && self.dots <= 304 && self.background_enable {
                self.reload_vert_v();
            }
        }

        if self.line == 241 && self.dots == 1 {
            // Set V-Blank on the 2nd dot of the 2nd line in V-Blank
            self.vblank = true;

            // TODO: Raise NMI (if enabled)
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
                if self.frame_odd && (self.background_enable || self.sprite_enable) {
                    self.dots += 1;
                }
            }
        }

        // if (self.line == 261 || self.line <= 239) && self.dots <= 257 && self.dots > 0 {
        //     // Visible (0 ... 239) and Pre-render (-1 / 261)
        //
        //     // Get pixel value at current dot
        //     if self.dots < 257 && self.line < 240 {
        //         if self.background_enable {
        //             let palette_i = ((self.tile_lo >> 7) | ((self.tile_hi >> 7) << 1)) | self.at;
        //             let palette_c = c.read(0x3F00 + palette_i as u16);
        //             let (r, g, b) = PALETTE[palette_c as usize];
        //
        //             // if x < 16 && self.line < 16 {
        //             //     // This should be all black!
        //             //     warn!("[dots: {}, line: {}] palette_i: ${:02X} palette_c: ${:02X}",
        //             //           self.dots,
        //             //           self.line,
        //             //           palette_i,
        //             //           palette_c);
        //             // }
        //
        //             let fb_offset = (self.line as usize * WIDTH + (self.dots as usize - 1)) * 4;
        //             self.framebuffer[fb_offset] = b;
        //             self.framebuffer[fb_offset + 1] = g;
        //             self.framebuffer[fb_offset + 2] = r;
        //             self.framebuffer[fb_offset + 3] = 0xFF;
        //
        //             // Shift tile and attribute byte
        //             self.tile_hi <<= 1;
        //             self.tile_lo <<= 1;
        //         } else {
        //             let fb_offset = (self.line as usize * WIDTH + (self.dots as usize - 1)) * 4;
        //
        //             self.framebuffer[fb_offset] = 0;
        //             self.framebuffer[fb_offset + 1] = 0;
        //             self.framebuffer[fb_offset + 2] = 0;
        //             self.framebuffer[fb_offset + 3] = 0xFF;
        //         }
        //     }
        //
        //     // Prepare next tile for rendering
        //     if self.background_enable {
        //         let rem = self.dots % 8;
        //         let nt_address = (self.v & 0xFFF) | 0x2000;
        //         let tile_base = if self.background_pattern_table_select {
        //             0x1000
        //         } else {
        //             0x0000
        //         };
        //
        //         if rem == 1 {
        //             // Fetch: Nametable (NT) at dot 1,9,17,...,249
        //             self.nt = c.read(nt_address);
        //             // if self.line < 8 {
        //             // warn!("Nametable ({}:{}): ${:04X} -> ${:02X}",
        //             //       self.line,
        //             //       self.dots,
        //             //       nt_address,
        //             //       self.nt);
        //             // }
        //         }
        //
        //         if rem == 3 {
        //             // Fetch: Attribute (AT) at dot 3,11,19,...,251
        //             // TODO: Understand
        //             let at_address = 0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) |
        //                              ((self.v >> 2) & 0x07);
        //
        //             let shift = ((self.v >> 4) & 4) | (self.v & 2);
        //             self.at_next = ((c.read(at_address) >> shift) & 3) << 2;
        //         }
        //
        //         if rem == 5 {
        //             // Fetch: Background Tile Low at dot 5,13,21,...,253
        //             let tile_address = ((self.v >> 12) & 0x7) | tile_base | ((self.nt as u16) << 4);
        //             self.tile_next_lo = c.read(tile_address);
        //             // if self.line < 8 {
        //             //     warn!("Tile Lo ({}:{}): [tile: {}, fine y: {}] ${:04X} -> ${:02X}",
        //             //           self.line,
        //             //           self.dots,
        //             //           self.nt,
        //             //           ((self.v >> 12) & 0x7),
        //             //           tile_address,
        //             //           self.tile_next_lo);
        //             // }
        //         }
        //
        //         if rem == 7 {
        //             // Fetch: Background Tile High at dot 7,15,23,...,255
        //             let tile_address = ((self.v >> 12) & 0x7) | 0x8 | tile_base |
        //                                ((self.nt as u16) << 4);
        //             self.tile_next_hi = c.read(tile_address);
        //             // warn!("Tile ({}) Hi: ${:04X} -> ${:02X}",
        //             //       self.nt,
        //             //       tile_address,
        //             //       self.tile_next_hi);
        //         }
        //
        //         if rem == 0 && self.dots != 0 && self.dots <= 248 {
        //             // Reload shifts
        //             // info!("inc hori(v) [ dots: {} / line: {} ]", self.dots, self.line);
        //             self.tile_hi = self.tile_next_hi;
        //             self.tile_lo = self.tile_next_lo;
        //             self.at = self.at_next;
        //
        //             // Increment: Vx at dots 8,16,...,64,...,248
        //             // TODO: Understand this
        //             if self.v & 0x1F == 31 {
        //                 self.v &= 0xFFE0;     // coarse X = 0
        //                 self.v ^= 0x0400;     // switch horizontal nametable
        //             } else {
        //                 self.v += 1;          // increment coarse X
        //             }
        //         }
        //
        //         if self.dots == 256 {
        //             // Increment: Vy at dot 256
        //             // TODO: Understand this
        //
        //             if self.v & 0x7000 != 0x7000 {
        //                 // Increment "fine Y"
        //                 self.v += 0x1000;
        //             } else {
        //                 // Set "fine Y" to 0
        //                 self.v &= 0x8FFF;
        //
        //                 // Let y = "coarse Y"
        //                 let mut y = (self.v & 0x3E0) >> 5;
        //                 if y == 29 {
        //                     // Set "coarse Y" to 0
        //                     y = 0;
        //                     // Switch vertical nametable
        //                     self.v ^= 0x0800;
        //                 } else if y == 31 {
        //                     // Set "coarse Y" to 0; don't switch nametable
        //                     y = 0;
        //                 } else {
        //                     // Increment "coarse Y"
        //                     y += 1;
        //                 }
        //
        //                 // Put "coarse Y" back into v
        //                 self.v = (self.v & 0xFC1F) | (y << 5);
        //             }
        //         }
        //
        //         if self.dots == 257 {
        //             // info!("hori(v) = hori(t) [ dots: {} / line: {} ]",
        //             //       self.dots,
        //             //       self.line);
        //             // Initilize: Vx to Tx at dot 257
        //             // TODO: Understand this
        //             self.v &= !0x41F;
        //             self.v |= self.t & 0x41F;
        //         }
        //     }
        // }
        //
        // if self.line == 261 {
        //     // Pre-render (-1 / 261)
        //     if self.dots == 1 {
        //         // Clear: V-Blank
        //         self.vblank = false;
        //
        //         // TODO: Clear: Sprite Overflow
        //         // TODO: Clear: Sprite 0 Hit
        //     }
        //
        //     if self.dots >= 280 && self.dots <= 304 && self.background_enable {
        //         // Initilize: Vy to Ty
        //         // info!("vert(v) = vert(t) [ dots: {} / line: {} ]",
        //         //       self.dots,
        //         //       self.line);
        //
        //         self.v &= !0x7BE0;
        //         self.v |= self.t & 0x7BE0;
        //     }
        // }
        //
        // if self.line >= 240 && self.line <= 260 {
        //     // In V-Blank; do mostly nothing
        //     if self.line == 241 && self.dots == 1 {
        //         // Set V-Blank
        //         self.vblank = true;
        //
        //         // TODO: Raise NMI (if enabled)
        //         // TODO: Signal screen refresh (to front-end)
        //     }
        // }
        //
        // // Increment dot counter
        // self.dots += 1;
        //
        // // Increment line counter; handle end-of-*
        // if self.dots >= 341 {
        //     // End of scanline
        //     self.line += 1;
        //     self.dots = 0;
        //
        //     if self.line >= 262 {
        //         // End of screen
        //         self.line = 0;
        //         self.frame_odd = !self.frame_odd;
        //
        //         // If this next frame is an _odd_ frame (and rendering is enabled);
        //         // skip the idle dot of the first scanline
        //         if self.frame_odd && (self.background_enable || self.sprite_enable) {
        //             self.dots += 1;
        //         }
        //     }
        // }
    }

    pub fn read(&mut self, _: &mut Controller, address: u16) -> u8 {
        match address % 8 {
            // TODO: Least significant bits previously written into a PPU register (0..3)
            2 => {
                let r = (self.vblank as u8) << 7;

                // Reading the status register will clear `vblank` and also
                // the address latch used by PPUSCROLL and PPUADDR.
                self.vblank = false;
                self.w = false;

                r
            }

            _ => {
                warn!("PPU::read received unmapped address: ${:04X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, c: &mut Controller, address: u16, value: u8) {
        match address % 8 {
            0 => {
                self.nmi_enable = value & 0x80 != 0;
                self.sprite_16 = value & 0x20 != 0;
                self.background_pattern_table_select = value & 0x10 != 0;
                self.sprite_pattern_table_select = value & 0x08 != 0;
                self.ram_address_increment = value & 0x04 != 0;
                self.base_nametable_address = value & 0x03;

                if value & 0x40 != 0 {
                    // Entering slave mode .. the hell?
                    warn!("unimplemented: NES PPU slave mode (this shorts the EXT circuit on a \
                            real NES so what do you want us to do here)");
                }
            }

            1 => {
                self.sprite_enable = value & 0x10 != 0;
                self.background_enable = value & 0x08 != 0;
                self.sprite_leftmost_enable = value & 0x04 != 0;
                self.background_leftmost_enable = value & 0x02 != 0;
                self.monochrome = value & 0x01 != 0;
            }

            // [OAMADDR]: OAM address port
            3 => {
                self.oam_address = value;
            }

            // [OAMDATA]: OAM data port
            4 => {
                self.oam[self.oam_address as usize] = value;
                self.oam_address = self.oam_address.wrapping_add(1);
            }

            5 => {
                if self.w {
                    self.t &= !0x73E0;
                } else {
                    self.t &= !0x1F;
                    // 3..7 -> t:0..4
                    self.t |= ((value >> 3) & 0x1F) as u16;
                    // 0..2 -> x
                    self.x = value & 0x7;
                }

                self.w = !self.w;
            }

            6 => {
                if self.w {
                    self.t &= !0xFF;
                    self.t |= value as u16;

                    self.v = self.t;
                } else {
                    self.t &= !0xFF00;
                    self.t |= ((value as u16) << 8) & 0x3F00;
                }

                self.w = !self.w;
            }

            7 => {
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
                warn!("PPU::write received unmapped address: ${:04X} with value ${:02X}",
                      address,
                      value);
            }
        }
    }
}
