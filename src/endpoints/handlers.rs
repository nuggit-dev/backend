// nuggit is a minimalistic, fast and secure hosting for private Git repositories.
// Copyright (C) 2020  Elisey Zanko
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::{service, Service};

impl warp::reject::Reject for service::Error {}

/// Represents a repository creation request.
#[derive(Serialize, Deserialize, Default)]
pub struct CreateRepoRequest {
    /// The name of the repository.
    pub name: String,
    /// A short description of the repository.
    #[serde(default)]
    pub description: String,
}

/// Represents a response indicating an error.
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    /// A short string with a brief explanation of the error.
    pub code: String,
    /// A human-readable message providing more details about the error.
    pub message: String,
}

/// Create a repository.
pub async fn create_repo(
    request: CreateRepoRequest,
    mut service: impl Service,
) -> Result<impl Reply, Rejection> {
    let r = service
        .create(&request.name, &request.description, "anonymous")
        .await;

    match r {
        Ok(repo) => Ok(warp::reply::json(&repo)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

/// Retrieve a repository.
pub async fn retrieve_repo(name: String, service: impl Service) -> Result<impl Reply, Rejection> {
    let r = service.retrieve(&name).await;

    match r {
        Ok(repo) => Ok(warp::reply::json(&repo)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

/// Handle rejection.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    // We won't reveal any details about unhandled rejections.
    let mut code = "internal_error";
    let mut message = "The server encountered an internal error.";
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    // Service errors.
    if let Some(e) = err.find::<service::Error>() {
        match e {
            service::Error::NotFound => {
                code = "repo_not_found";
                message = "The repository with such name was not found.";
                status = StatusCode::NOT_FOUND;
            }
            service::Error::AlreadyExists => {
                code = "repo_exists";
                message = "The repository with such name already exists.";
                status = StatusCode::CONFLICT;
            }
            service::Error::InvalidName => {
                code = "invalid_repo_name";
                message =
                    "Repository name is invalid. It must be an ASCII string up to 64 characters.";
                status = StatusCode::BAD_REQUEST;
            }
            service::Error::InvalidDescription => {
                code = "invalid_repo_description";
                message =
                    "Repository description is invalid. It must be a UTF-8 encoded string up to 256 characters.";
                status = StatusCode::BAD_REQUEST;
            }
            service::Error::NotImplemented => {
                code = "not_implemented";
                message = "The method is not implemented.";
                status = StatusCode::NOT_IMPLEMENTED;
            }
        };
    }
    // warp rejections.
    // Maybe there's a better way than calling `err.find()` this many times.
    else if let Some(_) = err.find::<warp::reject::InvalidHeader>() {
        code = "bad_request";
        message = "Request header is invalid.";
        status = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        code = "bad_request";
        message = "Query string is invalid.";
        status = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::LengthRequired>() {
        code = "length_required";
        message = "A content-length header is required.";
        status = StatusCode::LENGTH_REQUIRED;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = "method_not_allowed";
        message = "HTTP method not allowed.";
        status = StatusCode::METHOD_NOT_ALLOWED;
    } else if let Some(_) = err.find::<warp::reject::MissingCookie>() {
        code = "bad_request";
        message = "Cookie is missing.";
        status = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MissingHeader>() {
        code = "bad_request";
        message = "Request header is missing.";
        status = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        code = "payload_too_large";
        message = "The request payload is too large.";
        status = StatusCode::PAYLOAD_TOO_LARGE;
    } else if let Some(_) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = "unsupported_media_type";
        message = "The request's content-type is not supported.";
        status = StatusCode::UNSUPPORTED_MEDIA_TYPE;
    } else if let Some(_) = err.find::<warp::body::BodyDeserializeError>() {
        code = "bad_request";
        message = "Request body is invalid.";
        status = StatusCode::BAD_REQUEST;
    }
    // Unhandled rejections must be logged.
    else {
        eprintln!("Unhandled rejection: {:?}", err);
    }

    let json = warp::reply::json(&ErrorResponse {
        code: code.into(),
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, status))
}
