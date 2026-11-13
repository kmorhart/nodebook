use std::collections::HashMap;
use std::sync::LazyLock;

pub static ERROR_MESSAGES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    
    // auth
    m.insert("auth.invalid_credentials", "Invalid credentials");
    m.insert("auth.token_expired", "Session has expired");
    m.insert("auth.token_invalid", "Invalid token");
    m.insert("auth.token_missing", "Token is missing");
    m.insert("auth.unauthorized", "Unauthorized");
    m.insert("auth.lockout", "Please try again later.");

    // user
    m.insert("user.email_taken", "Email already in use");
    m.insert("user.username_taken", "Username already in use");
    m.insert("user.not_found", "User not found");

    // db
    m.insert("db.connection_failed", "Service unavailable");
    m.insert("db.query_failed", "An error occurred");

    // general
    m.insert("general.internal", "Internal server error");
    m.insert("general.method_not_allowed", "HTTP method not allowed");
    m.insert("general.bad_request", "Invalid request");
    m.insert("general.conflict", "Request conflict");
    m.insert("general.not_found", "Resource not found");

    m
});

pub fn get(key: &str) -> &'static str {
    ERROR_MESSAGES.get(key).unwrap_or(&"An unexpected error occurred")
}