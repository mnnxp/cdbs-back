use crate::errors::HostingError;
use crate::storage::backblaze::b2_types::UploadUrlData;

/// <https://www.backblaze.com/b2/docs/b2_get_upload_url.html>
pub(crate) async fn b2_get_upload_url(
    api_url: &str,
    authorization_token: &str,
    bucket_id: &str,
) -> Result<UploadUrlData, HostingError> {

    let url = format!("{}/b2api/v2/b2_get_upload_url", api_url);

    let response = reqwest::Client::new()
        .post(&url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json"
        )
        .header(
            reqwest::header::AUTHORIZATION,
            authorization_token,
        )
        .body(
            serde_json::json!({
                "bucketId": bucket_id
            }).to_string()
        )
        .send()
        .await?;

    if response.status().is_success() {
        debug!("Response OK: {:#?}", response);
        Ok(response.json().await?)
    } else {
        debug!("Response BAD: {:#?}", response);
        Err(HostingError::BackblazeError(response.text().await?))
    }
}
