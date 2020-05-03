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

use crate::service::Error;
use crate::storage::Storage;
use crate::{Repo, Service};
use async_trait::async_trait;

/// Manages repositories and their metadata.
#[derive(Clone)]
pub struct Nuggit<T> {
    storage: T,
}

impl<T> Nuggit<T>
where
    T: Storage,
{
    /// Creates a new service.
    pub fn new(storage: T) -> Nuggit<T> {
        Nuggit { storage }
    }
}

#[async_trait]
impl<T> Service for Nuggit<T>
where
    T: Storage,
{
    /// Creates a repository if its `name` and `description` is valid.
    async fn create(
        &mut self,
        name: &str,
        description: &str,
        creator: &str,
    ) -> Result<Repo, Error> {
        if name.is_empty() || name.len() > 64 || !name.is_ascii() {
            return Err(Error::InvalidName);
        }

        // Description is UTF-8, so we count Unicode Scalar Values.
        if description.chars().count() > 256 {
            return Err(Error::InvalidDescription);
        }

        let r = self.storage.create(name, description, creator).await;
        r.ok_or(Error::AlreadyExists)
    }

    /// Retrieves a repository.
    async fn retrieve(&self, name: &str) -> Result<Repo, Error> {
        let r = self.storage.retrieve(name).await;
        r.ok_or(Error::NotFound)
    }
}
