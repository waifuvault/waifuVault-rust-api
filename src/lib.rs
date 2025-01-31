//! This is the Rust version of the [Waifu Vault SDK](https://waifuvault.moe/) which is used to
//! interact with the file upload service.
//!
//! For Terms of Service and usage policy, please refer to the above website.
//!
//! # Uploading a file
//!
//! ```rust,no_run
//! use waifuvault::{
//!     ApiCaller,
//!     api::{WaifuUploadRequest}
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     // Upload a file from disk
//!     let request = WaifuUploadRequest::new()
//!         .file("/some/file/path") // Path to a file
//!         .password("set a password") // Set a password
//!         .one_time_download(true); // Delete after first access
//!     let response = caller.upload_file(request).await?;
//!
//!     // Upload a file from a URL
//!     let request = WaifuUploadRequest::new()
//!         .url("https://some-website/image.jpg"); // URL to content
//!     let response = caller.upload_file(request).await?;
//!
//!     // Upload a file from raw bytes
//!     let data = std::fs::read("some/file/path")?;
//!     let request = WaifuUploadRequest::new()
//!         .bytes(data, "name-to-store.rs"); // Raw file content and name to store on the vault
//!     let response = caller.upload_file(request).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Get File Information
//!
//! ```rust,no_run
//! use waifuvault::{
//!     ApiCaller,
//!     api::WaifuGetRequest
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let request = WaifuGetRequest::new("some-waifu-vault-token");
//!     let response = caller.file_info(request).await?;
//!
//!     // Do something with the response
//!
//!     Ok(())
//! }
//! ```
//!
//! # Modify Existing File Properties
//!
//! ```rust,no_run
//! use waifuvault::{
//!     ApiCaller,
//!     api::WaifuModificationRequest
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let request = WaifuModificationRequest::new("some-waifu-vault-token")
//!         .password("new_password") // Set a new password
//!         .previous_password("old_password") // Old password
//!         .custom_expiry("1h") // Set a new expiry
//!         .hide_filename(true); // Hide the filename
//!
//!     let response = caller.update_file(request).await?;
//!
//!     // Do something with the response
//!
//!     Ok(())
//! }
//! ```
//!
//! # Delete a file from Waifu Vault
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!     let response = caller.delete_file("some-waifu-token").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Download a file
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//! use std::io::Write;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     // Download a file with no password
//!     let content = caller.download_file("https://waifuvault.moe/f/some-file.ext", None).await?;
//!     let mut f = std::fs::File::create("downloaded_file.txt")?;
//!     f.write_all(&content)?;
//!
//!     // Download a file with no password
//!     let content = caller.download_file("https://waifuvault.moe/f/some-other-file.ext", Some("password".to_string())).await?;
//!     let mut f = std::fs::File::create("downloaded_file2.txt")?;
//!     f.write_all(&content)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Create a Bucket
//!
//! ```rust,no_run
//! use waifuvault::{ApiCaller, api::WaifuUploadRequest};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     // Create a new bucket to upload files to
//!     let bucket = caller.create_bucket().await?;
//!
//!     // You can now use the bucket token to upload files to the bucket
//!
//!     let request = WaifuUploadRequest::new()
//!         .file("/some/file/path")
//!         .bucket(&bucket.token)
//!         .password("set a password")
//!         .one_time_download(true);
//!     let response = caller.upload_file(request).await?;
//!
//!     // Do something with the response
//!
//!     Ok(())
//! }
//! ```
//!
//! # Delete a Bucket
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let token = "some-bucket-token";
//!
//!     // Delete the bucket and all files within
//!     caller.delete_bucket(token).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Get Bucket information
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let token = "some-bucket-token";
//!
//!     // Get bucket information
//!     let info = caller.get_bucket(token).await?;
//!
//!     // You can now get access to the file information for files inside the bucket
//!     for file in info.files.iter() {
//!         // Do something with the file information
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Create an Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let token = "some-bucket-token";
//!
//!     // Create a new album
//!     let album_info = caller.create_album(token, "new_album").await?;
//!
//!     // You now have access to the album token to perform actions on the album
//!
//!     Ok(())
//! }
//! ```
//!
//! # Associate Files With An Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!     let file_1_tkn = "file_1_tkn";
//!     let file_2_tkn = "file_2_tkn";
//!
//!     // Associate both files with the album
//!     let album_info = caller.associate_with_album(album_tkn, &[file_1_tkn, file_2_tkn]).await?;
//!
//!     // Both files should now be part of the album
//!
//!     Ok(())
//! }
//! ```
//!
//! # Disassociate Files From An Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!     let file_1_tkn = "file_1_tkn";
//!     let file_2_tkn = "file_2_tkn";
//!
//!     // Associate both files with the album
//!     let album_info = caller.disassociate_from_album(album_tkn, &[file_1_tkn, file_2_tkn]).await?;
//!
//!     // Both files should now be removed from the album
//!
//!     Ok(())
//! }
//! ```
//!
//! # Delete An Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!
//!     // Delete an album but keep the files in the bucket
//!     let status = caller.delete_album(album_tkn, false).await?;
//!     
//!     // We can also delete the album and any files from the bucket as well
//!     let status = caller.delete_album(album_tkn, true).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Get an Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!
//!     // Get information about the contents of an album
//!     let album_info = caller.get_album(album_tkn).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Share an Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!
//!     // Obtain a public URL to the album
//!     let status = caller.share_album(album_tkn).await?;
//!
//!     // The description contains the public URL you can use to access the album
//!     // on the web
//!     let public_url = status.description;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Revoke Access to a Public Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!
//!     // Revoke access to the public album
//!     // This will invalidate the Public URL to the album making it inaccessible
//!     let status = caller.revoke_album(album_tkn).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Download a Zip Archive of an Album
//!
//! ```rust,no_run
//! use waifuvault::ApiCaller;
//! use std::io::Write;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let caller = ApiCaller::new();
//!
//!     let album_tkn = "album-tkn";
//!
//!     // If the `file_ids` passed is `None`, it will download the entire album
//!     let contents = caller.download_album(album_tkn, None).await?;
//!
//!     // If you know the File IDs you want to download, you can specify them
//!     // This will only download those files from the album
//!     let contents = caller.download_album(album_tkn, Some(&[0, 1, 2])).await?;
//!
//!     // You can then unzip them in code or save them to disk like so
//!     let mut f = std::fs::File::create("archive.zip")?;
//!     f.write_all(&contents)?;
//!
//!     Ok(())
//! }
//! ```

