use std::error::Error;
use std::io::Read;
use reqwest::blocking::{Client, Response};
use reqwest::header::{CONTENT_LENGTH, RANGE};

fn read_body_soft (resp: Response) -> Result<Vec<u8>, reqwest::Error> {
    let mut resp = resp.error_for_status()?;

    let capacity = resp.headers()
        .get(CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let mut body_bytes: Vec<u8> = Vec::with_capacity(capacity);
    let mut buffer = [0u8; 8192]; // Буфер для чтения

    loop {
        match resp.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                body_bytes.extend_from_slice(&buffer[..n]);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {
                continue;
            }
            Err(_io_error) => {
                break;
            }
        }
    }
    Ok(body_bytes)
}


pub fn run(host: &str, port: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("http://{}:{}/", host, port);

    let initial_resp = client.get(&url).send()?;

    if !initial_resp.status().is_success() {
        return Err(Box::new(initial_resp.error_for_status().unwrap_err()));
    }

    let content_length = initial_resp.headers()
        .get(CONTENT_LENGTH)
        .ok_or("Отсутствует заголовок Content-Length")?
        .to_str()?
        .parse::<usize>()?;

    let mut data = read_body_soft(initial_resp)?;

    while data.len() < content_length {
        let current_len = data.len();
        let range_header = format!("bytes={}-{}", current_len, content_length);
        let range_resp = client.get(&url)
            .header(RANGE, range_header)
            .send()?;

        let chunk_bytes = read_body_soft(range_resp)?;

        if chunk_bytes.is_empty() && data.len() < content_length {
            break;
        }

        data.extend_from_slice(&chunk_bytes);
    }
    if data.len() != content_length {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            format!("Expected {} bytes, but got {}", content_length, data.len()),
        )));
    }
    Ok(data)
}

