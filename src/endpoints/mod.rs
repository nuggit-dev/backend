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

use warp::{Filter, Rejection, Reply};

use crate::endpoints::filters::with_service;
use crate::Service;

mod filters;
mod handlers;

pub use handlers::{CreateRepoRequest, ErrorResponse};

/// Combines all endpoints into a single API.
pub fn make(
    service: impl Service,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    make_create_repo(service.clone())
        .or(make_retrieve_repo(service))
        .recover(handlers::handle_rejection)
}

/// Create a repository.
///
/// `POST /repos`
fn make_create_repo(
    service: impl Service,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("repos")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_service(service))
        .and_then(handlers::create_repo)
}

/// Retrieve a repository.
///
/// `GET /repos/:name`
fn make_retrieve_repo(
    service: impl Service,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("repos" / String)
        .and(warp::get())
        .and(with_service(service))
        .and_then(handlers::retrieve_repo)
}