pub mod api;

use std::{collections::HashMap, path::PathBuf};

use api::*;

use anyhow::Context;
use reqwest::Client;

/// REST endpoint for the service
#[cfg(not(test))]
const API: &str = "https://waifuvault.moe/rest";

#[cfg(test)]
const API: &str = "http://127.0.0.1:8081/rest";

/// Api controller which calls the endpoint
#[derive(Debug, Clone, Default)]
pub struct ApiCaller {
    client: Client,
}

impl ApiCaller {
    /// Create a new Waifu Vault API Caller
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a bucket with the Waifu Vault API
    ///
    /// This bucket can be used to upload files into
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::{ApiCaller, api::WaifuUploadRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     // Create a new bucket to upload files to
    ///     let bucket = caller.create_bucket().await?;
    ///
    ///     // You can now use the bucket token to upload files to the bucket
    ///
    ///     let request = WaifuUploadRequest::new()
    ///         .file("/some/file/path")
    ///         .bucket(&bucket.token)
    ///         .password("set a password")
    ///         .one_time_download(true);
    ///     let response = caller.upload_file(request).await?;
    ///
    ///     // Do something with the response
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_bucket(&self) -> anyhow::Result<WaifuBucketEntry> {
        let url = format!("{API}/bucket/create");

        let response: WaifuApiResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("calling create bucket endpoint")?
            .json()
            .await
            .context("converting create bucket api response")?;

        match response {
            WaifuApiResponse::WaifuBucketResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response: {response:?}"),
        }
    }

    /// Deletes a Bucket with the Waifu Vault API
    ///
    /// This will remove ALL files contained within the bucket
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let token = "some-bucket-token";
    ///
    ///     // Delete the bucket and all files within
    ///     caller.delete_bucket(token).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_bucket(&self, token: &str) -> anyhow::Result<bool> {
        let url = format!("{API}/bucket/{}", token);
        let response: WaifuApiResponse = self
            .client
            .delete(&url)
            .send()
            .await
            .context("sending delete bucket request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::Delete(success) => Ok(success),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("Received unexpected response from DELETE bucket endpoint"),
        }
    }

