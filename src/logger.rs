
use serde::{Serialize, Deserialize};
use fern;
use serde_json;
use chrono;

#[derive(Serialize)]
struct Log {
    severity: String,
    payload: Payload,
    timestamp: String,
    trace: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    pub kind: LogKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<ErrorMessage>,
}

#[derive(Serialize, Deserialize)]
pub enum LogKind {
    Request,
    Response,
    Err,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub r#type: String,
    pub title: String,
    pub detail: String,
}

pub fn setup(level: log::LevelFilter) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%z");
            let payload: Result<Payload, serde_json::Error> =
                serde_json::from_str(&message.to_string());
            out.finish(format_args!(
                "{}",
                serde_json::json!(Log {
                severity: record.level().to_string(),
                payload: match payload {
                    Ok(payload) => payload,
                    Err(_) => Payload {
                        x_request_id: None,
                        host: None,
                        user_agent: None,
                        method: None,
                        uri: None,
                        status: None,
                        duration: None,
                        kind: LogKind::Err,
                        error_message: Some(ErrorMessage {
                            r#type: "primitive".to_string(),
                            title: "primitive error".to_string(),
                            detail: message.to_string()
                        })
                    },
                },
                timestamp: ts.to_string(),
                trace: match record.level().as_str() {
                    "TRACE" =>
                        record.file().unwrap_or("unknown").to_string()
                            + ":"
                            + &record.line().unwrap_or(0).to_string(),
                    _ => "".to_string(),
                }
            })
                    .to_string()
            ))
        })
        .chain(std::io::stdout())
        .level(level)
        .level_for("api", level)
        .apply()
        .unwrap();
}
