//! API types that can be received from the Waifu Vault API
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The main API responses that can be received
///
/// Serde will deserialize these into the appropriate type
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum WaifuApiResponse {
    /// Everything is good, contains the info about the uploaded file
    WaifuFileResponse(WaifuFileEntry),

    /// Everything is good, contains info about the album
    WaifuAlbumResponse(WaifuAlbumEntry),

    /// Everything is good, contains info about the bucket
    WaifuBucketResponse(WaifuBucketEntry),

    /// Everything is good, contains generic success / failure message
    WaifuGenericResponse(WaifuGenericMessage),

    /// Something went wrong, shows the error type and reason
    WaifuError(WaifuError),

    /// Special case for the delete endpoint, just a boolean flag
    /// if it was successful or not
    Delete(bool),
}

/// This is a standard response for the service containing info about the entry
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuFileEntry {
    /// File token - used for file info and deleting
    pub token: String,

    /// Location of the uploaded file
    pub url: String,

    /// Bucket identifier
    pub bucket: Option<String>,

    /// Album the file is associated with, if any
    pub album: Option<WaifuAlbumMetadata>,

    /// Number of views the file has
    pub views: usize,

    /// How long the file will exist for
    #[serde(rename = "retentionPeriod")]
    pub retention_period: serde_json::Value,

    /// Response options for the file
    pub options: Option<WaifuFileOptions>,
}

/// Response options for the uploaded file
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct WaifuFileOptions {
    /// If the filename is hidden
    #[serde(rename = "hideFilename")]
    pub hide_filename: bool,

    /// If this file will be deleted when it is accessed
    #[serde(rename = "oneTimeDownload")]
    pub one_time_download: bool,

    /// If this file requires a password
    pub protected: bool,
}

/// Successful response from the API when interacting with the Bucket API
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuBucketEntry {
    /// Bucket token identifier
    pub token: String,

    /// Files contained within the bucket
    pub files: Vec<WaifuFileEntry>,

    /// Albums associated with the bucket, if any
    pub albums: Option<Vec<WaifuAlbumMetadata>>,
}

/// Successful response from the API when interacting with the Album API
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuAlbumEntry {
    /// Album token identifier
    pub token: String,

    /// Bucket token identifier
    #[serde(rename = "bucketToken")]
    pub bucket_token: String,

    /// Public token identifier
    #[serde(rename = "publicToken")]
    pub public_token: Option<String>,

    /// Name of the Album
    pub name: String,

    /// Files contained within the Album
    pub files: Vec<WaifuFileEntry>,
}

/// Album metadata which shows which album a file is apart of
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuAlbumMetadata {
    /// Album token
    pub token: String,

    /// Public token
    #[serde(rename = "publicToken")]
    pub public_token: Option<String>,

    /// Album name
    pub name: String,

    /// Bucket name
    pub bucket: String,

    /// Date the album was created
    #[serde(rename = "dateCreated")]
    pub date_created: u64,
}

/// Generic response returned by the API indicating success / failure of operation
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuGenericMessage {
    /// If the operation was a success or not
    pub success: bool,

    /// Description provided by the API
    pub description: String,
}

/// A standard error, all errors from the service take this shape
#[derive(Debug, Deserialize, Clone)]
pub struct WaifuError {
    /// The name of the error, this is normally the HTTP exception thrown
    pub name: String,

    /// The thing that went wrong
    pub message: String,

    /// The HTTP status
    pub status: u16,
}

impl std::fmt::Display for WaifuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WaifuError {} ({})\nMessage: {}",
            self.name, self.status, self.message
        )
    }
}

impl std::error::Error for WaifuError {}

/// Upload request to upload content to the Waifu Vault
#[derive(Debug, Default, Clone)]
pub struct WaifuUploadRequest {
    /// Path to a file to upload
    pub(crate) file: Option<String>,

    /// URL of a resource to upload
    pub(crate) url: Option<String>,

    /// Raw bytes to upload to the vault
    pub(crate) bytes: Option<Vec<u8>>,

    /// Token of the bucket to upload to
    pub(crate) bucket: Option<String>,

    /// Filename to be used when uploading raw bytes
    pub(crate) filename: Option<String>,