    /// Gets information on files contained within a Bucket with the Waifu Vault API
    ///
    /// This returns a [`api::WaifuBucketEntry`] which contains an array of all files
    /// contained within the bucket as well as the bucket token.
    ///
    /// # Example
    ///
    ///
    pub async fn get_bucket(&self, token: &str) -> anyhow::Result<WaifuBucketEntry> {
        let url = format!("{API}/bucket/get");
        let mut body = HashMap::new();
        body.insert("bucket_token", token);

        let response: WaifuApiResponse = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("sending get bucket request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuBucketResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from get bucket endpoint: {response:?}"),
        }
    }

    /// Upload a file to Waifu Vault
    ///
    /// Takes an [`api::WaifuUploadRequest`] which details the content to upload and any
    /// necessary options.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::{
    ///     ApiCaller,
    ///     api::WaifuUploadRequest
    /// };
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///     let request = WaifuUploadRequest::new()
    ///         .file("/some/file/to/upload")
    ///         .password("supersecurepassword")
    ///         .expires("1h");
    ///
    ///     let response = caller.upload_file(request).await?;
    ///     // Do something with response
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_file(&self, request: WaifuUploadRequest) -> anyhow::Result<WaifuFileEntry> {
        let url = if let Some(bucket) = request.bucket {
            &format!("{API}/{bucket}")
        } else {
            API
        };

        let request = {
            let mut intermediate = self.client.put(url).query(&[
                ("hide_filename", request.hide_filename),
                ("oneTimeDownload", request.one_time_download),
            ]);

            if let Some(expiry) = request.expires {
                intermediate = intermediate.query(&[("expires", expiry)]);
            }

            if let Some(file) = request.file {
                let path = PathBuf::from(&file);
                let f = std::fs::read(&path)
                    .with_context(|| format!("reading file {}", path.display()))?;

                let filename = path.file_name().expect("this should be a valid filename");
                let filename = filename
                    .to_str()
                    .expect("this should be a valid convertion from os string");

                let file_part = reqwest::multipart::Part::bytes(f).file_name(filename.to_owned());
                let mut form = reqwest::multipart::Form::new().part("file", file_part);

                if let Some(password) = request.password {
                    form = form.text("password", password);
                }

                intermediate = intermediate.multipart(form);
            } else if let Some(url) = request.url {
                intermediate = match request.password {
                    Some(password) => intermediate.form(&[("url", url), ("password", password)]),
                    None => intermediate.form(&[("url", url)]),
                };
            } else if let (Some(raw), Some(filename)) = (request.bytes, request.filename) {
                let file_part = reqwest::multipart::Part::bytes(raw).file_name(filename);
                let mut form = reqwest::multipart::Form::new().part("file", file_part);

                if let Some(password) = request.password {
                    form = form.text("password", password);
                }

                intermediate = intermediate.multipart(form);
            } else {
                anyhow::bail!("need either a file, url, or stream");
            }

            intermediate
        };

        let response = request
            .send()
            .await
            .context("sending upload request")?
            .json()
            .await
            .context("converting upload response")?;

        let response = parse_response(response).context("parsing waifu api response")?;

        Ok(response)
    }

    /// Retrieves information about a file stored in Waifu Vault
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::{
    ///     ApiCaller,
    ///     api::WaifuGetRequest
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let token = "some-file-token-for-waifu-vault";
    ///     let caller = ApiCaller::new();
    ///     let request = WaifuGetRequest::new(token)
    ///         .formatted(true);
    ///
    ///     let response = caller.file_info(request).await?;
    ///     // Do something with response
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn file_info(&self, request: WaifuGetRequest) -> anyhow::Result<WaifuFileEntry> {
        let url = format!("{API}/{}", request.token);
        let request = self
            .client
            .get(url)
            .query(&[("formatted", request.formatted)]);

        let response: WaifuApiResponse = request
            .send()
            .await
            .context("sending file info request")?
            .json()
            .await
            .context("converting response")?;

        let response = parse_response(response).context("parsing waifu api response")?;

        Ok(response)
    }

    /// Updates options on a stored file in Waifu Vault
    ///
    /// Allows the changing of the password, expiry time, and whether to hide the filename or not
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::{
    ///     ApiCaller,
    ///     api::WaifuModificationRequest
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let token = "some-token";
    ///     let caller = ApiCaller::new();
    ///     let request = WaifuModificationRequest::new(token)
    ///         .password("banana")
    ///         .previous_password("apple")
    ///         .hide_filename(true);
    ///     let response = caller.update_file(request).await?;
    ///     // Do something with the response
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_file(
        &self,
        request: WaifuModificationRequest,
    ) -> anyhow::Result<WaifuFileEntry> {
        let url = format!("{API}/{}", request.token);
        let response: WaifuApiResponse = self
            .client
            .patch(url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("sending modification request")?
            .json()
            .await
            .context("converting response")?;

        let response = parse_response(response).context("parsing waifu api response")?;
        Ok(response)
    }

    /// Deletes a file from Waifu Vault
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let token = "token-to-delete";
    ///     let caller = ApiCaller::new();
    ///     let response = caller.delete_file(token).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_file(&self, token: &str) -> anyhow::Result<bool> {
        let url = format!("{API}/{}", token);
        let response: WaifuApiResponse = self
            .client
            .delete(url)
            .send()
            .await
            .context("sending delete request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::Delete(del) => Ok(del),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("received unexpected response from DELETE call"),
        }
    }

    /// Downloads a file from Waifu Vault
    ///
    /// Returns the contents of the file as an array of bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    /// use std::io::Write;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let url = "https://waifuvault.moe/f/[some-id]/file.jpg";
    ///     let caller = ApiCaller::new();
    ///     let file_bytes = caller.download_file(url, Some("securepassword".to_string())).await?;
    ///     let mut f = std::fs::File::create("downloaded.jpg")?;
    ///     f.write_all(&file_bytes)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_file(
        &self,
        url: &str,
        password: Option<String>,
    ) -> anyhow::Result<Vec<u8>> {
        let request = {
            let mut r = self.client.get(url);
            if let Some(password) = &password {
                r = r.header("x-password", password);
            }

            r
        };

        let response = request.send().await.context("sending download request")?;
        let status = response.status();

        match status {
            reqwest::StatusCode::OK => {}
            reqwest::StatusCode::FORBIDDEN => {
                if password.is_some() {
                    anyhow::bail!("supplied password is incorrect");
                } else {
                    anyhow::bail!("this file requires a password to download");
                }
            }
            _ => {
                let api_response: WaifuApiResponse =
                    response.json().await.context("converting error")?;
                match api_response {
                    WaifuApiResponse::WaifuError(err) => return Err(err.into()),
                    _ => anyhow::bail!("something went wrong"),
                }
            }
        }

        let content = response
            .bytes()
            .await
            .context("getting content bytes")?
            .to_vec();

        Ok(content)
    }

    /// Creates an album on the WaifuVault service
    ///
    /// This requires the token from a previously created bucket
    /// Returns information relating to the newly created album
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let bucket = caller.create_bucket().await?;
    ///     let album_info = caller.create_album(&bucket.token, "new_album").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_album(
        &self,
        bucket_token: &str,
        album_name: &str,
    ) -> anyhow::Result<WaifuAlbumEntry> {
        let url = format!("{API}/album/{}", bucket_token);
        let mut body = HashMap::new();
        body.insert("name", album_name);
        let response: WaifuApiResponse = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("sending create album request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuAlbumResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from create album endpoint: {response:?}"),
        }
    }

    /// Associates a collection of Files with an Album
    ///
    /// This requires an array of File tokens already present on Waifu Vault
    /// and a previously created album.
    /// Returns information relating to the updated album
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let file_token1 = "file-token-1";
    ///     let file_token2 = "file-token-2";
    ///     let album_token = "album-token";
    ///     let album_info = caller.associate_with_album(album_token, &[file_token1, file_token2]).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn associate_with_album(
        &self,
        album_token: &str,
        file_tokens: &[&str],
    ) -> anyhow::Result<WaifuAlbumEntry> {
        let url = format!("{API}/album/{}/associate", album_token);
        let mut body = HashMap::new();
        body.insert("fileTokens", file_tokens);

        let response: WaifuApiResponse = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("sending album association request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuAlbumResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from album association endpoint: {response:?}"),
        }
    }

    /// Disassociate a collection of Files with an Album
    ///
    /// This requires an array of File tokens already present on Waifu Vault
    /// and a previously created album.
    /// Returns information relating to the updated album
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let file_token1 = "file-token-1";
    ///     let file_token2 = "file-token-2";
    ///     let album_token = "album-token";
    ///     let album_info = caller.disassociate_from_album(album_token, &[file_token1, file_token2]).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn disassociate_from_album(
        &self,
        album_token: &str,
        file_tokens: &[&str],
    ) -> anyhow::Result<WaifuAlbumEntry> {
        let url = format!("{API}/album/{}/disassociate", album_token);
        let mut body = HashMap::new();
        body.insert("fileTokens", file_tokens);

        let response: WaifuApiResponse = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("sending album association request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuAlbumResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from album association endpoint: {response:?}"),
        }
    }

    /// Delete an album from Waifu Vault
    ///
    /// If `deleteFiles` is true, will delete the files from the bucket as well.
    ///
    /// Returns the status of the operation indicating if it was successful or not.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let album_token = "album-token";
    ///
    ///     // Keep files associated with the bucket
    ///     // Pass `true` to delete files from the bucket as well
    ///     let status = caller.delete_album(album_token, false).await?;
    ///
    ///     assert!(status.success);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_album(
        &self,
        album_token: &str,
        delete_files: bool,
    ) -> anyhow::Result<WaifuGenericMessage> {
        let url = format!("{API}/album/{}", album_token);
        let response: WaifuApiResponse = self
            .client
            .delete(&url)
            .query(&[("deleteFiles", delete_files)])
            .send()
            .await
            .context("sending album delete request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuGenericResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from album deletion endpoint: {response:?}"),
        }
    }

    /// Get information about album from Waifu Vault
    ///
    /// Returns information relating to the album
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let album_token = "album-token";
    ///     let album_info = caller.get_album(album_token).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_album(&self, album_token: &str) -> anyhow::Result<WaifuAlbumEntry> {
        let url = format!("{API}/album/{album_token}");
        let response: WaifuApiResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("sending get album request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuAlbumResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from get album endpoint: {response:?}"),
        }
    }

    /// Share an album from Waifu Vault
    ///
    /// Returns a staus object indicating the success of the operation.
    /// It also contains the URL of the public album in the `description`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let album_token = "album-token";
    ///     let status = caller.share_album(album_token).await?;
    ///
    ///     assert!(status.success);
    ///     let shareable_url = status.description;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn share_album(&self, album_token: &str) -> anyhow::Result<WaifuGenericMessage> {
        let url = format!("{API}/album/share/{album_token}");
        let response: WaifuApiResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("sending share album request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuGenericResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from get album endpoint: {response:?}"),
        }
    }

    /// Revokes public access from an album on Waifu Vault
    ///
    /// Any public URLs to the album are invalidated.
    /// Returns a staus object indicating the success of the operation.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let album_token = "album-token";
    ///     let status = caller.revoke_album(album_token).await?;
    ///
    ///     assert!(status.success);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn revoke_album(&self, album_token: &str) -> anyhow::Result<WaifuGenericMessage> {
        let url = format!("{API}/album/revoke/{album_token}");
        let response: WaifuApiResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("sending share album request")?
            .json()
            .await
            .context("converting response")?;

        match response {
            WaifuApiResponse::WaifuGenericResponse(resp) => Ok(resp),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("unexpected response from get album endpoint: {response:?}"),
        }
    }

    /// Downloads a zip archive of an album on Waifu Vault
    ///
    /// If `file_ids` is passed, it returns only those files in the archive.
    /// If `None`, the entire contents of the album are returned
    ///
    /// Returns a `Vec<u8>` containing the zipped data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use waifuvault::ApiCaller;
    /// use std::io::Write;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let caller = ApiCaller::new();
    ///
    ///     let album_token = "album-token";
    ///     let zipped_contents = caller.download_album(album_token, None).await?;
    ///     let mut f = std::fs::File::create("archive.zip")?;
    ///     f.write_all(&zipped_contents)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_album(
        &self,
        album_token: &str,
        file_ids: Option<&[usize]>,
    ) -> anyhow::Result<Vec<u8>> {
        let url = format!("{API}/album/download/{album_token}");
        let body = match file_ids {
            Some(ids) => ids,
            None => &vec![],
        };
        let response = self
            .client
            .post(&url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("sending download part album request")?;

        let status = response.status();
        match status {
            reqwest::StatusCode::OK => {}
            _ => {
                let api_response: WaifuApiResponse =
                    response.json().await.context("converting error")?;
                match api_response {
                    WaifuApiResponse::WaifuError(err) => return Err(err.into()),
                    _ => anyhow::bail!(
                        "unexpected error responser received from api: {api_response:?}"
                    ),
                }
            }
        }

        let content = response
            .bytes()
            .await
            .context("obtaining response bytes")?
            .to_vec();

        Ok(content)
    }
}

