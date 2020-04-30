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
use std::iter::FromIterator;

mod mock;

#[test]
fn create_error_if_name_is_not_ascii() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Service::new(m);

    // Note the fancy f!
    let name = "ƒoo";
    let err = s.create(name, "", "").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[test]
fn create_error_if_name_is_too_long() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Service::new(m);

    // Maybe there's a shorter way to create a long string.
    let name = String::from_iter(['t'; 65].iter());
    let err = s.create(name.as_str(), "", "").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[test]
fn create_error_if_name_is_empty() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Service::new(m);

    let name = "";
    let err = s.create(name, "", "").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidName);
}

#[test]
fn create_error_if_description_is_too_long() {
    let m: mock::storage::Mock = Default::default();
    let mut s = nuggit::Service::new(m);

    // Maybe there's a shorter way to create a long string.
    let description = String::from_iter(['t'; 257].iter());
    let err = s.create("test", description.as_str(), "").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::InvalidDescription);
}

#[test]
fn create_ok_if_description_is_empty() {
    let create_fn = || {
        Some(Repo {
            ..Default::default()
        })
    };
    let m = mock::storage::Mock {
        create_fn: Some(create_fn),
        retrieve_fn: None,
    };
    let mut s = nuggit::Service::new(m);

    let description = "";
    assert!(s.create("test", description, "").is_ok());
}

#[test]
fn create_error_if_storage_returns_none() {
    let create_fn = || None;
    let m = mock::storage::Mock {
        create_fn: Some(create_fn),
        retrieve_fn: None,
    };
    let mut s = nuggit::Service::new(m);

    let err = s.create("test", "", "").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::AlreadyExists);
}

#[test]
fn create_ok_if_storage_returns_some() {
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
    let mut s = nuggit::Service::new(m);

    let r = s.create("test", "", "").unwrap();
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

#[test]
fn retrieve_error_if_storage_returns_none() {
    let retrieve_fn = || None;
    let m = mock::storage::Mock {
        create_fn: None,
        retrieve_fn: Some(retrieve_fn),
    };
    let s = nuggit::Service::new(m);

    let err = s.retrieve("test").err();
    assert!(err.is_some());
    assert_eq!(err.unwrap(), Error::NotFound);
}

#[test]
fn retrieve_ok_if_storage_returns_some() {
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
    let s = nuggit::Service::new(m);

    let r = s.retrieve("test").unwrap();
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
