use sdl2::audio::AudioQueue;

const MASTER_VOLUME: f32 = 0.02;

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
            self.ch1.tick_sweep();
        }

        if self.counter == 8 {
            self.counter = 0;
            // TODO エンベロープ スイープ
            self.ch1.tick_envelope();
        }

        let freq = self.device.spec().freq;
        let must_add = freq / 512; // 絶対に作らないといけないサイズ
        let max_buffer_size = freq / 60 * 6; // 6フレーム
        let min_buffer_size = freq / 60 * 3; // 3フレーム
        let curret_buffer_size = self.device.size() as i32 / 4 / 2; // f32=4byte, 2 channle

        if curret_buffer_size > max_buffer_size {
            return;
        }

        let add_size = if curret_buffer_size < min_buffer_size {
            min_buffer_size - curret_buffer_size
        } else {
            must_add
        };

        let mut wave = Vec::with_capacity(add_size as usize * 2);
        for _ in 0..add_size {
            let ch1 = self.ch1.next(freq) * MASTER_VOLUME;

            // left
            wave.push(ch1);

            // right
            wave.push(ch1);
        }

        self.device.queue_audio(&wave).unwrap();
    }
}

pub struct Global {
    // 0xFF26
    nr52: u8,
    // 0xFF25
    nr51: u8,
    // 0xFF24
    nr50: u8,

    ch1_power: bool,
    ch2_power: bool,
    ch3_power: bool,
    ch4_power: bool,
}

impl Global {
    pub fn new() -> Self {
        Self {
            nr52: 0xF1,
            nr51: 0xF3,
            nr50: 0x77,

            ch1_power: true,
            ch2_power: false,
            ch3_power: false,
            ch4_power: false,
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
                    | (self.ch4_power as u8) << 3
                    | (self.ch3_power as u8) << 2
                    | (self.ch2_power as u8) << 1
                    | (self.ch1_power as u8) << 0
            }
            0xFF25 => self.nr51,
            0xFF24 => self.nr50,
            _ => panic!("should not reach"),
        }
    }
}

fn duty(duty: u8) -> f32 {
    match duty {
        0b00 => 0.125,
        0b01 => 0.25,
        0b10 => 0.50,
        0b11 => 0.75,
        _ => panic!("invalid duty {}", duty),
    }
}

fn volume(volume: u8) -> f32 {
    volume as f32 / 0x0F as f32
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

    phase: f32,
    // sweep
    sweep_pace: u8,
    current_period: u16,
    // envelope
    volume: u8,
    envelope_pace: u8,
}

impl Ch1 {
    pub fn new() -> Self {
        Self {
            nr10: 0x80,
            nr11: 0xBF,
            nr12: 0xF3,
            nr13: 0xFF,
            nr14: 0xBF,
            phase: 0.0,
            sweep_pace: 0,
            current_period: 0,
            volume: 0,
            envelope_pace: 0,
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.nr10 = value,
            0xFF11 => self.nr11 = value,
            0xFF12 => {
                self.nr12 = value;
                self.volume = self.initial_volume();
            }
            0xFF13 => {
                self.nr13 = value;
                self.current_period = self.period();
            }
            0xFF14 => {
                self.nr14 = value;
                self.current_period = self.period();
            }
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

    pub fn tick_sweep(&mut self) {
        if self.pace() == 0 {
            self.sweep_pace = 0;
            return;
        }
        self.sweep_pace += 1;
        if self.sweep_pace >= self.pace() {
            self.sweep_pace = 0;
            if self.direction() {
                // 1 = 減算
                self.current_period = self
                    .current_period
                    .wrapping_sub(self.current_period >> self.individual_step())
            } else {
                // 0 = 加算
                self.current_period = self
                    .current_period
                    .wrapping_add(self.current_period >> self.individual_step())
            }

            if self.current_period > 0x7FF {
                // TODO チャンネルをOFFにする
                //  -> globalのch1_powerをfalseにする
                // 一旦仮置き
                self.current_period = self.period();
            }
        }
    }

    pub fn tick_envelope(&mut self) {
        if self.sweep_pace() == 0 {
            self.envelope_pace = 0;
            return;
        }

        if self.initial_volume() == 0 && !self.env_dir() {
            // TODO (初期ボリューム = 0、エンベロープ = 減少)、DAC がオフ
        }

        self.envelope_pace += 1;
        if self.envelope_pace >= self.sweep_pace() {
            self.envelope_pace = 0;
            if self.env_dir() {
                // 1= 時間の経過とともに音量が増加
                if self.volume >= 0x0F {
                    return;
                }
                self.volume = self.volume.wrapping_add(1)
            } else {
                // 0= 時間の経過とともに音量が減少
                if self.volume == 0 {
                    return;
                }
                self.volume = self.volume.wrapping_sub(1)
            }
        }
    }

    pub fn next(&mut self, frequency: i32) -> f32 {
        let hz = 131072.0 / (2048.0 - self.current_period as f32);
        self.phase = (self.phase + (hz / frequency as f32)) % 1.0;
        return if self.phase > duty(self.duty()) {
            1.0
        } else {
            -1.0
        } * volume(self.volume);
    }
}
