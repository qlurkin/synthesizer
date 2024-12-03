#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Tone {
    pub octave: i32,
    pub semitone: i32,
}

#[allow(unused)]
impl Tone {
    pub fn get_frequency(&self) -> f32 {
        let base_frequency = 440.0_f32; // Fréquence du La 4ème octave
        let semitone_ratio = 2.0_f32.powf(1.0 / 12.0); // Ratio entre deux demi-tons successifs

        let semitone_difference = (self.octave - 4) * 12 + self.semitone; // Nombre total de demi-tons par rapport au La 4ème octave
        base_frequency * semitone_ratio.powi(semitone_difference)
    }

    pub fn up(&self, n: u32) -> Self {
        let mut octave = self.octave;
        let mut semitone = self.semitone;
        for _ in 0..n {
            if semitone == 2 {
                octave += 1;
            }
            semitone = (semitone + 9 + 1).rem_euclid(12) - 9;
        }
        Self { semitone, octave }
    }

    pub fn down(&self, n: u32) -> Self {
        let mut octave = self.octave;
        let mut semitone = self.semitone;
        for _ in 0..n {
            if semitone == -9 {
                octave -= 1;
            }
            semitone = (semitone + 9 - 1).rem_euclid(12) - 9;
        }
        Self { semitone, octave }
    }

    pub fn get_string(&self) -> String {
        let semitone_str = match self.semitone {
            0 => "A-",
            1 => "A#",
            2 => "B-",
            -9 => "C-",
            -8 => "C#",
            -7 => "D-",
            -6 => "D#",
            -5 => "E-",
            -4 => "F-",
            -3 => "F#",
            -2 => "G-",
            _ => "G#",
        };
        format!("{}{}", semitone_str, self.octave)
    }
}
