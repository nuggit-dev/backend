extern crate nuggit;

use warp::http::StatusCode;
use warp::test::request;

use nuggit::endpoints::{CreateRepoRequest, ErrorResponse};
use nuggit::Repo;

mod mock;

#[tokio::test]
async fn error_if_url_doesn_not_exist() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request().method("GET").path("/test").reply(&api).await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(err.code, "not_found");
}

#[tokio::test]
async fn create_repo_error_if_method_is_not_allowed() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let methods = [
        "GET", "HEAD", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
    ];
    for m in methods.iter() {
        let resp = request().method(m).path("/repos").reply(&api).await;
        let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{} is allowed",
            m
        );
        assert_eq!(err.code, "method_not_allowed");
    }
}

#[tokio::test]
async fn create_repo_error_if_request_body_is_empty() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request().method("POST").path("/repos").reply(&api).await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_request_body_is_not_json() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .body("test")
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_request_body_is_invalid_json() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body("{")
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_missing() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body("{}")
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_not_string() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body(r#"{"name": 666}"#)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_null() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body(r#"{"name": null}"#)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_empty() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "".into(),
        description: "".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "repo_name_invalid");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_too_long() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "t".repeat(65),
        description: "".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "repo_name_invalid");
}

#[tokio::test]
async fn create_repo_error_if_repo_name_is_not_ascii() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    // Note the fancy f!
    let req = CreateRepoRequest {
        name: "ƒoo".into(),
        description: "".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "repo_name_invalid");
}

#[tokio::test]
async fn create_repo_error_if_repo_description_is_not_string() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body(r#"{"name": "test", "description": 666}"#)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_description_is_null() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body(r#"{"name": "test", "description": null}"#)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "bad_request");
}

#[tokio::test]
async fn create_repo_error_if_repo_description_is_too_long() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "test".into(),
        description: "t".repeat(257),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(err.code, "repo_description_invalid");
}

#[tokio::test]
async fn create_repo_ok_if_repo_description_is_missing() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("POST")
        .path("/repos")
        .header("Content-Type", "application/json")
        .body(r#"{"name": "test"}"#)
        .reply(&api)
        .await;
    let repo: Repo = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        repo,
        Repo {
            name: "test".into(),
            description: "".into(),
            creator: "anonymous".into(),
            created: "2020-04-28T13:48:01.778470".into(),
        }
    );
}

#[tokio::test]
async fn create_repo_ok_if_repo_description_is_empty() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "test".into(),
        description: "".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let repo: Repo = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        repo,
        Repo {
            name: "test".into(),
            description: "".into(),
            creator: "anonymous".into(),
            created: "2020-04-28T13:48:01.778470".into(),
        }
    );
}

#[tokio::test]
async fn create_repo_ok() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "test".into(),
        description: "some".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let repo: Repo = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        repo,
        Repo {
            name: "test".into(),
            description: "some".into(),
            creator: "anonymous".into(),
            created: "2020-04-28T13:48:01.778470".into(),
        }
    );
}

#[tokio::test]
async fn create_repo_error_if_repo_already_exists() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "test".into(),
        description: "".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(err.code, "repo_exists");
}

#[tokio::test]
async fn retrieve_repo_error_if_method_is_not_allowed() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let methods = [
        "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
    ];
    for m in methods.iter() {
        let resp = request()
            .method(m)
            .path(format!("/repos/{name}", name = "test").as_str())
            .reply(&api)
            .await;
        let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{} is allowed",
            m
        );
        assert_eq!(err.code, "method_not_allowed");
    }
}

#[tokio::test]
async fn retrieve_repo_error_if_repo_doesn_not_exist() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let resp = request()
        .method("GET")
        .path(format!("/repos/{name}", name = "test").as_str())
        .reply(&api)
        .await;
    let err: ErrorResponse = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(err.code, "not_found");
}

#[tokio::test]
async fn retrieve_repo_ok() {
    let storage = nuggit::storage::InMemory::new();
    let service = nuggit::Nuggit::new(storage);
    let api = nuggit::endpoints::make(service);

    let req = CreateRepoRequest {
        name: "test".into(),
        description: "some".into(),
    };
    let resp = request()
        .method("POST")
        .path("/repos")
        .json(&req)
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp = request()
        .method("GET")
        .path(format!("/repos/{name}", name = "test").as_str())
        .reply(&api)
        .await;
    let repo: Repo = serde_json::from_slice(resp.body()).unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        repo,
        Repo {
            name: "test".into(),
            description: "some".into(),
            creator: "anonymous".into(),
            created: "2020-04-28T13:48:01.778470".into(),
        }
    );
}
