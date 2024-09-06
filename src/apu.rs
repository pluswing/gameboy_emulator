use sdl2::audio::AudioQueue;

pub struct APU {
    prev_div: u8,
    counter: u8,
    device: AudioQueue<f32>,

    pub global: Global,
    pub ch1: Ch1,
}

impl APU {
    pub fn new(device: AudioQueue<f32>) -> Self {
        Self {
            prev_div: 0,
            counter: 0,
            device,

            global: Global::new(),
            ch1: Ch1::new(),
        }
    }

    pub fn update(&mut self, div: u8) {
        let p = self.prev_div & 0x10;
        let c = div & 0x10;
        self.prev_div = div;
        // DIV-APUカウンタのビット4が1から0になるたびに処理を行う
        if !(p != 0 && c == 0) {
            return;
        }

        self.counter += 1;

        if self.counter % 2 == 0 {
            // TODO 音の長さ
        }

        if self.counter % 4 == 0 {
            // TODO CH1周波数スイープ
        }

        if self.counter == 8 {
            self.counter = 0;
            // TODO エンベロープ スイープ
        }

        // device.queue_audio(&wave)?;
    }
}

pub struct Global {
    // 0xFF26
    nr52: u8,
    // 0xFF25
    nr51: u8,
    // 0xFF24
    nr50: u8,
}

impl Global {
    pub fn new() -> Self {
        Self {
            nr52: 0xF1,
            nr51: 0xF3,
            nr50: 0x77,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF26 => {
                self.nr52 = value;
            }
            0xFF25 => {
                self.nr51 = value;
            }
            0xFF24 => {
                self.nr50 = value;
            }
            _ => panic!("should not reach"),
        }
    }

    pub fn power(&self) -> bool {
        self.nr52 & 0x80 != 0
    }

    pub fn ch4_power(&self) -> bool {
        self.nr52 & 0x08 != 0
    }
    pub fn ch3_power(&self) -> bool {
        self.nr52 & 0x04 != 0
    }
    pub fn ch2_power(&self) -> bool {
        self.nr52 & 0x02 != 0
    }
    pub fn ch1_power(&self) -> bool {
        self.nr52 & 0x01 != 0
    }

    // self.ch4_left = value & 0x80 != 0;
    // self.ch3_left = value & 0x40 != 0;
    // self.ch2_left = value & 0x20 != 0;
    // self.ch1_left = value & 0x10 != 0;
    // self.ch4_right = value & 0x08 != 0;
    // self.ch3_right = value & 0x04 != 0;
    // self.ch2_right = value & 0x02 != 0;
    // self.ch1_right = value & 0x01 != 0;

    // self.vin_left = value & 0x80 != 0;
    // self.left_volume = (value & 0x70) >> 4;
    // self.vin_right = value & 0x08 != 0;
    // self.right_volume = value & 0x07;

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0xFF26 => {
                (self.power() as u8) << 7
                    | (self.ch4_power() as u8) << 3
                    | (self.ch3_power() as u8) << 2
                    | (self.ch2_power() as u8) << 1
                    | (self.ch1_power() as u8) << 0
            }
            0xFF25 => self.nr51,
            0xFF24 => self.nr50,
            _ => panic!("should not reach"),
        }
    }
}

pub struct Ch1 {
    // 0xFF10
    nr10: u8,
    // 0xFF11
    nr11: u8,
    // 0xFF12
    nr12: u8,
    // 0xFF13
    nr13: u8,
    // 0xFF14
    nr14: u8,
}

impl Ch1 {
    pub fn new() -> Self {
        Self {
            nr10: 0x80,
            nr11: 0xBF,
            nr12: 0xF3,
            nr13: 0xFF,
            nr14: 0xBF,
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.nr10 = value,
            0xFF11 => self.nr11 = value,
            0xFF12 => self.nr12 = value,
            0xFF13 => self.nr13 = value,
            0xFF14 => self.nr14 = value,
            _ => panic!("should not reach"),
        }
    }

    // self.pace = (value & 0x70) >> 4;
    // self.direction = value & 0x08 != 0;
    // self.individual_step = value & 0x07;

    // self.duty = (value & 0xC0) >> 6;
    // self.initial_length = value & 0x3F;

    // self.initial_volume = (value & 0xF0) >> 4;
    // self.env_dir = (value & 0x08) != 0;
    // self.sweep_pace = value & 0x07;

    // self.period = (self.period & 0xFF00) | value as u16;

    // self.trigger = (value & 0x80) != 0;
    // self.length_enable = (value & 0x40) != 0;
    // self.period = (self.period & 0x00FF) | ((value as u16) << 8);

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.nr10 | 0x80,
            0xFF11 => self.nr11 | 0x3F,
            0xFF12 => self.nr12 | 0x00,
            0xFF13 => self.nr13 | 0xFF,
            0xFF14 => self.nr14 | 0xBF,
            _ => panic!("should not reach"),
        }
    }
}
