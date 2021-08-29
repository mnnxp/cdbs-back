use crate::errors::HostingError;
use crate::storage::backblaze::b2_types::{
    CreateKeyRequest,
    CreatedKeyData
};

/// <https://www.backblaze.com/b2/docs/b2_create_key.html>
pub(crate) async fn b2_create_key(
    api_url: &str,
    authorization_token: &str,
    create_key_req: &CreateKeyRequest,
) -> Result<CreatedKeyData, HostingError> {
    let url = format!("{}/b2api/v2/b2_create_key", api_url);
    debug!("B2_URL: {:#?}", url);

    let request = reqwest::Client::new()
        .post(&url)
        .header(
            reqwest::header::AUTHORIZATION,
            authorization_token.to_string()
        )
        .body(
            serde_json::json!(create_key_req).to_string()
        );

    let response = request.send().await?;
    // let mut response = request.send().await?;

    if response.status().is_success() {
        debug!("Response OK: {:#?}", response);
        // while let Some(chunk) = response.chunk().await? {
        //     debug!("Chunk: {:#?}", chunk);
        // }
        Ok(response.json().await?)
    } else {
        debug!("Response BAD: {:#?}", response);
        // while let Some(chunk) = response.chunk().await? {
        //     debug!("Chunk: {:#?}", chunk);
        // }
        Err(HostingError::BackblazeError(response.text().await?))
    }
}
