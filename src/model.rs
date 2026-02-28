use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::constant::IDIOM_LENGTH;
use crate::error::PinyinError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answer {
    pub word: String,
    pub pinyin: String,
    pub explanation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attempt {
    pub word: String,
    pub pinyin: String,
    pub verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub answer: Attempt,
    pub attempts: Vec<Attempt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ToneExplicit {
    High,
    Rising,
    Low,
    Falling,
}

type Pronunciation = String;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Initial(pub Option<String>);

type Vowel = String;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Tone(pub Option<ToneExplicit>);

impl Tone {
    pub fn from_num(tone: Option<u8>) -> Result<Self, PinyinError> {
        match tone {
            None | Some(0) => Ok(Self(None)),
            Some(1) => Ok(Self(Some(ToneExplicit::High))),
            Some(2) => Ok(Self(Some(ToneExplicit::Rising))),
            Some(3) => Ok(Self(Some(ToneExplicit::Low))),
            Some(4) => Ok(Self(Some(ToneExplicit::Falling))),
            Some(num) => Err(PinyinError::InvalidTone(num)),
        }
    }
}

impl Display for Tone {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Some(ToneExplicit::High) => "1",
                Some(ToneExplicit::Rising) => "2",
                Some(ToneExplicit::Low) => "3",
                Some(ToneExplicit::Falling) => "4",
                None => "",
            }
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Pinyin {
    pub pronunciation: Pronunciation,
    pub initial: Initial,
    pub vowel: Vowel,
    pub tone: Tone,
}

pub trait CharInfo {
    fn literal(&self) -> String;

    fn pronunciation(&self) -> Pronunciation;

    fn initial(&self) -> Initial;

    fn vowel(&self) -> Vowel;

    fn tone(&self) -> Tone;
}

#[derive(Debug, Serialize)]
pub struct Character {
    pub literal: String,
    pub pinyin: Pinyin,
}

impl CharInfo for Character {
    fn literal(&self) -> String {
        self.literal.to_owned()
    }

    fn pronunciation(&self) -> Pronunciation {
        self.pinyin.pronunciation.to_owned()
    }

    fn initial(&self) -> Initial {
        self.pinyin.initial.to_owned()
    }

    fn vowel(&self) -> Vowel {
        self.pinyin.vowel.to_owned()
    }

    fn tone(&self) -> Tone {
        self.pinyin.tone
    }
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub enum State {
    Correct,

    Misplaced,

    #[default]
    Missing,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Correct => "correct",
                State::Misplaced => "misplaced",
                State::Missing => "missing",
            }
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct PinyinCount {
    pub literals: HashMap<String, usize>,
    pub pronunciations: HashMap<String, usize>,
    pub initials: HashMap<String, usize>,
    pub vowels: HashMap<String, usize>,
    pub tones: HashMap<String, usize>,
}

impl PinyinCount {
    pub fn match_initial(&mut self, initial: String) -> bool {
        if let Some(v) = self.initials.get_mut(&initial) {
            if *v > 0 {
                *v -= 1;
                return true;
            }
        }

        false
    }

    pub fn match_initial_char(&mut self, character: &impl CharInfo) -> bool {
        if let Some(initial) = character.initial().0 {
            self.match_initial(initial)
        } else {
            false
        }
    }

    pub fn match_vowel(&mut self, vowel: String) -> bool {
        if let Some(v) = self.vowels.get_mut(&vowel) {
            if *v > 0 {
                *v -= 1;
                return true;
            }
        }

        false
    }

    pub fn match_vowel_char(&mut self, character: &impl CharInfo) -> bool {
        self.match_vowel(character.vowel())
    }

    pub fn match_tone(&mut self, tone: Tone) -> bool {
        if let Some(v) = self.tones.get_mut(&tone.to_string()) {
            if *v > 0 {
                *v -= 1;
                return true;
            }
        }

        false
    }

    pub fn match_tone_char(&mut self, character: &impl CharInfo) -> bool {
        let tone = character.tone();
        if tone.0.is_some() {
            self.match_tone(tone)
        } else {
            false
        }
    }

    pub fn match_pronunciation(&mut self, pronunciation: Pronunciation) -> bool {
        if let Some(v) = self.pronunciations.get_mut(&pronunciation) {
            if *v > 0 {
                *v -= 1;
                return true;
            }
        }

        false
    }

    pub fn match_pronunciation_char(&mut self, character: &impl CharInfo) -> bool {
        self.match_pronunciation(character.pronunciation())
    }

    pub fn match_whole_char(&mut self, character: &impl CharInfo) -> bool {
        if let Some(v) = self.literals.get_mut(&character.literal()) {
            if *v > 0 {
                *v -= 1;
                return true;
            }
        }

        false

        // self.match_pronunciation_char(character);
        // self.match_initial_char(character);
        // self.match_vowel_char(character);
        // self.match_tone_char(character);
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CharacterResult {
    pub whole: State,
    pub pronunciation: State,
    pub initial: State,
    pub vowel: State,
    pub tone: State,
}

#[derive(Debug, Serialize)]
pub struct CalculatedCharacter {
    pub literal: String,
    pub pinyin: Pinyin,
    pub result: CharacterResult,
}

impl CharInfo for CalculatedCharacter {
    fn literal(&self) -> String {
        self.literal.to_owned()
    }

    fn pronunciation(&self) -> Pronunciation {
        self.pinyin.pronunciation.to_owned()
    }

    fn initial(&self) -> Initial {
        self.pinyin.initial.to_owned()
    }

    fn vowel(&self) -> Vowel {
        self.pinyin.vowel.to_owned()
    }

    fn tone(&self) -> Tone {
        self.pinyin.tone
    }
}

#[derive(Debug, Serialize)]
// pub struct CalculatedAttempt(pub [CalculatedCharacter; IDIOM_LENGTH]);
pub struct CalculatedAttempt {
    pub(crate) characters: [CalculatedCharacter; IDIOM_LENGTH],
    pub(crate) verified: bool,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub result: Vec<CalculatedAttempt>,
    pub max_attempt_count: usize,
    pub finished: bool,
}
