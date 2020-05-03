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

use async_trait::async_trait;
use nuggit::{Repo, Storage};

/// Mocks a storage.
#[derive(Clone, Default)]
pub struct Mock {
    /// If set, the result of calling this function will be returned from `create()`.
    pub create_fn: Option<fn() -> Option<Repo>>,
    /// If set, the result of calling this function will be returned from `retrieve()`.
    pub retrieve_fn: Option<fn() -> Option<Repo>>,
}

#[async_trait]
impl Storage for Mock {
    /// Calls `create_fn` if it is not `None` and returns the result.
    /// Returns `None` otherwise.
    async fn create(&mut self, _name: &str, _description: &str, _creator: &str) -> Option<Repo> {
        if let Some(f) = self.create_fn {
            return f();
        }
        None
    }

    /// Calls `retrieve_fn` if it is not `None` and returns the result.
    /// Returns `None` otherwise.
    async fn retrieve(&self, _name: &str) -> Option<Repo> {
        if let Some(f) = self.retrieve_fn {
            return f();
        }
        None
    }
}
