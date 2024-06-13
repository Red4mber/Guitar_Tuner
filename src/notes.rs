use std::fmt;

// Woah, proper documentation ?!
// Yeah, and I quickly got real tired of writing those, so don't get used to it.


// All of this only really works in 12-Tone Equal Temperament,
// so if you really want microtones you should go elsewhere you jazzman

const DEFAULT_A4_FREQ: f32 = 440.0;

/// Enumeration representing all 12 notes of the chromatic scale
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Note {
    C(u8),
    CSharp(u8),
    D(u8),
    DSharp(u8),
    E(u8),
    F(u8),
    FSharp(u8),
    G(u8),
    GSharp(u8),
    A(u8),
    ASharp(u8),
    B(u8)
}
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Note::C(o)      => format!("C{}",  o),
            Note::CSharp(o) => format!("C#{}", o),
            Note::D(o)      => format!("D{}",  o),
            Note::DSharp(o) => format!("D#{}", o),
            Note::E(o)      => format!("E{}",  o),
            Note::F(o)      => format!("F{}",  o),
            Note::FSharp(o) => format!("F#{}", o),
            Note::G(o)      => format!("G{}",  o),
            Note::GSharp(o) => format!("G#{}", o),
            Note::A(o)      => format!("A{}",  o),
            Note::ASharp(o) => format!("A#{}", o),
            Note::B(o)      => format!("B{}",  o),
        };
        s.fmt(f)
    }
}

impl Note {
    /// Calculates the frequency of the note in Hz.
    ///
    /// This function uses the equal temperament tuning system, where each octave is divided
    /// into 12 equally spaced semitones. The frequency of any note can be calculated as:
    ///
    /// ```text
    /// f = A4_freq * 2^((n - 57) / 12)
    /// ```
    ///
    /// Where:
    /// - `f` is the frequency of the target note in Hz.
    /// - `A4_freq` is the frequency of A4 (typically 440 Hz, but can be customized).
    /// - `n` is the number of semitones from C0 (e.g., A4 is 57 semitones from C0).
    ///
    /// # Arguments
    ///
    /// * `a4_freq` - An optional frequency for A4. If `None`, the default 440 Hz is used.
    ///
    /// # Returns
    ///
    /// The frequency of the note in Hz.
    ///
    pub fn frequency(&self, custom_a4_freq: Option<f32>) -> f32 {
        let a4_frequency = custom_a4_freq.unwrap_or(DEFAULT_A4_FREQ);
        let (semitones, octave) = match self {
            Note::C(o) => (0, *o),
            Note::CSharp(o) => (1, *o),
            Note::D(o) => (2, *o),
            Note::DSharp(o) => (3, *o),
            Note::E(o) => (4, *o),
            Note::F(o) => (5, *o),
            Note::FSharp(o) => (6, *o),
            Note::G(o) => (7, *o),
            Note::GSharp(o) => (8, *o),
            Note::A(o) => (9, *o),
            Note::ASharp(o) => (10, *o),
            Note::B(o) => (11, *o),
        };
        let semitones_from_a4 = (octave as i8 - 4) * 12 + semitones - 9; // 4 octaves, minus 4 semitones to get from C to A
        a4_frequency * 2f32.powf(semitones_from_a4 as f32 / 12.0)
    }



    /// Converts a given frequency to a musical note and its deviation from the perfect pitch in cents.
    ///
    /// This function takes a frequency in Hz and an optional custom A4 frequency (default is 440 Hz).
    /// It calculates the closest musical note and how many cents it deviates from the perfect pitch.
    ///
    /// # Arguments
    ///
    /// * `freq` - The frequency in Hz to convert to a musical note.
    /// * `custom_a4_freq` - An optional custom frequency for A4 (default is 440 Hz).
    ///
    /// # Returns
    ///
    /// A tuple `(Note, f64)` where:
    /// - `Note` is an enum representing the closest musical note.
    /// - `f64` is the deviation in cents from the perfect pitch (-50 to +50 cents).
    ///   Positive values mean the frequency is higher than the perfect pitch.
    ///
    /// # Notes
    ///
    /// - Frequencies corresponding to black keys (e.g., A♯/B♭) are mapped to the next white key's octave.
    /// - The function assumes 12-tone equal temperament.
    pub fn from_frequency(freq: f32, custom_a4_freq: Option<f32>) -> (Self, f32) {
        let a4_frequency = custom_a4_freq.unwrap_or(DEFAULT_A4_FREQ);

        // n  =  12*log2(fn/440 Hz).
        let notes_to_a4 = 12.0 * (freq / a4_frequency).log2();
        let rounded_notes_to_a4 = notes_to_a4.round_ties_even() as i8;
        let cents_off = (notes_to_a4 - rounded_notes_to_a4 as f32) * 100.0;

        let octave = ((rounded_notes_to_a4 + 9 + 48)/12) as u8;

        let note_number = (((rounded_notes_to_a4 + 9) % 12) + 12) % 12;

        let note = match note_number.abs() {
            0 => Note::C(octave),
            1 => Note::CSharp(octave),
            2 => Note::D(octave),
            3 => Note::DSharp(octave),
            4 => Note::E(octave),
            5 => Note::F(octave),
            6 => Note::FSharp(octave),
            7 => Note::G(octave),
            8 => Note::GSharp(octave),
            9 => Note::A(octave),
            10 => Note::ASharp(octave),
            11 => Note::B(octave),
            _ => unreachable!(),
        };

        (note, cents_off)
    }
}