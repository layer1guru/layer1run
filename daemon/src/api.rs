/* Initialize constants. */
const L1GURU_ENDPOINT: &str = "https://layer1.guru/v1/";

/**
 * Call
 *
 * Make a (remote) API call.
 */
#[tokio::main]
pub async fn call(_endpoint: &str, _json: &str) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url: String = format!("{}{}", L1GURU_ENDPOINT, _endpoint);

    // let headers = [("Authorization", "Bearer YOUR_API_KEY"), ("X-Custom-Header", "value")];

    /* Initialize client */
    let client: reqwest::Client = reqwest::Client::new();
    
    /* Make remote data request. */
    let response: reqwest::Response = client
        .post(url)
        .header("content-type", "application/json")
        // .headers(headers.into_iter().collect())
        .body(_json.to_string())
        .send()
        .await?;

    // TODO Validate status (200 OK)        
// println!("Status: {}", response.status());
    
    // let response_body = response.json().await?;
    let response_body: String = response.text().await?;
// println!("Response body:\n{}", response_body);

    /* Return response. */
    Ok(response_body)
}