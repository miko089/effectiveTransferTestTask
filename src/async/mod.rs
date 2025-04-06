use futures::future::join_all;
use reqwest::{header, Client};
use std::error::Error;
use tokio::time::{sleep, Duration};

const CHUNK_SIZE: usize = 64 * 1024;
const MAX_RETRIES: u32 = 5;
const RETRY_DELAY: Duration = Duration::from_millis(20);

pub async fn run(host: &str, port: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let url = format!("http://{}:{}/", host, port);
    let content_length: usize = client
        .get(&url)
        .send()
        .await?
        .headers()
        .get(header::CONTENT_LENGTH)
        .ok_or("Missing content-length header")?
        .to_str()?
        .parse::<usize>()
        .map_err(|_| "Invalid content-length header")?;
    let tasks_count = (content_length + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let mut tasks = Vec::with_capacity(tasks_count);
    let mut data: Vec<u8> = Vec::with_capacity(content_length);
    for i in 0..tasks_count {
        let start = i * CHUNK_SIZE;
        let end = std::cmp::min(start + CHUNK_SIZE, content_length);
        let client_clone = client.clone();
        let url_clone = url.clone();
        let task = tokio::spawn(async move {
            let range_header = format!("bytes={}-{}", start, end);
            let mut last_error: Option<Box<dyn Error + Send + Sync>> = None;

            for attempt in 0..=MAX_RETRIES {
                if attempt > 0 {
                    sleep(RETRY_DELAY).await;
                }

                let client_attempt = client_clone.clone();
                let url_attempt = url_clone.clone();

                match client_attempt
                    .get(&url_attempt)
                    .header(header::RANGE, range_header.clone())
                    .send()
                    .await
                {
                    Ok(resp) => match resp.bytes().await {
                        Ok(bytes) => {
                            return Ok(bytes);
                        }
                        Err(e) => {
                            let err_msg =
                                format!("Error reading bytes for {}: {}", range_header, e);
                            last_error = Some(Box::new(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                err_msg,
                            )));
                            break;
                        }
                    },
                    Err(e) => {
                        last_error = Some(Box::new(e));
                    }
                }
            }

            let final_err_msg = format!(
                "Failed to fetch range {} after {} retries. Last error: {:?}",
                range_header,
                MAX_RETRIES,
                last_error
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "Unknown error".to_string())
            );
            Err(final_err_msg.into())
        });
        tasks.push(task);
    }
    let results = join_all(tasks).await;
    let mut i = -1;
    for result in results {
        i += 1;
        match result {
            Ok(task_output) => match task_output {
                Ok(bytes) => {
                    data.extend_from_slice(&bytes);
                }
                Err(e) => {
                    eprintln!("Error in task1 {}: {}", i, e);
                    return Err(e);
                }
            },
            Err(e) => {
                eprintln!("Error in task2 {}: {}", i, e);
                return Err(Box::new(e));
            }
        }
    }
    Ok(data)
}
