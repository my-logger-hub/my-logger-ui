use dioxus::prelude::*;

#[get("/api/insights?env")]
pub async fn get(env: String) -> Result<Vec<String>, ServerFnError> {
    let client = crate::server::APP_CTX.get_client(env.as_str()).await;
    let response = client.get_insights_keys(()).await;

    match response {
        Ok(response) => Ok(response.keys),
        Err(_) => Ok(vec![]),
    }
}
