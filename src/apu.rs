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
    power: bool,
    ch1_power: bool,
    ch2_power: bool,
    ch3_power: bool,
    ch4_power: bool,
    // 0xFF25
    ch1_right: bool,
    ch1_left: bool,
    ch2_right: bool,
    ch2_left: bool,
    ch3_right: bool,
    ch3_left: bool,
    ch4_right: bool,
    ch4_left: bool,

    // 0xFF24
    vin_left: bool,
    vin_right: bool,
    left_volume: u8,
    right_volume: u8,
}

impl Global {
    pub fn new() -> Self {
        Self {
            power: false,
            ch1_power: false,
            ch2_power: false,
            ch3_power: false,
            ch4_power: false,
            ch4_left: false,
            ch3_left: false,
            ch2_left: false,
            ch1_left: false,
            ch4_right: false,
            ch3_right: false,
            ch2_right: false,
            ch1_right: false,
            vin_left: false,
            left_volume: 0,
            vin_right: false,
            right_volume: 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF26 => {
                self.power = value & 0x80 != 0;
                self.ch4_power = value & 0x08 != 0;
                self.ch3_power = value & 0x04 != 0;
                self.ch2_power = value & 0x02 != 0;
                self.ch1_power = value & 0x01 != 0;
            }
            0xFF25 => {
                self.ch4_left = value & 0x80 != 0;
                self.ch3_left = value & 0x40 != 0;
                self.ch2_left = value & 0x20 != 0;
                self.ch1_left = value & 0x10 != 0;
                self.ch4_right = value & 0x08 != 0;
                self.ch3_right = value & 0x04 != 0;
                self.ch2_right = value & 0x02 != 0;
                self.ch1_right = value & 0x01 != 0;
            }
            0xFF24 => {
                self.vin_left = value & 0x80 != 0;
                self.left_volume = (value & 0x70) >> 4;
                self.vin_right = value & 0x08 != 0;
                self.right_volume = value & 0x07;
            }
            _ => panic!("should not reach"),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF26 => {
                (if self.power { 1 } else { 0 }) << 7
                    | (if self.ch4_power { 1 } else { 0 }) << 3
                    | (if self.ch3_power { 1 } else { 0 }) << 2
                    | (if self.ch2_power { 1 } else { 0 }) << 1
                    | (if self.ch1_power { 1 } else { 0 }) << 0
            }
            0xFF25 => {
                (if self.ch4_left { 1 } else { 0 }) << 7
                    | (if self.ch3_left { 1 } else { 0 }) << 6
                    | (if self.ch2_left { 1 } else { 0 }) << 5
                    | (if self.ch1_left { 1 } else { 0 }) << 4
                    | (if self.ch4_right { 1 } else { 0 }) << 3
                    | (if self.ch3_right { 1 } else { 0 }) << 2
                    | (if self.ch2_right { 1 } else { 0 }) << 1
                    | (if self.ch1_right { 1 } else { 0 }) << 0
            }
            0xFF24 => {
                (if self.vin_left { 1 } else { 0 }) << 7
                    | self.left_volume << 4
                    | (if self.vin_right { 1 } else { 0 }) << 3
                    | self.right_volume
            }
            _ => panic!("should not reach"),
        }
    }
}

pub struct Ch1 {
    // 0xFF10
    pace: u8,
    direction: bool,
    individual_step: u8,
    // 0xFF11
    duty: u8,
    initial_length: u8,
    // 0xFF12
    initial_volume: u8,
    env_dir: bool,
    sweep_pace: u8,
    // 0xFF13 | 0xFF14
    period: u16,
    // 0xFF14
    trigger: bool,
    length_enable: bool,
}

impl Ch1 {
    pub fn new() -> Self {
        Self {
            pace: 0,
            direction: false,
            individual_step: 0,
            duty: 0,
            initial_length: 0,
            initial_volume: 0,
            env_dir: false,
            sweep_pace: 0,
            period: 0,
            trigger: false,
            length_enable: false,
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => {
                self.pace = (value & 0x70) >> 4;
                self.direction = value & 0x08 != 0;
                self.individual_step = value & 0x07;
            }
            0xFF11 => {
                self.duty = (value & 0xC0) >> 6;
                self.initial_length = value & 0x3F;
            }
            0xFF12 => {
                self.initial_volume = (value & 0xF0) >> 4;
                self.env_dir = (value & 0x08) != 0;
                self.sweep_pace = value & 0x07;
            }
            0xFF13 => {
                self.period = (self.period & 0xFF00) | value as u16;
            }
            0xFF14 => {
                self.trigger = (value & 0x80) != 0;
                self.length_enable = (value & 0x40) != 0;
                self.period = (self.period & 0x00FF) | ((value as u16) << 8);
            }
            _ => panic!("should not reach"),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF10 => {
                self.pace << 4 | (if self.direction { 1 } else { 0 }) << 3 | self.individual_step
            }
            0xFF11 => {
                // self.duty = (value & 0xC0) >> 6;
                // self.initial_length = value & 0x3F;
            }
            0xFF12 => {
                // self.initial_volume = (value & 0xF0) >> 4;
                // self.env_dir = (value & 0x08) != 0;
                // self.sweep_pace = value & 0x07;
            }
            0xFF13 => {
                // self.period = (self.period & 0xFF00) | value as u16;
            }
            0xFF14 => {
                // self.trigger = (value & 0x80) != 0;
                // self.length_enable = (value & 0x40) != 0;
                // self.period = (self.period & 0x00FF) | ((value as u16) << 8);
            }
            _ => panic!("should not reach"),
        }
    }
}
