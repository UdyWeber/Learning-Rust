use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GitHubGeneralStatus {
    description: String,
    indicator: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GitHubComponent {
    name: String,
    status: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GitHubStatusResponse {
    status: GitHubGeneralStatus,
    components: Vec<GitHubComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestPostRequest {
    detail: String,
    all_good: bool,
    components: Vec<GitHubComponent>,
}



pub async fn make_simple_request() {
    let client = Client::new();

    let response = client
        .get("https://www.githubstatus.com/api/v2/summary.json")
        .send()
        .await
        .unwrap()
        .json::<GitHubStatusResponse>()
        .await
        .unwrap();

    for component in response.components.iter() {
        println!(
            "Component name: {}, status: {}",
            component.name, component.status
        );
    }

    let test_body = TestPostRequest {
        components: response.components,
        detail: "Those are the Github components statuses".into(),
        all_good: true,
    };

    client
        .post("https://webhook.site/db61c02f-e7e1-44ff-aa49-98d98246ba43")
        .json(&test_body)
        .send()
        .await
        .unwrap();
} 