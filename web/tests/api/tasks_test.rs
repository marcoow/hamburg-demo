use axum::{
    body::Body,
    http::{self, Method},
};
use fake::{Fake, Faker};
use googletest::prelude::*;
use hamburg_demo_db::{entities, transaction, Error};
use hamburg_demo_macros::db_test;
use hamburg_demo_web::test_helpers::{BodyExt, DbTestContext, RouterExt};
use hyper::StatusCode;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

async fn test_create_invalid(context: &DbTestContext) {
    todo!("send invalid changeset, assert 422 response!");

    /* Example:
    let payload = json!(entities::tasks::TaskChangeset {
        description: String::from("")
    });

    let response = context
        .app
        .request("/tasks")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));
    */
}

async fn test_create_success(context: &DbTestContext) {
    todo!("send valid changeset, assert 201 response!");

    /* Example:
    let changeset: entities::tasks::TaskChangeset = Faker.fake();
    let payload = json!(changeset);

    let response = context
        .app
        .request("/tasks")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::CREATED));

    let tasks = entities::tasks::load_all(&context.db_pool).await.unwrap();
    assert_that!(tasks, len(eq(1)));
    assert_that!(
        tasks.first().unwrap().description,
        eq(&changeset.description)
    );
    */
}

#[db_test]
async fn test_read_all(context: &DbTestContext) {
    let changeset: entities::tasks::TaskChangeset = Faker.fake();
    entities::tasks::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context.app.request("/tasks").send().await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let tasks: Vec<entities::tasks::Task> = response
        .into_body()
        .into_json::<Vec<entities::tasks::Task>>()
        .await;
    assert_that!(tasks, len(eq(1)));
    assert_that!(
        tasks.first().unwrap().description,
        eq(&changeset.description)
    );
}

async fn test_read_one_nonexistent(context: &DbTestContext) {
    todo!("read non-existent entity, assert 404 response!");

    /* Example:
    let response = context
        .app
        .request(&format!("/tasks/{}", Uuid::new_v4()))
        .body(payload.to_string())
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
    */
}

async fn test_read_one_success(context: &DbTestContext) {
    todo!("arrange DB, load entity, assert it is returned!");

    /* Example:
    let task_changeset: entities::tasks::TaskChangeset = Faker.fake();
    let task = create_tasktask(task_changeset.clone(), &context.db_pool)
        .await
        .unwrap();
    let task_id = task.id;

    let response = context
        .app
        .request(&format!("/tasks/{}", task_id))
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let task: entities::tasks::Task = response.into_body().into_json::<entities::tasks::Task>(response).await;
    assert_that!(task.id, eq(task_id));
    assert_that!(task.description, eq(task_changeset.description));
    */
}

async fn test_update_invalid(context: &DbTestContext) {
    todo!("arrange DB, send invalid changeset, assert 422 response and nothing changes in DB!");

    /* Example:
    let task_changeset: entities::tasks::TaskChangesetChangeset = Faker.fake();
    let task = create_task(task_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(entities::tasks::TaskChangeset {
        description: String::from("")
    });

    let response = context
        .app
        .request(&format!("/tasks/{}", task_id))
        .method(Method::PUT)
        .body(payload.to_string())
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));

    let task_after = load_task(task.id, &context.db_pool).await.unwrap();
    assert_that!(task_after.description, eq(task.description));
    */
}

async fn test_update_nonexistent(context: &DbTestContext) {
    todo!("send valid changeset for non-existent ID, assert 404 response!");

    /* Example:
    let task_changeset: entities::tasks::TaskChangeset = Faker.fake();
    let payload = json!(task_changeset);

    let response = context
        .app
        .request(&format!("/tasks/{}", Uuid::new_v4()))
        .method(Method::PUT)
        .body(payload.to_string())
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
    */
}

async fn test_update_success(context: &DbTestContext) {
    todo!("arrange DB, send valid changeset, assert 200 response and changes applied in DB!");

    /* Example:
    let task_changeset: entities::tasks::TaskChangeset = Faker.fake();
    let task = create_task(task_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let task_changeset: entities::tasks::TaskChangeset = Faker.fake();
    let payload = json!(task_changeset);

    let response = context
        .app
        .request(&format!("/tasks/{}", task.id))
        .method(Method::PUT)
        .body(payload.to_string())
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let task: entities::tasks::Task = response.into_body.into_json::<Task>(response).await;
    assert_that!(task.description, eq(task_changeset.description.clone()));

    let task = load_task(task.id, &context.db_pool).await.unwrap();
    assert_that!(task.description, eq(task_changeset.description));
    */
}

async fn test_delete_nonexistent(context: &DbTestContext) {
    todo!("delete non-existing ID, assert 404 response!");

    /* Example:
    let response = context
        .app
        .request(&format!("/tasks/{}", Uuid::new_v4()))
        .method(Method::DELETE)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
    */
}

async fn test_delete_success(context: &DbTestContext) {
    todo!("arrange DB, delete, assert 204 response and record deleted in dB!");

    /* Example:
    let task_changeset: entities::tasks::TaskChangeset = Faker.fake();
    let task = create_task(task_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/tasks/{}", Uuid::new_v4()))
        .method(Method::DELETE)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NO_CONTENT));

    let result = load_task(task.id, &context.db_pool).await;
    assert_that!(result, err(anything()));
    */
}
