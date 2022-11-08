extern crate reqwest;
extern crate serde_json;

pub struct GitHubApi {
	pub owner: String,
	pub repo: String
}

impl GitHubApi {
	// https://api.github.com/repos/pimnik98/SayoriOS/releases
	async fn _method(&self, method: String) -> reqwest::Result<String> {
		/*
		let request = reqwest::blocking::get(
			"https://api.github.com/repos/".to_string()+
            &self.owner+"/"+
            &self.repo+"/"+
            &method
		);
		*/
		let request = reqwest::Client::builder()
		.user_agent("Mozilla/5.0")
		.build()?
		.get(
			"https://api.github.com/repos/".to_string()+
            &self.owner+"/"+
            &self.repo+"/"+
            &method
		).send().await;

        match request {
            Err(err) => Err(err),
            Ok(data) => {
                match data.text().await {
                    Err(e) => Err(e),
                    Ok(s) => Ok(s)
                }
            }
        }
	}

	pub async fn method(&self, method: String) -> serde_json::Result<serde_json::Value> {
		let data = self._method(method).await;

		match data {
			Ok(d) => {
				serde_json::from_str(&d)
			},
			Err(..) => {
				serde_json::from_str("")
			}
		}
	}
}
