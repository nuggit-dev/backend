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

use crate::storage::Storage;
use crate::Repo;

// Represents a service error.
#[derive(Debug)]
pub enum Error {
    // Returned if a repository already exists.
    AlreadyExists,
    // Returned if a repository is not found.
    NotFound,
}

// Manages repositories and their metadata.
pub struct Service<T: Storage> {
    storage: T,
}

impl<T: Storage> Service<T> {
    // Creates a new service.
    pub fn new(storage: T) -> Service<T> {
        Service { storage }
    }
    // Creates a repository.
    pub fn create(&mut self, name: &str, description: &str, creator: &str) -> Result<Repo, Error> {
        self.storage
            .create(name, description, creator)
            .ok_or(Error::AlreadyExists)
    }
    // Retrieves a repository.
    pub fn retrieve(&self, name: &str) -> Result<Repo, Error> {
        self.storage.retrieve(name).ok_or(Error::NotFound)
    }
}
