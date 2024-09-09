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

    pub fn ch4_left(&self) -> bool {
        self.nr51 & 0x80 != 0
    }
    pub fn ch3_left(&self) -> bool {
        self.nr51 & 0x40 != 0
    }
    pub fn ch2_left(&self) -> bool {
        self.nr51 & 0x20 != 0
    }
    pub fn ch1_left(&self) -> bool {
        self.nr51 & 0x10 != 0
    }
    pub fn ch4_right(&self) -> bool {
        self.nr51 & 0x08 != 0
    }
    pub fn ch3_right(&self) -> bool {
        self.nr51 & 0x04 != 0
    }
    pub fn ch2_right(&self) -> bool {
        self.nr51 & 0x02 != 0
    }
    pub fn ch1_right(&self) -> bool {
        self.nr51 & 0x01 != 0
    }

    pub fn vin_left(&self) -> bool {
        self.nr50 & 0x80 != 0
    }
    pub fn left_volume(&self) -> u8 {
        (self.nr50 & 0x70) >> 4
    }
    pub fn vin_right(&self) -> bool {
        self.nr50 & 0x08 != 0
    }
    pub fn right_volume(&self) -> u8 {
        self.nr50 & 0x07
    }

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

    pub fn pace(&self) -> u8 {
        (self.nr10 & 0x70) >> 4
    }
    pub fn direction(&self) -> bool {
        self.nr10 & 0x08 != 0
    }
    pub fn individual_step(&self) -> u8 {
        self.nr10 & 0x07
    }

    pub fn duty(&self) -> u8 {
        (self.nr11 & 0xC0) >> 6
    }
    pub fn initial_length(&self) -> u8 {
        self.nr11 & 0x3F
    }

    pub fn initial_volume(&self) -> u8 {
        (self.nr12 & 0xF0) >> 4
    }
    pub fn env_dir(&self) -> bool {
        self.nr12 & 0x08 != 0
    }
    pub fn sweep_pace(&self) -> u8 {
        self.nr12 & 0x07
    }

    pub fn trigger(&self) -> bool {
        self.nr14 & 0x80 != 0
    }
    pub fn length_enable(&self) -> bool {
        self.nr14 & 0x40 != 0
    }
    pub fn period(&self) -> u16 {
        ((self.nr14 & 0x07) as u16) << 8 | self.nr13 as u16
    }

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
