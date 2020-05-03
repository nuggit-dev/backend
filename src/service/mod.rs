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

use crate::Repo;
use async_trait::async_trait;

/// Represents a service error.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Returned if a repository already exists.
    AlreadyExists,
    /// Returned if a repository is not found.
    NotFound,
    /// Returned if repository name is invalid.
    InvalidName,
    /// Returned if repository description is invalid.
    InvalidDescription,
    /// Returned if a method is not implemented.
    NotImplemented,
}

/// Represents a service that manages repositories and their metadata.
#[async_trait]
pub trait Service: Send + Sync + Clone {
    /// Create a repository.
    async fn create(&mut self, name: &str, description: &str, creator: &str)
        -> Result<Repo, Error>;
    /// Retrieve a repository.
    async fn retrieve(&self, name: &str) -> Result<Repo, Error>;
}

pub mod nuggit;
pub use self::nuggit::Nuggit;