/// Parses the response from the Waifu Vault API and converts it to
/// a concrete type
pub(crate) fn parse_response(response: WaifuApiResponse) -> anyhow::Result<WaifuFileEntry> {
    match response {
        WaifuApiResponse::WaifuFileResponse(resp) => Ok(resp),
        WaifuApiResponse::WaifuError(err) => Err(anyhow::anyhow!(err)),
        _ => unreachable!("unused"),
    }
}

#[cfg(test)]
mod tests {
    // When running in test environment, it uses a local API version so the WaifuVault
    // must be set up to run locally.
    use super::*;
    use anyhow::Result;
    use rand::RngCore;
    use sha1::{Digest, Sha1};
    use std::path::PathBuf;
    use tokio::{fs, io::AsyncWriteExt};

    // I know I could use `tempfile` here but scope issues made it awkward
    // at least with this i can control when to delete the temp file
    struct TempFileCreator {
        file: PathBuf,
    }

    impl TempFileCreator {
        pub async fn new(filename: &str) -> Result<Self> {
            let tmp = std::env::temp_dir();

            let mut data = Vec::with_capacity(16_384);
            rand::thread_rng().fill_bytes(&mut data);

            let test_file = tmp.join(filename);
            let mut f = tokio::fs::File::create(&test_file).await?;
            f.write_all(&mut data).await?;

            return Ok(Self { file: test_file });
        }
    }

