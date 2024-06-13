use std::collections::HashMap;
use super::notes::Note;

// TODO: ADD MORE TUNINGS
// TODO: HANDLE 7 AND 8 STRINGS GUITARS
// TODO: SUPPORT CUSTOM TUNINGS
pub fn get_tunings() -> HashMap<String, Vec<Note>> {
    let mut tunings = HashMap::new();

    // STANDARD TUNING
    tunings.insert("standard".to_string(), vec![Note::E(2), Note::A(2), Note::D(3), Note::G(3), Note::B(3), Note::E(4)]);

    // OPEN TUNINGS
    tunings.insert("open_a".to_string(), vec![Note::D(2), Note::A(2), Note::CSharp(3), Note::E(3), Note::A(3), Note::E(4)]);

    // DROP TUNINGS
    tunings.insert("drop_d".to_string(), vec![Note::D(2), Note::A(2), Note::D(3), Note::G(3), Note::B(3), Note::E(4)]);
    tunings.insert("double_drop_d".to_string(), vec![Note::D(2), Note::A(2), Note::D(3), Note::G(3), Note::B(3), Note::D(4)]);

    // WEIRD TUNINGS
    tunings.insert("dadgad".to_string(), vec![Note::D(2), Note::A(2), Note::D(3), Note::G(3), Note::A(3), Note::D(4)]);

    tunings
}

