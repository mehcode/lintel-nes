use std::vec::Vec;

// Generate memory Controller trait for PPU
make_controller!();

#[derive(Default)]
pub struct PPU {
    /// Video RAM — 2 KiB
    vram: Vec<u8>,

    /// Current scanline being rendered (261 = -1)
    line: u16,

    /// Even/Odd frame (toggled each frame; regardless if rendering is enabled or not)
    frame_odd: bool,

    /// Dot counter for the current scanline
    dots: u16,

    /// Current V-RAM address (15 bits)
    v: u16,

    /// Temporary V-RAM address (15 bits); more directly controlled by PPU registers
    t: u16,
}

impl PPU {
    pub fn reset(&mut self) {
        self.vram.clear();
        self.vram.resize(2 * 1024, 0);

        self.line = 261;  // -1 (pre-render scanline)
        self.frame_odd = false;
        self.dots = 0;
        self.v = 0;
        self.t = 0;
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
                // TODO: Set V-Blank
                // TODO: Raise NMI (if enabled)
                // TODO: Signal screen refresh (to front-end)
            }
        }

        if (self.line == 261 || self.line <= 239) && self.dots <= 257 && self.dots > 0 {
            // Visible (0 ... 239) and Pre-render (-1 / 261)

            // Fetch: Nametable (NT) at dot 1,9,17,...,249
            // (self.v & 0xFFF)

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
}
