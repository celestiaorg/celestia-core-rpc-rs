//! This module defines the `Authorization` type for
//! authorizing a HTTP or WebSocket RPC client using
//! HTTP Basic authentication.

use alloc::{
    format,
    string::{String, ToString},
};
use core::fmt;

use subtle_encoding::base64;
use url::Url;

/// An HTTP authorization.
///
/// Currently only HTTP Basic authentication is supported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Authorization {
    Basic(String),
}

impl fmt::Display for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic(cred) => write!(f, "Basic {cred}"),
        }
    }
}

/// Extract the authorization, if any, from the authority part of the given URI.
///
/// This authorization can then be supplied to the RPC server via
/// the `Authorization` HTTP header.
pub fn authorize(uri: &str) -> Option<Authorization> {
    let url = Url::parse(uri).ok()?;
    let username = url.username();
    if username.is_empty() {
        return None;
    }

    let password = url.password().unwrap_or_default();
    let userpass = if password.is_empty() {
        username.to_string()
    } else {
        format!("{username}:{password}")
    };

    let bytes = base64::encode(userpass);
    let credentials = String::from_utf8_lossy(bytes.as_slice());
    Some(Authorization::Basic(credentials.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_auth_absent() {
        assert_eq!(authorize("http://example.com"), None);
    }

    #[test]
    fn extract_auth_username_only() {
        let base64 = "dG90bw==".to_string();
        assert_eq!(
            authorize("http://toto@example.com"),
            Some(Authorization::Basic(base64))
        );
    }

    #[test]
    fn extract_auth_username_password() {
        let base64 = "dG90bzp0YXRh".to_string();
        assert_eq!(
            authorize("http://toto:tata@example.com"),
            Some(Authorization::Basic(base64))
        );
    }
}
