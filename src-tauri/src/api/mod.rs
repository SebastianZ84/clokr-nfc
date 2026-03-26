pub mod queue;

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct NfcPunchRequest {
    #[serde(rename = "nfcCardId")]
    nfc_card_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NfcPunchResponse {
    pub action: String,
    pub employee: Option<EmployeeInfo>,
    pub time: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "balanceHours")]
    pub balance_hours: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmployeeInfo {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "employeeNumber")]
    pub employee_number: String,
}

pub async fn nfc_punch(
    client: &reqwest::Client,
    api_url: &str,
    nfc_card_id: &str,
    api_key: Option<&str>,
) -> Result<NfcPunchResponse, PunchError> {
    let url = format!("{api_url}/api/v1/time-entries/nfc-punch");
    let body = NfcPunchRequest {
        nfc_card_id: nfc_card_id.to_string(),
    };

    let mut req = client
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(5));

    // Add API key as Bearer token
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {key}"));
    }

    let resp = req
        .send()
        .await
        .map_err(|e| {
            error!("Network error: {e}");
            PunchError::Network(e.to_string())
        })?;

    let status = resp.status().as_u16();
    let text = resp.text().await.unwrap_or_default();

    match status {
        200 => {
            let data: NfcPunchResponse = serde_json::from_str(&text).map_err(|e| {
                error!("Parse error: {e}");
                PunchError::Parse(e.to_string())
            })?;
            info!(
                "Punch: {} - {} {}",
                data.action,
                data.employee
                    .as_ref()
                    .map(|e| format!("{} {}", e.first_name, e.last_name))
                    .unwrap_or_default(),
                data.time.as_deref().unwrap_or("")
            );
            Ok(data)
        }
        404 => Ok(NfcPunchResponse {
            action: "UNKNOWN".to_string(),
            employee: None,
            time: None,
            error: Some("Unbekannte Karte".to_string()),
            balance_hours: None,
        }),
        401 => Ok(NfcPunchResponse {
            action: "UNAUTHORIZED".to_string(),
            employee: None,
            time: None,
            error: Some("Ungültiger API-Schlüssel".to_string()),
            balance_hours: None,
        }),
        403 => Ok(NfcPunchResponse {
            action: "FORBIDDEN".to_string(),
            employee: None,
            time: None,
            error: Some("Zugriff verweigert".to_string()),
            balance_hours: None,
        }),
        409 => {
            let data: NfcPunchResponse =
                serde_json::from_str(&text).unwrap_or(NfcPunchResponse {
                    action: "BLOCKED".to_string(),
                    employee: None,
                    time: None,
                    error: Some("Gesperrt".to_string()),
                    balance_hours: None,
                });
            Ok(data)
        }
        _ => Err(PunchError::Api(format!("HTTP {status}: {text}"))),
    }
}

#[derive(Debug)]
pub enum PunchError {
    Network(String),
    Parse(String),
    Api(String),
}

#[derive(Debug, Deserialize)]
struct AllowedCardsResponse {
    cards: Vec<String>,
}

pub async fn fetch_allowed_cards(
    client: &reqwest::Client,
    api_url: &str,
    api_key: Option<&str>,
) -> Result<std::collections::HashSet<String>, PunchError> {
    let url = format!("{api_url}/api/v1/terminals/allowed-cards");

    let mut req = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10));

    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {key}"));
    }

    let resp = req.send().await.map_err(|e| {
        error!("Allowlist fetch network error: {e}");
        PunchError::Network(e.to_string())
    })?;

    let status = resp.status().as_u16();
    let text = resp.text().await.unwrap_or_default();

    match status {
        200 => {
            let data: AllowedCardsResponse = serde_json::from_str(&text).map_err(|e| {
                error!("Allowlist parse error: {e}");
                PunchError::Parse(e.to_string())
            })?;
            info!("Fetched allowlist: {} cards", data.cards.len());
            Ok(data.cards.into_iter().collect())
        }
        401 => Err(PunchError::Api("Unauthorized – invalid API key".to_string())),
        _ => Err(PunchError::Api(format!("HTTP {status}: {text}"))),
    }
}
