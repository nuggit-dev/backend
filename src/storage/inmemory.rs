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
use std::collections::HashMap;

/// Implements in-memory storage of repository metadata.
/// Note, that this storage is neither thread-safe nor efficient.
/// It's only meant for testing.
pub struct InMemory {
    s: HashMap<String, Repo>,
}

impl InMemory {
    /// Creates an empty storage.
    pub fn new() -> InMemory {
        InMemory { s: HashMap::new() }
    }
}

impl Storage for InMemory {
    /// Creates a repository.
    fn create(&mut self, name: &str, description: &str, creator: &str) -> Option<Repo> {
        if self.s.contains_key(name) {
            return None;
        }

        let created = "2020-04-28T13:48:01.778470";
        self.s.insert(
            name.to_owned(),
            Repo {
                name: name.to_owned(),
                description: description.to_owned(),
                creator: creator.to_owned(),
                created: created.to_owned(),
            },
        );

        Some(Repo {
            name: name.to_owned(),
            description: description.to_owned(),
            creator: creator.to_owned(),
            created: created.to_owned(),
        })
    }

    /// Retrieves a repository.
    fn retrieve(&self, name: &str) -> Option<Repo> {
        match self.s.get(name) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }
}
