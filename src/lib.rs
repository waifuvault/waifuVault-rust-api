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
//!     api::{WaifuUploadRequest, WaifuResponse}
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
pub mod api;

use std::{collections::HashMap, path::PathBuf};

use api::*;

use anyhow::Context;
use reqwest::Client;

/// REST endpoint for the service
const API: &str = "https://waifuvault.moe/rest";

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

    pub async fn create_bucket(&self) -> anyhow::Result<WaifuBucketResponse> {
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

    pub async fn delete_bucket(&self, token: impl AsRef<str>) -> anyhow::Result<()> {
        let url = format!("{API}/bucket/{}", token.as_ref());
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
            WaifuApiResponse::Delete(_) => Ok(()),
            WaifuApiResponse::WaifuError(err) => Err(err.into()),
            _ => anyhow::bail!("Received unexpected response from DELETE bucket endpoint"),
        }
    }

    pub async fn get_bucket(&self, token: impl AsRef<str>) -> anyhow::Result<WaifuBucketResponse> {
        let url = format!("{API}/bucket/get");
        let mut body = HashMap::new();
        body.insert("bucket_token", token.as_ref());

        let response: WaifuApiResponse = self
            .client
            .post(&url)
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
    ///     api::{WaifuUploadRequest, WaifuResponse}
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
    pub async fn upload_file(&self, request: WaifuUploadRequest) -> anyhow::Result<WaifuResponse> {
        let url = if let Some(bucket) = request.bucket {
            &format!("{API}/{bucket}")
        } else {
            API
        };

        let request = {
            let mut intermediate = self.client.put(url).query(&[
                ("hide_filename", request.one_time_download),
                ("one_time_download", request.one_time_download),
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
    ///     api::{WaifuGetRequest, WaifuResponse}
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
    pub async fn file_info(&self, request: WaifuGetRequest) -> anyhow::Result<WaifuResponse> {
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
    ///     api::{WaifuModificationRequest, WaifuResponse}
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
    ) -> anyhow::Result<WaifuResponse> {
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
    pub async fn delete_file(&self, token: impl AsRef<str>) -> anyhow::Result<bool> {
        let url = format!("{API}/{}", token.as_ref());
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
        url: impl AsRef<str>,
        password: Option<String>,
    ) -> anyhow::Result<Vec<u8>> {
        let request = {
            let mut r = self.client.get(url.as_ref());
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
}

/// Parses the response from the Waifu Vault API and converts it to
/// a concrete type
pub(crate) fn parse_response(response: WaifuApiResponse) -> anyhow::Result<WaifuResponse> {
    match response {
        WaifuApiResponse::WaifuResponse(resp) => Ok(resp),
        WaifuApiResponse::WaifuError(err) => Err(anyhow::anyhow!(err)),
        WaifuApiResponse::Delete(_) => unreachable!("unused"),
        WaifuApiResponse::WaifuBucketResponse(_) => unreachable!("unused"),
    }
}

#[cfg(test)]
mod tests {
    // These tests run against the actual API endpoint because i dont know how to mock these calls
    // Each test has a `be_nice()` call which adds a small delay for requests
    // Until mocking is easier, test liberally
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
        let success = caller.delete_file(token).await?;

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

        let response = caller.download_file(url, None).await?;
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
            .download_file(url, Some("banana".to_string()))
            .await?;
        let result = hash_item(&response);

        assert_eq!(og_hash, result);
        be_nice().await;
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
