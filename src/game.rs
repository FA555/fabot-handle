use crate::constant::{IDIOM_LENGTH, INITIALS, SPECIAL_INITIALS};
use crate::error::{IdiomError, PinyinError};
use crate::model::*;

impl TryFrom<&str> for Pinyin {
    type Error = PinyinError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars: Vec<char> = value.chars().collect();
        if chars.is_empty() {
            return Err(Self::Error::InvalidLength(chars.len()));
        }

        let tone = {
            let tone_num = chars.last().and_then(|c| c.to_digit(10).map(|d| d as u8));
            if tone_num.is_some() {
                chars.pop();
            }
            Tone::from_num(tone_num)?
        };

        let pronunciation = chars.iter().collect::<String>();

        let initial = {
            let mut initial = chars.first().unwrap().to_string();
            if INITIALS.contains(&initial) {
                if SPECIAL_INITIALS.contains(&initial) && chars.get(1) == Some(&'h') {
                    initial.push('h');
                }

                chars = chars.split_off(initial.len());

                Some(initial)
            } else {
                None
            }
        };

        Ok(Pinyin {
            pronunciation,
            initial: Initial(initial),
            vowel: chars.iter().collect(),
            tone,
        })
    }
}

impl TryFrom<Attempt> for [Character; IDIOM_LENGTH] {
    type Error = IdiomError;

    fn try_from(attempt: Attempt) -> Result<Self, Self::Error> {
        let words = attempt.word.chars().collect::<Vec<_>>();

        let pinyin_s = attempt
            .pinyin
            .split_whitespace()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        if words.len() != pinyin_s.len() || words.len() != IDIOM_LENGTH {
            Err(IdiomError::InconsistentLength(
                words.len(),
                pinyin_s.len(),
                IDIOM_LENGTH,
            ))?
        }

        let characters = words
            .into_iter()
            .zip(pinyin_s)
            .map(|(literal, pinyin)| Character {
                literal: literal.to_string(),
                pinyin,
            })
            .collect::<Vec<_>>();

        Ok(characters.try_into().unwrap())
    }
}

impl PinyinCount {
    pub fn from_attempt(attempt: &[Character; IDIOM_LENGTH]) -> Self {
        let mut count: PinyinCount = Default::default();

        for character in attempt {
            *count.literals.entry(character.literal.clone()).or_insert(0) += 1;

            *count
                .pronunciations
                .entry(character.pinyin.pronunciation.clone())
                .or_insert(0) += 1;

            if let Some(initial) = character.pinyin.initial.0.clone() {
                *count.initials.entry(initial).or_insert(0) += 1;
            }

            *count
                .vowels
                .entry(character.pinyin.vowel.clone())
                .or_insert(0) += 1;

            if character.pinyin.tone.0.is_some() {
                *count
                    .tones
                    .entry(character.pinyin.tone.to_string())
                    .or_insert(0) += 1;
            }
        }

        count
    }
}

impl From<Character> for CalculatedCharacter {
    fn from(character: Character) -> Self {
        Self {
            literal: character.literal,
            pinyin: character.pinyin,
            result: Default::default(),
        }
    }
}

impl From<&Character> for CalculatedCharacter {
    fn from(character: &Character) -> Self {
        Self {
            literal: character.literal.clone(),
            pinyin: character.pinyin.clone(),
            result: Default::default(),
        }
    }
}

impl CalculatedAttempt {
    pub fn from_attempt(
        answer: &[Character; IDIOM_LENGTH],
        attempt: &Attempt,
    ) -> Result<Self, IdiomError> {
        let mut answer_count = PinyinCount::from_attempt(answer);

        let characters: [Character; IDIOM_LENGTH] = attempt.clone().try_into()?;
        let mut res: [CalculatedCharacter; IDIOM_LENGTH] = characters.map(Into::into);

        for (char, ans_char) in res.iter_mut().zip(answer) {
            if char.literal() == ans_char.literal() {
                answer_count.match_whole_char(char);
                char.result.whole = State::Correct;
            }
        }
        for char in &mut res {
            if char.result.whole != State::Missing {
                continue;
            }
            if answer_count.literals.contains_key(&char.literal())
                && answer_count.match_whole_char(char)
            {
                char.result.whole = State::Misplaced;
            }
        }

        for (char, ans_char) in res.iter_mut().zip(answer) {
            if char.pronunciation() == ans_char.pronunciation() {
                answer_count.match_pronunciation_char(char);
                char.result.pronunciation = State::Correct;
            }
        }
        for char in &mut res {
            if char.result.pronunciation != State::Missing {
                continue;
            }
            if answer_count
                .pronunciations
                .contains_key(&char.pronunciation())
                && answer_count.match_pronunciation_char(char)
            {
                char.result.pronunciation = State::Misplaced;
            }
        }

        for (char, ans_char) in res.iter_mut().zip(answer) {
            if char.initial() == ans_char.initial() {
                answer_count.match_initial_char(char);
                char.result.initial = State::Correct;
            }
        }
        for char in &mut res {
            if char.result.initial != State::Missing {
                continue;
            }
            if let Some(initial) = char.initial().0 {
                if answer_count.initials.contains_key(&initial)
                    && answer_count.match_initial_char(char)
                {
                    char.result.initial = State::Misplaced;
                }
            }
        }

        for (char, ans_char) in res.iter_mut().zip(answer) {
            if char.vowel() == ans_char.vowel() {
                answer_count.match_vowel_char(char);
                char.result.vowel = State::Correct;
            }
        }
        for char in &mut res {
            if char.result.vowel != State::Missing {
                continue;
            }
            if answer_count.vowels.contains_key(&char.vowel())
                && answer_count.match_vowel_char(char)
            {
                char.result.vowel = State::Misplaced;
            }
        }

        for (char, ans_char) in res.iter_mut().zip(answer) {
            if char.tone() == ans_char.tone() {
                answer_count.match_tone_char(char);
                char.result.tone = State::Correct;
            }
        }
        for char in &mut res {
            if char.result.tone != State::Missing {
                continue;
            }
            let tone_str = char.tone().to_string();
            if answer_count.tones.contains_key(&tone_str) && answer_count.match_tone_char(char) {
                char.result.tone = State::Misplaced;
            }
        }

        Ok(Self(res))
    }
}
