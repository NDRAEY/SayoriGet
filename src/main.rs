mod gh_api;
use crate::gh_api::gh_api::GitHubApi;
use std::io;
use std::io::Write;
use std::process;

fn log(message: String) {
    println!("[LOG] {}", message);
}

fn warn(message: String) {
    println!("[\x1b[33;1mWARN\x1b[0m] {}", message);
}

fn error(message: String) {
    println!("[\x1b[31;1mERR\x1b[0m] {}", message);
}

fn main() {
    println!("SayoriGet v1.0 by NDRAEY 2022");

    let sayori_original = GitHubApi {
        owner: "pimnik98".to_string(),
        repo: "SayoriOS".to_string(),
    };

    let mut version: Option<String> = None;

    let data = sayori_original.method("releases".to_string());
    match data {
        Ok(values) => {
            println!("\n{}", "=".repeat(40));

            let arraylen = values.as_array().unwrap().len();
            let mut versions: Vec<&str> = Vec::new();

            for i in 0..arraylen {
                let curobj = values[i].as_object().unwrap();

                let tag_name = &curobj["tag_name"].as_str().unwrap();
                let name = &curobj["name"].as_str().unwrap();

                println!("[{}] {} ({})", i + 1, name, tag_name);
                versions.push(tag_name);
            }

            let mut selected = arraylen + 1;

            if version == None {
                // For future
                println!("");
                while selected > arraylen {
                    let mut tempstr = String::new();
                    print!("Select entry > ");
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut tempstr)
                        .expect("ERR: Unable to read user input");

                    tempstr = tempstr.trim().to_string();
                    version = Some(tempstr);
                    selected = version.unwrap().parse::<usize>().unwrap() - 1;
                }
            }

            log(format!("Selected version is: {}", versions[selected]));

            let mut url: Option<&str> = None;

            let curobj = values[selected]["assets"].as_array().unwrap();
            //let mut found = false;

            for j in 0..curobj.len() {
                if curobj[j]["content_type"] == "application/x-cd-image" {
                    url = Some(curobj[j]["browser_download_url"].as_str().unwrap());
                    //found = true;
                    break;
                }
            }

            if url == None {
                error(format!(
                    "No ISO images found for version: {}",
                    versions[selected]
                ));
                process::exit(1);
            }

            println!("This url: {}", url.unwrap());
        }
        Err(err) => {
            error(format!("Failed to parse JSON! ({})", err));
        }
    }
}
