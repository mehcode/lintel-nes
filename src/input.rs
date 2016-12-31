use sdl2::keyboard::Scancode;

// TODO: Remove SDL2 in favor of something more generic
// TODO: Definitely need to figure out controller/keyboard mapping

#[derive(Default)]
pub struct Input {
    // State variables for "live" state
    state_p1_a: bool,
    state_p1_b: bool,
    state_p1_select: bool,
    state_p1_start: bool,
    state_p1_up: bool,
    state_p1_down: bool,
    state_p1_left: bool,
    state_p1_right: bool,

    /// Buffer of P1
    /// bit:    0     1     2     3     4     5     6     7
    /// button: A     B  Select Start  Up   Down  Left  Right
    buffer_1: u8,

    /// Buffer of P2 (same structure as P1)
    buffer_2: u8,

    /// Reload; when true any attempt to read will reload state registers (before the read).
    ///     After reload; reload is set to false unless strobe is true.
    reload: bool,

    /// Strobe; when true, reload will not stop.
    strobe: bool,
}

impl Input {
    fn on_key(&mut self, scancode: Scancode, pressed: bool) {
        match scancode {
            Scancode::Z => {
                self.state_p1_a = pressed;
            }

            Scancode::X => {
                self.state_p1_b = pressed;
            }

            Scancode::LShift => {
                self.state_p1_select = pressed;
            }

            Scancode::Return => {
                self.state_p1_start = pressed;
            }

            Scancode::Up => {
                self.state_p1_up = pressed;
            }

            Scancode::Down => {
                self.state_p1_down = pressed;
            }

            Scancode::Left => {
                self.state_p1_left = pressed;
            }

            Scancode::Right => {
                self.state_p1_right = pressed;
            }

            _ => {}
        }
    }

    pub fn on_key_down(&mut self, scancode: Scancode) {
        self.on_key(scancode, true);

        if self.strobe {
            self.reload();
        }
    }

    pub fn on_key_up(&mut self, scancode: Scancode) {
        self.on_key(scancode, false);

        if self.strobe {
            self.reload();
        }
    }

    pub fn reset(&mut self) {
        self.strobe = false;
        self.reload = false;

        self.buffer_1 = 0;
        self.buffer_2 = 0;

        self.state_p1_a = false;
        self.state_p1_b = false;
        self.state_p1_select = false;
        self.state_p1_start = false;
        self.state_p1_up = false;
        self.state_p1_down = false;
        self.state_p1_left = false;
        self.state_p1_right = false;
    }

    fn reload(&mut self) {
        self.buffer_1 =
            (self.state_p1_a as u8) | ((self.state_p1_b as u8) << 1) |
            ((self.state_p1_select as u8) << 2) | ((self.state_p1_start as u8) << 3) |
            ((self.state_p1_up as u8) << 4) |
            ((self.state_p1_down as u8) << 5) | ((self.state_p1_left as u8) << 6) |
            ((self.state_p1_right as u8) << 7);

        self.buffer_2 = 0;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if self.strobe {
            self.reload();
        }

        match address {
            0x4016 => {
                let r = self.buffer_1 & 0x1;
                self.buffer_1 >>= 1;

                r
            }

            0x4017 => {
                let r = self.buffer_2 & 0x1;
                self.buffer_2 >>= 1;

                r
            }

            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address == 0x4016 {
            if value & 0x1 != 0 {
                // Strobe On
                self.strobe = true;
                self.reload();
            } else {
                // Strobe Off
                self.strobe = false;
            }
        }
    }
}