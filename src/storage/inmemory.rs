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

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::storage::Storage;
use crate::Repo;

/// Implements in-memory storage of repository metadata.
/// Note, that the implementation is not efficient because it does a lot of copying.
/// It's only meant for testing.
#[derive(Clone)]
pub struct InMemory {
    map: Arc<RwLock<HashMap<String, Repo>>>,
}

impl InMemory {
    /// Creates an empty storage.
    pub fn new() -> InMemory {
        InMemory {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for InMemory {
    /// Creates a repository.
    async fn create(&mut self, name: &str, description: &str, creator: &str) -> Option<Repo> {
        let mut map = self.map.write().await;

        if map.contains_key(name) {
            return None;
        }

        let created = "2020-04-28T13:48:01.778470";
        map.insert(
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
    async fn retrieve(&self, name: &str) -> Option<Repo> {
        let map = self.map.read().await;
        match map.get(name) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }
}
