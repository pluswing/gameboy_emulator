pub struct Joypad {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,

    pub select_direction: bool,
    pub select_action: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            a: false,
            b: false,
            start: false,
            select: false,

            select_direction: true,
            select_action: true,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.select_direction = (value & 0x10) == 0;
        self.select_action = (value & 0x20) == 0;
    }

    pub fn read(&self) -> u8 {
        let mut value = 0xC0
            | if self.select_direction { 0x00 } else { 0x10 }
            | if self.select_action { 0x00 } else { 0x20 };

        if (self.select_direction && self.select_action)
            | (!self.select_direction && !self.select_action)
        {
            return value | 0x0F;
        }

        if self.select_direction {
            value = value
                | if self.right { 0x00 } else { 0x01 }
                | if self.left { 0x00 } else { 0x02 }
                | if self.up { 0x00 } else { 0x04 }
                | if self.down { 0x00 } else { 0x08 }
        }

        if self.select_action {
            value = value
                | if self.a { 0x00 } else { 0x01 }
                | if self.b { 0x00 } else { 0x02 }
                | if self.select { 0x00 } else { 0x04 }
                | if self.start { 0x00 } else { 0x08 }
        }

        value
    }
}
