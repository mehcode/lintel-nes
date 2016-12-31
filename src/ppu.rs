use std::vec::Vec;

// TODO: Color Emphasis
// TODO: Sprite overflow
// TODO: Sprite #0 Hit
// TODO: Sprites (at all)

// Generate memory Controller trait for PPU
make_controller!();

pub struct Frame<'a> {
    // Pixel data
    pub data: &'a [u8],

    // Pixel pitch
    pub pitch: usize,

    // Width (in pixels)
    pub width: usize,

    // Height (in pixels)
    pub height: usize,
}

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
    /// Callback: Refresh (v-blank)
    on_refresh: Option<Box<FnMut(Frame) -> ()>>,

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

    /// Set to suppress normal V-Blank set during `step`
    supress_vblank: bool,

    /// Set when NMI occurs as the signal is delayed 1 dot
    nmi_timer: u8,

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

    /// "Current" Tile Data Hi in use by the rendering pipeline.
    /// NOTE: The upper bits are the current tile and the lower bits are
    ///       the next tile.
    cur_tile_hi: u16,

    /// "Current" Tile Data Lo in use by the rendering pipeline.
    /// NOTE: The upper bits are the current tile and the lower bits are
    ///       the next tile.
    cur_tile_lo: u16,

    /// "Current" Attribute Data in use by the rendering pipeline.
    /// NOTE: The upper bits are the current tile and the lower bits are
    ///       the next tile.
    cur_attribute: u16,
}

impl PPU {
    pub fn set_on_refresh(&mut self, callback: Box<FnMut(Frame) -> ()>) {
        self.on_refresh = Some(callback);
    }

