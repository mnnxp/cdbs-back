use crate::errors::HostingError;
use crate::storage::backblaze::b2_types::{
    // ResponseForUploadFile,
    FileHeaders,
};

// /// <https://www.backblaze.com/b2/docs/b2_download_file_by_name.html>
// pub async fn b2_download_file_by_name(
//     api_url: &str,
//     authorization_token: &str,
//     file_id: &str,
// ) -> Result<ResponseForUploadFile, HostingError> {
//     let url = format!(
//         "{}/b2api/v2/b2_download_file_by_name?fileName={}",
//         api_url,
//         file_id
//     );
//     debug!("B2_URL: {:#?}", url);
//
//     let request = reqwest::Client::new()
//         .get(&url)
//         .header(
//             reqwest::header::AUTHORIZATION,
//             authorization_token.to_string()
//         );
//
//     // let response = request.send().await?;
//     let mut response = request.send().await?;
//
//     if response.status().is_success() {
//         debug!("Response OK: {:#?}", response);
//         while let Some(chunk) = response.chunk().await? {
//             debug!("Chunk: {:#?}", chunk);
//         }
//         Ok(response.json().await?)
//     } else {
//         debug!("Response BAD: {:#?}", response);
//         while let Some(chunk) = response.chunk().await? {
//             debug!("Chunk: {:#?}", chunk);
//         }
//         Err(HostingError::BackblazeError(response.text().await?))
//     }
// }

/// Gets only the headers information of file
/// <https://www.backblaze.com/b2/docs/b2_download_file_by_id.html>
pub async fn b2_headers_file_by_id(
    api_url: &str,
    authorization_token: &str,
    file_id: &str,
) -> Result<FileHeaders, HostingError> {
    let url = format!(
        "{}/b2api/v2/b2_download_file_by_id?fileId={}",
        api_url,
        file_id
    );
    debug!("B2_URL: {:#?}", url);

    let request = reqwest::Client::new()
        .head(&url)
        .header(
            reqwest::header::AUTHORIZATION,
            authorization_token.to_string()
        );

    let response = request.send().await?;

    if response.status().is_success() {
        debug!("Response OK: {:#?}", response);
        Ok(FileHeaders::from(&response))
    } else {
        debug!("Response BAD: {:#?}", response);
        Err(HostingError::BackblazeError(response.text().await?))
    }
}












//
