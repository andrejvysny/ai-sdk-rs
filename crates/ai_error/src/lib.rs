//! Error types for the AI SDK.
//!
//! This crate provides a unified error taxonomy for all AI SDK operations,
//! enabling consistent error handling and reporting across the ecosystem.

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

use std::time::Duration;
use thiserror::Error;

/// Main error type for AI SDK operations.
///
/// This error type covers all possible error conditions that can occur
/// during AI operations, from authentication failures to provider-specific errors.
#[derive(Error, Debug)]
pub enum AiError {
    /// Authentication with the provider failed.
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Rate limit exceeded. Retry after the specified duration if provided.
    #[error("Rate limit exceeded{}", .retry_after.map(|d| format!(", retry after {:?}", d)).unwrap_or_default())]
    RateLimit {
        /// Optional duration to wait before retrying
        retry_after: Option<Duration>,
    },

    /// Tool execution failed.
    #[error("Tool error in '{tool_name}': {message}")]
    Tool {
        /// Name of the tool that failed
        tool_name: String,
        /// Error message
        message: String,
    },

    /// Input validation failed.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Network error occurred.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Provider-specific error.
    #[error("Provider error ({provider}){}: {message}", .code.as_ref().map(|c| format!(" [{}]", c)).unwrap_or_default())]
    Provider {
        /// Provider name (e.g., "openai", "anthropic")
        provider: String,
        /// Error message
        message: String,
        /// Optional error code from the provider
        code: Option<String>,
    },

    /// Request timeout.
    #[error("Request timeout after {0:?}")]
    Timeout(Duration),

    /// Serialization or deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Internal SDK error.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Requested tool not found.
    #[error("No such tool: {0}")]
    NoSuchTool(String),

    /// Tool input validation failed.
    #[error("Invalid tool input for '{tool_name}': {reason}")]
    InvalidToolInput {
        /// Name of the tool with invalid input
        tool_name: String,
        /// Reason for validation failure
        reason: String,
    },

    /// JSON Schema validation failed.
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    /// Streaming error.
    #[error("Stream error: {0}")]
    Stream(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

impl AiError {
    /// Returns the error code suitable for wire format transmission.
    ///
    /// This maps error variants to standardized error codes that can be
    /// sent to clients via SSE or other protocols.
    pub fn error_code(&self) -> &str {
        match self {
            AiError::Auth(_) => "AUTH_ERROR",
            AiError::RateLimit { .. } => "RATE_LIMIT_ERROR",
            AiError::Tool { .. } => "TOOL_ERROR",
            AiError::Validation(_) => "VALIDATION_ERROR",
            AiError::Network(_) => "NETWORK_ERROR",
            AiError::Provider { .. } => "PROVIDER_ERROR",
            AiError::Timeout(_) => "TIMEOUT_ERROR",
            AiError::Serialization(_) => "SERIALIZATION_ERROR",
            AiError::Internal(_) => "INTERNAL_ERROR",
            AiError::NoSuchTool(_) => "TOOL_ERROR",
            AiError::InvalidToolInput { .. } => "TOOL_ERROR",
            AiError::SchemaValidation(_) => "VALIDATION_ERROR",
            AiError::Stream(_) => "STREAM_ERROR",
            AiError::Config(_) => "CONFIG_ERROR",
        }
    }

    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AiError::RateLimit { .. } | AiError::Network(_) | AiError::Timeout(_)
        )
    }

    /// Returns the suggested retry delay for retryable errors.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            AiError::RateLimit { retry_after } => *retry_after,
            AiError::Network(_) => Some(Duration::from_secs(1)),
            AiError::Timeout(_) => Some(Duration::from_secs(2)),
            _ => None,
        }
    }
}

/// Result type alias using [`AiError`].
pub type Result<T> = std::result::Result<T, AiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(
            AiError::Auth("failed".into()).error_code(),
            "AUTH_ERROR"
        );
        assert_eq!(
            AiError::Tool {
                tool_name: "test".into(),
                message: "failed".into()
            }
            .error_code(),
            "TOOL_ERROR"
        );
        assert_eq!(
            AiError::Validation("invalid".into()).error_code(),
            "VALIDATION_ERROR"
        );
    }

    #[test]
    fn test_retryable() {
        assert!(AiError::RateLimit {
            retry_after: Some(Duration::from_secs(5))
        }
        .is_retryable());
        assert!(!AiError::Auth("failed".into()).is_retryable());
        assert!(!AiError::Validation("invalid".into()).is_retryable());
    }

    #[test]
    fn test_retry_after() {
        let error = AiError::RateLimit {
            retry_after: Some(Duration::from_secs(10)),
        };
        assert_eq!(error.retry_after(), Some(Duration::from_secs(10)));

        let error = AiError::Auth("failed".into());
        assert_eq!(error.retry_after(), None);
    }
}
