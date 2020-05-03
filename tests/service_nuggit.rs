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

use nuggit::service::Error;
use nuggit::Repo;
use nuggit::Service;

mod mock;

#[tokio::test]
async fn create_error_if_name_is_not_ascii() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Nuggit::new(m);

    // Note the fancy f!
    let name = "ƒoo";
    let err = s.create(name, "", "").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[tokio::test]
async fn create_error_if_name_is_too_long() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Nuggit::new(m);

    let name = "t".repeat(65);
    let err = s.create(name.as_str(), "", "").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[tokio::test]
async fn create_error_if_name_is_empty() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Nuggit::new(m);

    let name = "";
    let err = s.create(name, "", "").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[tokio::test]
async fn create_error_if_description_is_too_long() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Nuggit::new(m);

    let description = "t".repeat(257);
    let err = s.create("test", description.as_str(), "").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidDescription);
}

#[tokio::test]
async fn create_ok_if_description_is_empty() {
    let create_fn = || {
        Some(Repo {
            ..Default::default()
        })
    };
    let m = mock::storage::Mock {
        create_fn: Some(create_fn),
        retrieve_fn: None,
    };
    let mut s = nuggit::Nuggit::new(m);

    let description = "";
    assert!(s.create("test", description, "").await.is_ok());
}

#[tokio::test]
async fn create_error_if_storage_returns_none() {
    let create_fn = || None;
    let m = mock::storage::Mock {
        create_fn: Some(create_fn),
        retrieve_fn: None,
    };
    let mut s = nuggit::Nuggit::new(m);

    let err = s.create("test", "", "").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::AlreadyExists);
}

#[tokio::test]
async fn create_ok_if_storage_returns_some() {
    let create_fn = || {
        Some(Repo {
            name: String::from("some"),
            description: String::from("test"),
            creator: String::from("bob"),
            created: String::from("2020-04-28T13:48:01.778470"),
        })
    };
    let m = mock::storage::Mock {
        create_fn: Some(create_fn),
        retrieve_fn: None,
    };
    let mut s = nuggit::Nuggit::new(m);

    let r = s.create("test", "", "").await.unwrap();
    assert_eq!(
        r,
        Repo {
            name: String::from("some"),
            description: String::from("test"),
            creator: String::from("bob"),
            created: String::from("2020-04-28T13:48:01.778470"),
        }
    );
}

#[tokio::test]
async fn retrieve_error_if_storage_returns_none() {
    let retrieve_fn = || None;
    let m = mock::storage::Mock {
        create_fn: None,
        retrieve_fn: Some(retrieve_fn),
    };
    let s = nuggit::Nuggit::new(m);

    let err = s.retrieve("test").await.err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::NotFound);
}

#[tokio::test]
async fn retrieve_ok_if_storage_returns_some() {
    let retrieve_fn = || {
        Some(Repo {
            name: String::from("some"),
            description: String::from("test"),
            creator: String::from("bob"),
            created: String::from("2020-04-28T13:48:01.778470"),
        })
    };
    let m = mock::storage::Mock {
        create_fn: None,
        retrieve_fn: Some(retrieve_fn),
    };
    let s = nuggit::Nuggit::new(m);

    let r = s.retrieve("test").await.unwrap();
    assert_eq!(
        r,
        Repo {
            name: String::from("some"),
            description: String::from("test"),
            creator: String::from("bob"),
            created: String::from("2020-04-28T13:48:01.778470"),
        }
    );
}
