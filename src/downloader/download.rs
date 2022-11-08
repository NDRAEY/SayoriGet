use reqwest;
use std::fs::File;
use std::cmp::min;
use std::io::Write;
use futures_util::StreamExt;
use std::io;

pub async fn progress(url: String, path: String) {
    let res = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build().unwrap()
        .get(url)
        .send()
        .await
        .or(Err("Failed to make GET request!"))
        .unwrap();
    let total_size = res.content_length();

    let mut file =
        File::create(path.clone())
        .or(Err(format!("Failed to create file '{}'", path.clone())))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).unwrap();
        let result = file.write_all(&chunk)
                     .or(Err(format!("Error while writing to file")));
        let new = min(downloaded + (chunk.len() as u64), total_size.unwrap());
        downloaded = new;

        let ntsize: f64 = total_size.unwrap() as f64;
		print!("[{:.1}%]   {:.0} of {:.0} kB\r",
				 (new as f64/ntsize)*100_f64,
				 new as f64/1024_f64,
				 ntsize/1024_f64);
        io::stdout().flush().unwrap();
    }
    println!();
}
