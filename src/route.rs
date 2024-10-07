use crate::constant::{IMAGE_DIR, MAX_ATTEMPT_COUNT};
use crate::dict::{ANSWERS, DICT, REVERSE_ANSWERS};
use crate::error::OmniError;
use crate::model::{Answer, CalculatedAttempt, Input, Output};

use axum::extract::{Json, Query};
use axum::http::StatusCode;
use base64::{engine::general_purpose, Engine};
use rand::seq::SliceRandom;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    message: String,
    image_base64: Option<String>,
}

impl ImageResponse {
    fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            image_base64: None,
        }
    }

    fn with_image(message: impl ToString, image_base64: String) -> Self {
        Self {
            message: message.to_string(),
            image_base64: Some(image_base64),
        }
    }
}

pub async fn ping(Query(params): Query<HashMap<String, String>>) -> String {
    params
        .get("arg")
        .unwrap_or(&"ping".into())
        .replace("i", "o")
}

pub async fn start() -> Json<Answer> {
    Json(ANSWERS.choose(&mut rand::thread_rng()).unwrap().clone())
}

fn gen_image(data: Output) -> Result<String, OmniError> {
    let uuid = crate::util::gen_uuid();
    let data_file = &format!("data-{uuid}.json");
    let data_path = &format!("{IMAGE_DIR}{data_file}");
    let image_path = &format!("{IMAGE_DIR}handle-{uuid}.png");

    std::fs::write(data_path, serde_json::to_string(&data)?)?;

    std::process::Command::new("typst")
        .args([
            "compile",
            "image/main.typ",
            "--ppi",
            "400",
            "--input",
            &format!("path={data_file}"),
            image_path,
        ])
        .output()?;

    let image_binary = std::fs::read(image_path)?;
    let image_base64 = general_purpose::STANDARD.encode(image_binary);

    std::fs::remove_file(image_path)?;
    std::fs::remove_file(data_path)?;

    Ok(image_base64)
}

fn attempt_inner(input: Input) -> Result<String, OmniError> {
    eprintln!("{:#?}", input);

    let finished = input.attempts.len() >= MAX_ATTEMPT_COUNT
        || (!input.attempts.is_empty() && input.attempts.last().unwrap().word == input.answer.word);

    let answer = input.answer.try_into()?;

    let output = Output {
        result: input
            .attempts
            .iter()
            .map(|attempt| CalculatedAttempt::from_attempt(&answer, attempt))
            .collect::<Result<Vec<_>, _>>()?,
        max_attempt_count: MAX_ATTEMPT_COUNT,
        finished,
    };

    gen_image(output)
}

pub async fn attempt(Json(input): Json<Input>) -> (StatusCode, Json<ImageResponse>) {
    match attempt_inner(input) {
        Ok(image_base64) => (
            StatusCode::OK,
            Json(ImageResponse::with_image("ok", image_base64)),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ImageResponse::new(format!("error: {}", e))),
        ),
    }
}

pub async fn try_get_pinyin(Query(params): Query<HashMap<String, String>>) -> Json<Option<Answer>> {
    let word = match params.get("word") {
        Some(word) => word,
        None => return Json(None),
    };

    Json(REVERSE_ANSWERS.get(word).map(|index| DICT[*index].clone()))
}
