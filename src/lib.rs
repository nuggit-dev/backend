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

use serde::{Deserialize, Serialize};

/// Represents repository metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    /// The name of the repository.
    pub name: String,
    /// A short description of the repository.
    pub description: String,
    /// ID of the user who created the repository.
    pub creator: String,
    /// Date and time at which the repository was created.
    pub created: String,
}

pub mod endpoints;

pub mod service;
pub use service::Nuggit;
pub use service::Service;

pub mod storage;
pub use storage::Storage;
