use sdl2::audio::AudioQueue;

pub struct APU {
    prev_div: u8,
    counter: u8,
    device: AudioQueue<f32>,
}

impl APU {
    pub fn new(device: AudioQueue<f32>) -> Self {
        Self {
            prev_div: 0,
            counter: 0,
            device,
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