    pub fn reset(&mut self) {
        self.framebuffer.clear();
        self.framebuffer.resize(WIDTH * HEIGHT * 4, 0);

        self.background_pattern_table_select = false;
        self.base_nametable_address = 0;
        self.sprite_16 = false;
        self.sprite_pattern_table_select = false;
        self.ram_address_increment = false;
        self.vblank = false;
        self.supress_vblank = false;

        self.nmi_enable = false;
        self.nmi_timer = 0;

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

        self.cur_tile_lo = 0;
        self.cur_tile_hi = 0;
        self.cur_attribute = 0;
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
        if self.v & 0x7000 != 0x7000 {
            // Increment "Fine Y" (row number of tile)
            self.v += 0x1000;
        } else {
            // "Fine Y" overflows into "Coarse Y" (line on screen)
            let mut coarse_y = (self.v & 0x3E0) >> 5;

            // Set "Fine Y" to 0
            self.v &= !0x7000;

            if coarse_y == 29 {
                // "Coarse Y" overflows by toggling to vertical
                // nametable (mirroring)
                coarse_y = 0;
                self.v ^= 0x0800;
            } else if coarse_y == 31 {
                // "Coarse Y" is out of bounds (because it was set
                // directly). It can be incremented till 31 in which
                // it is then set to 0 where it will not toggle the vertical
                // nametable.
                coarse_y = 0;
            } else {
                // Increment "Coarse Y"
                coarse_y += 1;
            }

            self.v = (self.v & !0x3E0) | (coarse_y << 5);
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
            let a = (self.cur_attribute >> 8) as u8;
            let p1 = (self.cur_tile_lo >> (15 - self.x)) as u8;
            let p2 = ((self.cur_tile_hi >> (15 - self.x)) as u8) << 1;
            let palette_index = (a | p1 | p2) & 0x0F;

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

    pub fn step(&mut self, c: &mut Controller, nmi_occurred: &mut bool) {
        // On ODD Frames; the 338th dot of line 261 (-1) is skipped if
        // rendering is enabled
        if self.line == 261 && self.dots == 338 && self.frame_odd &&
           (self.background_enable || self.sprite_enable) {
            self.dots += 1;
        }

        // Check for pending NMI
        if self.nmi_timer > 0 {
            self.nmi_timer -= 1;
            if self.nmi_timer == 0 {
                *nmi_occurred = true;
            }
        }

        // Render
        if self.line <= 239 && (1..257).contains(self.dots) {
            self.render_pixel(c);
        }

        // Background: Fetch
        if self.background_enable {
            if self.line == 261 || self.line <= 239 {
                // Each dot after a dot that was renderable; we should
                // shift the current tile data registers by 1
                if (2..258).contains(self.dots) || (322..338).contains(self.dots) {
                    self.cur_tile_lo <<= 1;
                    self.cur_tile_hi <<= 1;
                }

                if (1..257).contains(self.dots) || (321..337).contains(self.dots) {
                    match self.dots % 8 {
                        2 => self.fetch_nametable(c),
                        3 => self.fetch_attribute(c),
                        5 => self.fetch_tile_lo(c),
                        7 => self.fetch_tile_hi(c),

                        0 => {
                            // Reload shift registers
                            self.cur_tile_hi |= self.nx_tile_hi as u16;
                            self.cur_tile_lo |= self.nx_tile_lo as u16;

                            self.cur_attribute <<= 8;
                            self.cur_attribute |= self.nx_attribute as u16;

                            // Increment horz(v)
                            self.increment_horz_v();
                        }

                        _ => {}
                    }
                }

                // Increment `vert(v)`
                if self.dots == 256 {
                    self.increment_vert_v();
                }

                // Reload `horz(v)`
                if self.dots == 257 {
                    self.reload_horz_v();
                }
            }

            // On line 261 (-1); there is a short period where `vert(v)` is
            // repeatedly reloaded with `vert(t)`
            if self.line == 261 && (280..305).contains(self.dots) {
                self.reload_vert_v();
            }
        }

        // Clear V-Blank (and other PPU flags)
        if self.line == 261 && self.dots == 1 {
            self.vblank = false;
            self.frame_odd = !self.frame_odd;

            // TODO: Clear: Sprite Overflow
            // TODO: Clear: Sprite 0 Hit
        }

        // Set V-Blank on the 2nd dot of the 2nd line in V-Blank
        if self.line == 241 && self.dots == 1 {
            if !self.supress_vblank {
                self.vblank = true;

                if self.nmi_enable {
                    self.nmi_timer = 2;
                }

                // Trigger the front-end to refresh the scren
                if let Some(ref mut on_refresh) = self.on_refresh {
                    (on_refresh)(Frame {
                        data: &self.framebuffer,
                        width: WIDTH,
                        height: HEIGHT,
                        pitch: WIDTH * 4,
                    });
                }
            } else {
                self.supress_vblank = false;
            }
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
            }
        }
    }

    pub fn read(&mut self, _: &mut Controller, address: u16) -> u8 {
        match address % 8 {
            2 => {
                if self.line == 241 {
                    if self.dots == 1 {
                        // Reading $2002 exactly 1 dot _before_ V-Blank should
                        // be set
                        self.supress_vblank = true;
                    } else if self.dots <= 3 {
                        // Reading $2002 on or exactly 1 dot _after_ V-Blank
                        // is set results in the NMI not being fired
                        self.nmi_timer = 0;
                    }
                }

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
                let prev_nmi_enable = self.nmi_enable;

                self.nmi_enable = value & 0x80 != 0;
                self.sprite_16 = value & 0x20 != 0;
                self.background_pattern_table_select = value & 0x10 != 0;
                self.sprite_pattern_table_select = value & 0x08 != 0;
                self.ram_address_increment = value & 0x04 != 0;

                // Bits 0-1 affect base nametable address which is in bits 10-11 of T
                self.t = (self.t & !0xC00) | ((value as u16 & 0x03) << 10);

                if value & 0x40 != 0 {
                    // Entering slave mode .. the hell?
                    warn!("unimplemented: NES PPU slave mode (this shorts the EXT circuit on a \
                            real NES so what do you want us to do here)");
                }

                if !prev_nmi_enable && self.nmi_enable && self.vblank && self.dots != 1 {
                    // If NMI was disabled and is now enabled, AND; V-Blank is
                    // still set, trigger NMI again
                    // NOTE: Enabling NMI the exact PPU dot that NMI should go out
                    //       normally causes NMI to not go out at all
                    self.nmi_timer = 2;
                } else if prev_nmi_enable && !self.nmi_enable {
                    // Prevent NMI from occuring; it was just disabled
                    self.nmi_timer = 0;
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
                self.v += if self.ram_address_increment { 32 } else { 1 };
            }

            _ => {
                warn!("PPU::write received unmapped address: ${:04X} with value ${:02X}",
                      address,
                      value);
            }
        }
    }
}