    /// Set an expiry for the content
    /// This is a string containing a number and a letter (m for mins, h for hours, d for days)
    /// Leave blank to keep the file for as long as the retention policy allows
    pub(crate) expires: Option<String>,

    /// Hide the filename from the generated URL
    pub(crate) hide_filename: bool,

    /// Set a password for the file
    /// This encrypts the file on the server which can only be accessed by
    /// retrieving it with the x-password header set
    pub(crate) password: Option<String>,

    /// Delete the file after first access
    pub(crate) one_time_download: bool,
}

impl WaifuUploadRequest {
    /// Create a new upload request
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the file field on the request
    pub fn file(mut self, file: impl AsRef<Path>) -> Self {
        let file = file.as_ref().display().to_string();
        self.file = Some(file);
        self
    }

    /// Sets the url field on the request
    pub fn url(mut self, url: impl AsRef<str>) -> Self {
        self.url = Some(url.as_ref().to_string());
        self
    }

    /// Sets the bytes field on the request
    pub fn bytes(mut self, bytes: Vec<u8>, filename: impl AsRef<str>) -> Self {
        self.bytes = Some(bytes);
        self.filename = Some(filename.as_ref().to_string());
        self
    }

    /// Sets the bucket token on the request
    pub fn bucket(mut self, token: impl AsRef<str>) -> Self {
        self.bucket = Some(token.as_ref().to_string());
        self
    }

    /// Sets the expires field on the request
    pub fn expires(mut self, expires: impl AsRef<str>) -> Self {
        self.expires = Some(expires.as_ref().to_string());
        self
    }

    /// Sets the hide_filename field on the request
    pub fn hide_filename(mut self, hide: bool) -> Self {
        self.hide_filename = hide;
        self
    }

    /// Sets the password field on the request
    pub fn password(mut self, password: impl AsRef<str>) -> Self {
        self.password = Some(password.as_ref().to_string());
        self
    }

    /// Sets the one_time_download field on the request
    pub fn one_time_download(mut self, otd: bool) -> Self {
        self.one_time_download = otd;
        self
    }
}

/// Request to be sent when requesting file information from the API
#[derive(Debug, Default, Clone)]
pub struct WaifuGetRequest {
    /// Token used to access the content
    pub(crate) token: String,

    /// Flag to display the expiry time in human-readable format
    pub(crate) formatted: bool,
}

impl WaifuGetRequest {
    /// Create a new Get Request
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            token: token.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Set the formatted field on the request
    pub fn formatted(mut self, format: bool) -> Self {
        self.formatted = format;
        self
    }
}

/// Modification request to be sent when updating options on
/// the target resource stored in the vault
#[derive(Debug, Default, Clone, Serialize)]
pub struct WaifuModificationRequest {
    /// Token used to access the content
    #[serde(skip)]
    pub(crate) token: String,

    /// Sets a password for the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) password: Option<String>,

    /// The previous password of the content used when switching password
    /// if one existed before
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previousPassword")]
    pub(crate) previous_password: Option<String>,

    /// Update the expiry time on the content
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customExpiry")]
    pub(crate) custom_expiry: Option<String>,

    /// Update the hide_filename flag for the content
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideFilename")]
    pub(crate) hide_filename: Option<bool>,
}

impl WaifuModificationRequest {
    /// Create a new Modification request
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            token: token.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Set the password field on the request
    pub fn password(mut self, password: impl AsRef<str>) -> Self {
        self.password = Some(password.as_ref().to_string());
        self
    }

    /// Set the previous_password field on the request
    pub fn previous_password(mut self, prev_pwd: impl AsRef<str>) -> Self {
        self.previous_password = Some(prev_pwd.as_ref().to_string());
        self
    }

    /// Set the custom_expiry field on the request
    pub fn custom_expiry(mut self, expiry: impl AsRef<str>) -> Self {
        self.custom_expiry = Some(expiry.as_ref().to_string());
        self
    }

    /// Set the hide_filename field on the request
    pub fn hide_filename(mut self, hide: bool) -> Self {
        self.hide_filename = Some(hide);
        self
    }
}
