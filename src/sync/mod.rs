use std::error::Error;
use std::io::Read;
use ureq;
use ureq::Body;
use ureq::http::Response;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";

fn read_to(resp: Response<Body>, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    if let Err(err) = resp.into_body().into_reader().read_to_end(buf) {
        if err.kind() != std::io::ErrorKind::UnexpectedEof {
            return Err(Box::new(err));
        }
    }
    Ok(())
}

pub fn run() -> Result<Vec<u8>, Box<dyn Error>>{
    let mut data: Vec<u8> = Vec::new();

    let mut resp = ureq::get(&format!("http://{}:{}/", HOST, PORT))
        .call();
    if let Err(e) = resp {
        return Err(Box::new(e));
    }
    let mut resp = resp.unwrap();

    let content_length = resp.headers()["content-length"]
        .to_str().unwrap()
        .parse::<usize>()
        .ok();

    match content_length {
        Some(content_length) => {
            data.reserve(content_length);
        },
        None => {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                                    "Invalid content-length")));
        }
    }
    let content_length = content_length.unwrap();
    read_to(resp, &mut data)?;

    while data.len() < content_length {
        let mut buf = Vec::new();
        let mut resp =
            ureq::get(&format!("http://{}:{}/", HOST, PORT))
                .header("Range", format!("bytes={}-{}", data.len(), content_length))
                .call();
        if let Err(e) = resp {
            return Err(Box::new(e));
        }
        let resp = resp.unwrap();
        read_to(resp, &mut buf)?;

        data.extend(buf);
    }
    Ok(data)
}
