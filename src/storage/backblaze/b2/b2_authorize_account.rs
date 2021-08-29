use crate::errors::HostingError;
use crate::storage::backblaze::b2_types::AuthorizeAccountData;

/// <https://www.backblaze.com/b2/docs/b2_authorize_account.html>
pub(crate) async fn b2_authorize_account(
    api_url: &str,
    application_key_id: &str,
    application_key: &str,
) -> Result<AuthorizeAccountData, HostingError> {
    let url = format!("{}/b2api/v2/b2_authorize_account", api_url);
    debug!("B2_URL: {:#?}", url);

    let request = reqwest::Client::new()
        .get(&url)
        .basic_auth(
            application_key_id.to_string(),
            Some(application_key.to_string())
        );

    debug!("Request authorize account: {:#?}", request);

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
        Err(HostingError::BackblazeError(response.text().await?))
    }
}
