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

extern crate nuggit;
use crate::nuggit::storage::Storage;

#[test]
fn create_ok_if_repo_does_not_exist() {
    let mut s = nuggit::storage::InMemory::new();

    let expected = nuggit::Repo {
        name: String::from("some"),
        description: String::from("test"),
        creator: String::from("bob"),
        created: String::from("2020-04-28T13:48:01.778470"),
    };

    let r = s
        .create(&expected.name, &expected.description, &expected.creator)
        .unwrap();

    assert_eq!(r, expected)
}

#[test]
fn create_error_if_repo_already_exists() {
    let mut s = nuggit::storage::InMemory::new();
    s.create("test", "", "").unwrap();
    assert!(s.create("test", "", "").is_none());
}

#[test]
fn retrieve_error_if_repo_does_not_exist() {
    let s = nuggit::storage::InMemory::new();
    assert!(s.retrieve("test").is_none());
}

#[test]
fn retrieve_ok_if_repo_exists() {
    let mut s = nuggit::storage::InMemory::new();

    let expected = nuggit::Repo {
        name: String::from("some"),
        description: String::from("test"),
        creator: String::from("bob"),
        created: String::from("2020-04-28T13:48:01.778470"),
    };

    s.create(&expected.name, &expected.description, &expected.creator)
        .unwrap();

    let r = s.retrieve(&expected.name).unwrap();

    assert_eq!(r, expected)
}
