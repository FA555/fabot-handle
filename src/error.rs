use thiserror::Error;
#[derive(Debug, Error)]
pub enum PinyinError {
    #[error("invalid length (`{0}`): expected > 0")]
    InvalidLength(usize),
    #[error("invalid tone (`{0}`): expected one of {{0, 1, 2, 3, 4}}, or nothing")]
    InvalidTone(u8),
}

#[derive(Debug, Error)]
pub enum IdiomError {
    #[error("inconsistent length (word: `{0}`, pinyin: `{1}`): expected {2}")]
    InconsistentLength(usize, usize, usize),
    #[error(transparent)]
    InvalidPinyin(#[from] PinyinError),

}

#[derive(Debug, Error)]
pub enum OmniError {
    #[error(transparent)]
    Idiom(#[from] IdiomError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
