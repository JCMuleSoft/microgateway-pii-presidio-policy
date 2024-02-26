use anyhow::Result;
use pdk::api::hl::*;
use serde::{Deserialize, Serialize};

use crate::generated::config::Config;

// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

#[derive(Serialize, Clone, Debug)]
struct PresidioAnalyzeRequest<'a> {
    text: String,
    language: &'a str,
    score_threshold: f64,
    entities: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
struct PresidioAnalyzeResponse {
    start: i32,
    end: i32,
    entity_type: String,
    analysis_explanation: Option<PresidioAnalysisExplanation>,
    recognition_metadata: Option<PresidioRecognizedMetadata>,
}

#[derive(Deserialize, Clone, Debug)]
struct PresidioRecognizedMetadata {
    recognizer_name: String,
}

#[derive(Deserialize, Clone, Debug)]
struct PresidioAnalysisExplanation {
    recognizer: String,
    pattern_name: String,
    pattern: String,
    original_score: f64,
    score: f64,
    textual_explanation: String,
    supportive_context_word: String,
}

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter<'a>(
    request_state: RequestState,
    http_client: &HttpClient,
    config: &'a Config,
) -> Flow<()> {
    logger::info!("Config is {:?}", config);
    let language = config.language.as_deref().unwrap_or("en");
    let cluster_name = config.server.as_deref().unwrap_or("presidio");
    let server_url = config.server_url.as_str();
    let action = config.action.as_deref().unwrap_or("Reject");
    let score_threshold = config.score_threshold.unwrap_or(0.5);

    let body_state = request_state.into_body_state().await;
    let body_vec = body_state.body();

    let request_body = String::from_utf8(body_vec).unwrap();
    let presidio_request = PresidioAnalyzeRequest {
        text: request_body,
        language,
        score_threshold,
        entities: None,
    };
    let v = serde_json::to_vec(&presidio_request).unwrap();

    let result = http_client
        .request(cluster_name, server_url)
        .path("/analyze")
        .body(v.as_slice())
        .headers(vec![("Content-Type", "application/json")])
        .post()
        .await;

    return match result {
        Ok(client_response) => {
            let http_result = client_response.body();
            let json: Vec<PresidioAnalyzeResponse> = serde_json::from_slice(http_result).unwrap();

            if !json.is_empty() {
                let reason = json
                    .iter()
                    .map(|r| format!("{} at {},{}", r.entity_type, r.start, r.end))
                    .collect::<Vec<String>>()
                    .join("\n");

                if action == "Reject" {
                    Flow::Break(Response::new(401).with_body(format!(
                        "Your body has sensitive data. Details:\n {}",
                        reason
                    )))
                } else {
                    logger::error!("Request has sensitive data reason:\n {} ", reason);
                    Flow::Continue(())
                }
            } else {
                Flow::Continue(())
            }
        }
        Err(error) => {
            logger::info!("Error while trying to get to presidio {:?}", error);
            Flow::Break(Response::new(401).with_body(format!(
                "Unable to verify the request:\n {:?}",
                error
            )))
        }
    };
}

#[entrypoint]
async fn configure(
    launcher: Launcher,
    Configuration(bytes): Configuration,
    http_client: HttpClient,
) -> Result<()> {
    let config = serde_json::from_slice(&bytes)?;
    let filter = on_request(|rs| request_filter(rs, &http_client, &config));
    launcher.launch(filter).await?;
    Ok(())
}
