use std::collections::HashMap;
use std::sync::Arc;

use actix_web::http::header::HeaderValue;
use actix_web::HttpRequest;
use actix_web::web::Data;
use chrono::Utc;

use crate::constant;
use crate::squire;

lazy_static::lazy_static! {
    static ref SESSION_MAPPING: std::sync::Mutex<HashMap<String, String>> = std::sync::Mutex::new(HashMap::new());
}

struct Credentials {
    username: String,
    signature: String,
    timestamp: String,
}

pub struct AuthToken {
    pub ok: bool,
    pub detail: String,
    pub username: String,
}


/// Extracts credentials from the authorization header in the following steps
///
/// 1. Decodes the base64 encoded header
///
/// 2. Splits it into 3 parts with first one being the username followed by the signature and timestamp
///
/// 3. Converts the username from hex into a string.
fn extract_credentials(authorization: Option<&HeaderValue>) -> Result<Credentials, &'static str> {
    let header = authorization.unwrap().to_str().unwrap().to_string();
    // fixme: use match with Ok and Err to extract error response
    // base64 encoded in JavaScript using inbuilt btoa function
    if let Ok(decoded_auth) = squire::secure::base64_decode(&header) {
        if decoded_auth.is_empty() {
            log::error!("Empty payload");  // todo: change this
            return Err("Authorization payload was empty");
        }
        let vector: Vec<&str> = decoded_auth.split(',').collect();
        return Ok(Credentials {
            // Decode hex username into string to retrieve password from config file
            username: squire::secure::hex_decode(vector.first().unwrap()),
            signature: vector.get(1).unwrap().to_string(),
            timestamp: vector.get(2).unwrap().to_string()
        });
    }
    return Err("Failed to extract credentials");
}

pub fn verify_login(
    request: &HttpRequest,
    config: &Data<Arc<squire::settings::Config>>,
) -> Result<HashMap<&'static str, String>, String> {
    let authorization = request.headers().get("authorization");
    let err_response;
    if authorization.is_some() {
        // fixme: use match with Ok and Err to extract error response
        if let Ok(credentials) = extract_credentials(authorization) {
            let password = config.authorization.get(&credentials.username);
            if password.is_some() {  // Check if the username is present in HashMap as key
                let message = format!("{}{}{}",
                                      squire::secure::hex_encode(&credentials.username),
                                      squire::secure::hex_encode(password.unwrap()),
                                      credentials.timestamp);
                // Create a new signature with hex encoded username and password stored in config file as plain text
                let expected_signature = squire::secure::calculate_hash(message);
                if expected_signature == credentials.signature {
                    let key = squire::secure::keygen();
                    SESSION_MAPPING.lock().unwrap().insert(credentials.username.to_string(), key.to_string());
                    let mut mapped = HashMap::new();
                    mapped.insert("username", credentials.username.to_string());
                    mapped.insert("key", key.to_string());
                    mapped.insert("timestamp", credentials.timestamp.to_string());
                    return Ok(mapped);
                } else {
                    err_response = format!("{} entered bad credentials", credentials.username);
                }
            } else {
                err_response = format!("{} is not allowed", credentials.username);
            }
        } else {
            err_response = "Failed to extract credentials".to_string();
        }
    } else {
        err_response = "No authorization headers were received".to_string();
    }
    return Err(err_response)
}

pub fn verify_token(request: &HttpRequest, config: &Data<Arc<squire::settings::Config>>) -> AuthToken {
    if SESSION_MAPPING.lock().unwrap().is_empty() {
        log::warn!("No stored sessions, no point in validating further");
        let ok = false;
        let detail = "".to_string();
        let username = "NA".to_string();
        return AuthToken { ok, detail, username };
    }
    if let Some(cookie) = request.cookie("session_token") {
        if let Ok(decrypted) = constant::FERNET.decrypt(cookie.value()) {
            let payload: HashMap<String, String> = serde_json::from_str(&String::from_utf8_lossy(&decrypted)).unwrap();
            let username = payload.get("username").unwrap().to_string();
            let cookie_key = payload.get("key").unwrap().to_string();
            let timestamp = payload.get("timestamp").unwrap().parse::<i64>().unwrap();
            let stored_key = SESSION_MAPPING.lock().unwrap().get(&username).unwrap().to_string();
            let current_time = Utc::now().timestamp();
            // Max time and expiry for session token is set in the Cookie, but this is a fallback mechanism
            if stored_key != *cookie_key {
                let ok = false;
                let detail = "Invalid session token".to_string();
                return AuthToken { ok, detail, username };
            }
            if current_time - timestamp > config.session_duration as i64 {
                let ok = false;
                let detail = "Session Expired".to_string();
                return AuthToken { ok, detail, username };
            }
            let ok = true;
            let detail = format!("Session valid for {}s", timestamp + config.session_duration as i64 - current_time);
            return AuthToken { ok, detail, username };
        }
    }
    let ok = false;
    let detail = "Invalid session token".to_string();
    let username = "NA".to_string();
    AuthToken { ok, detail, username }
}
