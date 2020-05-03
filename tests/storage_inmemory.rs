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

#[tokio::test]
async fn create_ok_if_repo_does_not_exist() {
    let mut s = nuggit::storage::InMemory::new();

    let expected = nuggit::Repo {
        name: String::from("some"),
        description: String::from("test"),
        creator: String::from("bob"),
        created: String::from("2020-04-28T13:48:01.778470"),
    };

    let r = s
        .create(&expected.name, &expected.description, &expected.creator)
        .await
        .unwrap();

    assert_eq!(r, expected)
}

#[tokio::test]
async fn create_none_if_repo_already_exists() {
    let mut s = nuggit::storage::InMemory::new();
    s.create("test", "", "").await.unwrap();
    let r = s.create("test", "", "").await;
    assert!(r.is_none());
}

#[tokio::test]
async fn retrieve_none_if_repo_does_not_exist() {
    let s = nuggit::storage::InMemory::new();
    let r = s.retrieve("test").await;
    assert!(r.is_none());
}

#[tokio::test]
async fn retrieve_some_if_repo_exists() {
    let mut s = nuggit::storage::InMemory::new();

    let expected = nuggit::Repo {
        name: String::from("some"),
        description: String::from("test"),
        creator: String::from("bob"),
        created: String::from("2020-04-28T13:48:01.778470"),
    };

    s.create(&expected.name, &expected.description, &expected.creator)
        .await
        .unwrap();

    let r = s.retrieve(&expected.name).await.unwrap();

    assert_eq!(r, expected)
}
