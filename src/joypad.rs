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

    pub fn read(&self) {
        let mut value = 0x3F
            + if self.select_direction { -0x10 } else { 0x00 }
            + if self.select_action { -0x20 } else { 0x00 };

        // TODO これはあったほうが良い？
        if self.select_direction && self.select_action {
            continue;
        }

        if self.select_direction {
            value = value
                + if self.right { -0x01 } else { 0x00 }
                + if self.left { -0x02 } else { 0x00 }
                + if self.up { -0x04 } else { 0x00 }
                + if self.down { -0x08 } else { 0x00 }
        }

        if self.select_action {
            value = value
                + if self.a { -0x01 } else { 0x00 }
                + if self.b { -0x02 } else { 0x00 }
                + if self.select { -0x04 } else { 0x00 }
                + if self.start { -0x08 } else { 0x00 }
        }
    }
}
