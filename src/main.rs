use std::env;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use tokio::task::JoinSet;

const API_BASE_URL: &str = "https://jules.googleapis.com/v1alpha";
const MAX_PAGE_SIZE: usize = 100;

#[derive(Debug, Deserialize)]
struct ListSessionsResponse {
    #[serde(default)]
    sessions: Vec<Session>,
}

#[derive(Clone, Debug, Deserialize)]
struct Session {
    name: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("JULES_API_KEY")
        .context("JULES_API_KEY is not set. Ensure it is configured in GitHub Actions secrets.")?;

    if api_key.trim().is_empty() {
        bail!("JULES_API_KEY is empty. Ensure a valid key is set in GitHub Actions secrets.");
    }

    let client = build_client(&api_key)?;
    let started_at = Instant::now();
    let deleted = delete_sessions(&client).await?;

    println!("Deleted sessions: {}", deleted);
    println!("Elapsed: {:.2?}", started_at.elapsed());

    Ok(())
}

fn build_client(api_key: &str) -> Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-goog-api-key",
        HeaderValue::from_str(api_key)
            .context("JULES_API_KEY contains invalid header characters.")?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .context("failed to build HTTP client")
}

async fn delete_sessions(client: &reqwest::Client) -> Result<usize> {
    let mut total_deleted = 0usize;

    loop {
        // Re-list from the first page after each batch so shrinking results do not leave gaps.
        let sessions = list_sessions(client, MAX_PAGE_SIZE).await?;
        if sessions.is_empty() {
            break;
        }

        let batch_deleted = delete_batch(client, sessions).await?;
        total_deleted += batch_deleted;

        println!("Deleted so far: {}", total_deleted);

        if batch_deleted == 0 {
            bail!("no sessions were deleted in the last batch");
        }
    }

    Ok(total_deleted)
}

async fn list_sessions(client: &reqwest::Client, page_size: usize) -> Result<Vec<Session>> {
    let response = client
        .get(format!("{API_BASE_URL}/sessions"))
        .query(&[("pageSize", page_size)])
        .send()
        .await
        .context("failed to request session list")?
        .error_for_status()
        .context("Jules API rejected session list request")?;

    let payload = response
        .json::<ListSessionsResponse>()
        .await
        .context("failed to decode session list response")?;

    Ok(payload.sessions)
}

async fn delete_batch(client: &reqwest::Client, sessions: Vec<Session>) -> Result<usize> {
    let batch_size = sessions.len();
    let started_at = Instant::now();
    let mut deleted = 0usize;
    let mut tasks = JoinSet::new();

    for session in sessions {
        let client = client.clone();
        tasks.spawn(async move { delete_session(&client, &session).await });
    }

    while let Some(task_result) = tasks.join_next().await {
        task_result
            .context("delete task panicked")?
            .context("parallel delete failed")?;
        deleted += 1;
    }

    println!(
        "Deleted listed batch of {} in {:.2?}",
        batch_size,
        started_at.elapsed()
    );

    Ok(deleted)
}

async fn delete_session(client: &reqwest::Client, session: &Session) -> Result<()> {
    let response = client
        .delete(format!("{API_BASE_URL}/{}", session.name))
        .send()
        .await
        .with_context(|| format!("failed to send delete request for {}", session.id))?;

    match response.error_for_status_ref() {
        Ok(_) => Ok(()),
        Err(error) if error.status() == Some(reqwest::StatusCode::NOT_FOUND) => Ok(()),
        Err(error) => Err(error)
            .with_context(|| format!("Jules API rejected delete request for {}", session.id)),
    }
}
