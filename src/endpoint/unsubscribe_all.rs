//! `/unsubscribe_all` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{dialect::Dialect, request::RequestMessage};

/// Request to unsubscribe from all events.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::UnsubscribeAll
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

/// Status responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {}

impl crate::Response for Response {}
