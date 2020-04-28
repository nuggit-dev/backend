// nuggit-server implements the Nuggit REST API.
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

// Represents a repository.
struct Repo {
    // The name of the repository.
    name: String,
    // A short description of the repository.
    description: String,
    // ID of the user who created the repository.
    creator: String,
    // Date and time at which the repository was created.
    created: String,
}

fn main() {
    println!("Hello, world!");
}