    impl Drop for TempFileCreator {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.file);
        }
    }

    struct Dropper<'a> {
        pub album_tkn: Option<String>,
        pub bucket_tkn: Option<String>,
        pub file_tkn: Option<String>,
        pub caller: &'a ApiCaller,
    }

    impl<'a> Dropper<'a> {
        pub fn new(caller: &'a ApiCaller) -> Self {
            Self {
                caller,
                album_tkn: None,
                bucket_tkn: None,
                file_tkn: None,
            }
        }

        pub async fn create_bucket(&mut self) -> Result<WaifuBucketEntry> {
            match self.caller.create_bucket().await {
                Ok(r) => {
                    self.bucket_tkn = Some(r.token.clone());
                    Ok(r)
                }
                Err(e) => {
                    self.destroy().await?;
                    return Err(e);
                }
            }
        }

        pub async fn upload_file(
            &mut self,
            upload_req: WaifuUploadRequest,
        ) -> Result<WaifuFileEntry> {
            match self.caller.upload_file(upload_req).await {
                Ok(r) => {
                    self.file_tkn = Some(r.token.clone());
                    Ok(r)
                }
                Err(e) => {
                    self.destroy().await?;
                    return Err(e);
                }
            }
        }

        pub async fn create_album(&mut self, bkt_tkn: &str, name: &str) -> Result<WaifuAlbumEntry> {
            match self.caller.create_album(bkt_tkn, name).await {
                Ok(r) => {
                    self.album_tkn = Some(r.token.clone());
                    Ok(r)
                }
                Err(e) => {
                    self.destroy().await?;
                    return Err(e);
                }
            }
        }

        pub async fn destroy(&self) -> anyhow::Result<()> {
            if let Some(ref f_tkn) = self.file_tkn {
                self.caller.delete_file(&f_tkn).await?;
            }

            if let Some(ref b_tkn) = self.bucket_tkn {
                self.caller.delete_bucket(&b_tkn).await?;
            }

            if let Some(ref a_tkn) = self.album_tkn {
                self.caller.delete_album(&a_tkn, true).await?;
            }

            Ok(())
        }
    }

    async fn cleanup(caller: &ApiCaller, token: &str) -> Result<()> {
        caller.delete_file(token).await?;

        Ok(())
    }

    #[tokio::test]
    async fn upload_file() -> Result<()> {
        let tmp = TempFileCreator::new("upload_basic.bin").await?;
        assert!(tmp.file.exists());
        let caller = ApiCaller::new();
        let upload_request = api::WaifuUploadRequest::new().file(&tmp.file);

        let response = caller
            .upload_file(upload_request)
            .await
            .context("upload file - basic");

        assert!(response.is_ok());

        let response = response?;
        let options = response
            .options
            .expect("expected options when there are none");

        assert!(!options.hide_filename);
        assert!(!options.protected);
        assert!(!options.one_time_download);

        cleanup(&caller, &response.token).await?;
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn upload_file_with_options() -> Result<()> {
        let tmp = TempFileCreator::new("upload_with_options.bin").await?;
        let caller = ApiCaller::new();
        let upload_request = api::WaifuUploadRequest::new()
            .file(&tmp.file)
            .expires("1h")
            .password("apple")
            .one_time_download(true)
            .hide_filename(true);

        let response = caller
            .upload_file(upload_request)
            .await
            .context("upload file with options")?;
        let options = response
            .options
            .expect("expected options when there are none");

        assert!(options.hide_filename);
        assert!(options.protected);
        assert!(options.one_time_download);

        cleanup(&caller, &response.token).await?;
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn upload_file_from_url() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";
        let caller = ApiCaller::new();
        let request = WaifuUploadRequest::new().url(url).expires("1h");

        let response = caller
            .upload_file(request)
            .await
            .context("upload from url")?;
        let options = response
            .options
            .expect("expected options when there are none");

        assert!(!options.hide_filename);
        assert!(!options.protected);
        assert!(!options.one_time_download);

        cleanup(&caller, &response.token).await?;
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn upload_file_bytes() -> Result<()> {
        let tmp = TempFileCreator::new("upload_from_raw_bytes.bin").await?;
        let caller = ApiCaller::new();
        let content = fs::read(&tmp.file).await?;
        let request = WaifuUploadRequest::new()
            .bytes(content, "test_raw_bytes.bin")
            .expires("1h");

        let response = caller.upload_file(request).await?;
        cleanup(&caller, &response.token).await?;
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn get_file_info() -> Result<()> {
        let tmp = TempFileCreator::new("get_file_info_basic.bin").await?;
        let caller = ApiCaller::new();
        let upload = WaifuUploadRequest::new().file(&tmp.file);
        let response = caller.upload_file(upload).await?;

        let token = response.token;
        let options = response
            .options
            .expect("options expected but there are none");
        let get_req = WaifuGetRequest::new(&token);
        let response = caller.file_info(get_req).await?;

        assert_eq!(&token, &response.token);
        assert_eq!(
            &options,
            &response
                .options
                .expect("expected options but there are none")
        );
        cleanup(&caller, &response.token).await?;
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn invalid_token() -> Result<()> {
        let caller = ApiCaller::new();
        let request = WaifuGetRequest::new("hithere");
        let response = caller.file_info(request).await;
        assert!(response.is_err());
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn patch_entry() -> Result<()> {
        let tmp = TempFileCreator::new("some_entry_to_be_patched.bin").await?;
        let caller = ApiCaller::new();

        let init = WaifuUploadRequest::new().file(&tmp.file).expires("1h");
        let response = caller.upload_file(init).await?;
        let token = response.token;
        let original_exp = response.retention_period;
        let original_opts = response.options.unwrap();
        be_nice().await;

        // Add a password
        let mod_request = WaifuModificationRequest::new(&token).password("banana");
        let response = caller.update_file(mod_request).await?;
        let options = response.options.unwrap();
        assert!(options.protected);
        assert_ne!(options.protected, original_opts.protected);
        be_nice().await;

        // Add an expiry
        let mod_request = WaifuModificationRequest::new(&token).custom_expiry("5m");
        let response = caller.update_file(mod_request).await?;
        assert_ne!(response.retention_period, original_exp);
        be_nice().await;

        // Hide the filename
        let mod_request = WaifuModificationRequest::new(&token).hide_filename(true);
        let response = caller.update_file(mod_request).await?;
        let options = response.options.unwrap();
        assert!(options.hide_filename);
        assert_ne!(options.hide_filename, original_opts.hide_filename);
        be_nice().await;

        // Update password
        let mod_request = WaifuModificationRequest::new(&token)
            .password("apple")
            .previous_password("banana");
        let response = caller.update_file(mod_request).await?;
        let options = response.options.unwrap();
        assert!(options.protected);
        be_nice().await;

        cleanup(&caller, &token).await?;
        be_nice().await;
        Ok(())
    }

    #[tokio::test]
    async fn delete_file() -> Result<()> {
        let tmp = TempFileCreator::new("something_to_delete.bin").await?;
        let caller = ApiCaller::new();
        let request = WaifuUploadRequest::new().file(&tmp.file);
        let response = caller.upload_file(request).await?;
        let token = response.token;
        let success = caller.delete_file(&token).await?;

        assert!(success);
        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn download_file() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";
        let original = reqwest::get(url).await?.bytes().await?.to_vec();
        let og_hash = hash_item(&original);

        let caller = ApiCaller::new();
        let request = WaifuUploadRequest::new().url(url).expires("1h");
        let response = caller.upload_file(request).await?;
        let url = response.url;

        let response = caller.download_file(&url, None).await?;
        let result = hash_item(&response);

        assert_eq!(og_hash, result);
        be_nice().await;
        Ok(())
    }

    #[tokio::test]
    async fn download_file_with_password() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";
        let original = reqwest::get(url).await?.bytes().await?.to_vec();
        let og_hash = hash_item(&original);

        let caller = ApiCaller::new();
        let request = WaifuUploadRequest::new()
            .url(url)
            .expires("1h")
            .password("banana");
        let response = caller
            .upload_file(request)
            .await
            .context("uploading protected file to download")?;
        let url = response.url;

        let response = caller
            .download_file(&url, Some("banana".to_string()))
            .await?;
        let result = hash_item(&response);

        assert_eq!(og_hash, result);
        be_nice().await;
        Ok(())
    }

    #[tokio::test]
    async fn create_and_delete_bucket() -> Result<()> {
        let caller = ApiCaller::new();
        let response = caller.create_bucket().await?;
        assert!(!response.token.is_empty());

        let token = response.token;
        assert!(response.files.is_empty());

        let resp = caller.delete_bucket(&token).await?;
        assert!(resp);

        be_nice().await;
        Ok(())
    }

    #[tokio::test]
    async fn create_and_get_bucket() -> Result<()> {
        let caller = ApiCaller::new();
        let bucket = caller.create_bucket().await?;
        let token = bucket.token;

        let info = caller.get_bucket(&token).await?;
        assert!(info.token == token);
        assert!(info.files.is_empty());

        caller.delete_bucket(&token).await?;

        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn create_bucket_and_upload_file() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";
        let caller = ApiCaller::new();

        let bucket = caller.create_bucket().await?;
        let token = bucket.token;
        let request = WaifuUploadRequest::new()
            .bucket(&token)
            .url(url)
            .expires("1h");

        caller.upload_file(request).await?;

        let info = caller.get_bucket(&token).await?;
        assert!(info.files.len() == 1);

        caller.delete_bucket(&token).await?;

        be_nice().await;

        Ok(())
    }

    #[tokio::test]
    async fn delete_bucket_incorrect_token() -> Result<()> {
        let caller = ApiCaller::new();
        let response = caller.delete_bucket("garbage").await;
        assert!(response.is_err());

        let inner = response.unwrap_err();
        let waifu_err = inner.downcast::<WaifuError>()?;
        assert_eq!(waifu_err.status, 400);

        Ok(())
    }

    #[tokio::test]
    async fn create_album() -> Result<()> {
        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket_response = dropper.create_bucket().await?;
        assert!(!bucket_response.token.is_empty());

        let bkt_tkn = bucket_response.token;
        let album_response = dropper.create_album(&bkt_tkn, "bigknob").await?;
        assert!(album_response.files.is_empty());
        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn associate_file_with_album() -> Result<()> {
        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let url = "https://waifuvault.moe/assets/custom/images/08.png";

        let bucket = dropper.create_bucket().await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");

        let file = dropper.upload_file(request).await?;
        let album = dropper.create_album(&bucket.token, "kdkfjdk").await?;

        let success = match caller
            .associate_with_album(&album.token, &[&file.token])
            .await
        {
            Ok(success) => success,
            Err(e) => {
                dropper.destroy().await?;
                return Err(e);
            }
        };

        assert_eq!(success.bucket_token, bucket.token);
        assert_eq!(success.files.len(), 1);

        let f = &success.files[0];
        assert_eq!(f.token, file.token);
        assert_eq!(f.bucket, Some(bucket.token));

        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn disassociate_from_album() -> Result<()> {
        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let url = "https://waifuvault.moe/assets/custom/images/08.png";

        let bucket = dropper.create_bucket().await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");

        let file = dropper.upload_file(request).await?;
        let album = dropper.create_album(&bucket.token, "kdkfjdk").await?;

        let assoc = match caller
            .associate_with_album(&album.token, &[&file.token])
            .await
        {
            Ok(success) => success,
            Err(e) => {
                dropper.destroy().await?;
                return Err(e);
            }
        };

        assert_eq!(assoc.files.len(), 1);

        let disassoc = match caller
            .disassociate_from_album(&album.token, &[&file.token])
            .await
        {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert!(disassoc.files.is_empty());

        let _ = dropper.destroy().await;
        Ok(())
    }

    #[tokio::test]
    async fn delete_album_keep_files() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";

        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket = dropper.create_bucket().await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");

        dropper.upload_file(request).await?;
        let album = dropper.create_album(&bucket.token, "test_bucket").await?;

        let delete = match caller.delete_album(&album.token, false).await {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert!(delete.success);

        let bucket_info = caller.get_bucket(&bucket.token).await?;
        assert!(!bucket_info.files.is_empty());

        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn delete_album_remove_files() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";

        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket = dropper.create_bucket().await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");

        dropper.upload_file(request).await?;
        let album = dropper.create_album(&bucket.token, "test_bucket").await?;

        let delete = match caller.delete_album(&album.token, true).await {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert!(delete.success);

        let bucket_info = caller.get_bucket(&bucket.token).await?;
        assert!(bucket_info.files.is_empty());

        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn get_album() -> Result<()> {
        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket = dropper.create_bucket().await?;
        let album = dropper.create_album(&bucket.token, "test_bucket").await?;
        let retrieve = match caller.get_album(&album.token).await {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert_eq!(retrieve.token, album.token);
        assert_eq!(retrieve.bucket_token, bucket.token);
        assert!(retrieve.public_token.is_none());

        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn share_album() -> Result<()> {
        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket = dropper.create_bucket().await?;
        let album = dropper.create_album(&bucket.token, "test_bucket").await?;

        let share = match caller.share_album(&album.token).await {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert!(share.success);
        assert!(share
            .description
            .starts_with("http://localhost:8081/album/"));

        let _ = dropper.destroy().await;

        Ok(())
    }

    #[tokio::test]
    async fn download_album() -> Result<()> {
        let url = "https://waifuvault.moe/assets/custom/images/08.png";

        let caller = ApiCaller::new();
        let mut dropper = Dropper::new(&caller);
        let bucket = dropper.create_bucket().await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");

        let file = dropper.upload_file(request).await?;
        let request = WaifuUploadRequest::new()
            .bucket(&bucket.token)
            .url(url)
            .expires("1h");
        let file2 = dropper.upload_file(request).await?;
        let album = dropper.create_album(&bucket.token, "test_bucket").await?;
        caller
            .associate_with_album(&album.token, &[&file.token, &file2.token])
            .await?;

        let files = match caller.download_album(&album.token, None).await {
            Ok(success) => success,
            Err(e) => {
                let _ = dropper.destroy().await;
                return Err(e);
            }
        };

        assert!(!files.is_empty());

        let _ = dropper.destroy().await;

        Ok(())
    }

    fn hash_item(content: &Vec<u8>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(content);
        let raw = hasher.finalize();

        hex::encode(raw)
    }

    async fn be_nice() {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
