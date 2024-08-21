# Waifu Vault SDK

This is the official API bindings for interacting with the [Waifu Vault](https://waifuvault.moe) API.

For more information on Terms of Service and usage policy, please refer to the above website.

## Install

```bash
cargo add waifuvault
```

# Usage

The following interactions are allowed:

* [Upload a File](#upload-file)
* [Get File Information](#file-info)
* [Modify File Options](#modify-file)
* [Delete a File](#delete-file)
* [Download a File](#download-file)
* [Create a Bucket](#create-bucket)
* [Delete a Bucket](#delete-bucket)
* [Get Bucket Information](#get-bucket)

## Upload a File<a id="upload-file"></a>

<a>

The following options can be set when creating a `WaifuUploadRequest`:

* `file`: Optional value to upload a file from disk
* `url`: Optional value to upload content from a URL
* `bytes`: Optional value to upload raw bytes
* `bucket`: Optional value to upload the file to a specific bucket
* `expires`: Optional value to define the expiry time for the content
    * Valid values are: `m`, `h`, `d`
    * If not set, the content exists for as long as the retention policy of the service
* `hide_filename`: Optional flag to set to hide the filename from the URL generated
* `password`: Optional value to set if the content should be encrypted or not
* `one_time_download`: Optional flag to set if the content should be deleted after first access 


 ```rust
 use waifuvault::{
     ApiCaller,
     api::{WaifuUploadRequest, WaifuResponse}
 };

 #[tokio::main]
 async fn main() -> anyhow::Result<()> {
     let caller = ApiCaller::new();

     // Upload a file from disk
     let request = WaifuUploadRequest::new()
         .file("/some/file/path") // Path to a file
         .password("set a password") // Set a password
         .one_time_download(true); // Delete after first access
     let response = caller.upload_file(request).await?;

     // Upload a file from a URL
     let request = WaifuUploadRequest::new()
         .url("https://some-website/image.jpg"); // URL to content
     let response = caller.upload_file(request).await?;

     // Upload a file from raw bytes
     let data = std::fs::read("some/file/path")?;
     let request = WaifuUploadRequest::new()
         .bytes(data, "name-to-store.rs"); // Raw file content and name to store on the vault
     let response = caller.upload_file(request).await?;

     Ok(())
 }
 ```

## Get File Information<a id="file-info"></a>

Retrieves information about a file stored with the API

This requires a token that is obtained from the response when uploading a file.

The following parameters can be set when using the `WaifuGetRequest`:

* `token`: The token used to retrieve the file
* `formatted`: Optional flag to determine if the expiry time is human-readable


```rust
use waifuvault::{
    ApiCaller,
    api::WaifuGetRequest
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    let request = WaifuGetRequest::new("some-waifu-vault-token");
    let response = caller.file_info(request).await?;

    Ok(())
}
```

## Modify File Options<a id="modify-file"></a>

Modifies the options for a stored file in the API

The following parameters can be used to update a file's information:

* `password`: Sets a new password for a file
    * If a password already exists, `previous_password` must also be used
* `previous_password`: The previous password for the file (required when setting a new password on encrypted content)
* `custom_expiry`: Sets a new expiry time for the content
* `hide_filename`: Sets the flag to hide the filename from the URL


```rust
use waifuvault::{
    ApiCaller,
    api::WaifuModificationRequest
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    let request = WaifuModificationRequest::new("some-waifu-vault-token")
        .password("new_password") // Set a new password
        .previous_password("old_password") // Old password
        .custom_expiry("1h") // Set a new expiry
        .hide_filename(true); // Hide the filename

    let response = caller.update_file(request).await?;

    // Do something with the response

    Ok(())
}
```

## Delete a File<a id="delete-file"></a>

Deletes a file using the API denoted by the content token.


```rust,no_run
use waifuvault::ApiCaller;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();
    let response = caller.delete_file("some-waifu-token").await?;

    Ok(())
}
```

## Download a File<a id="download-file"></a>

Downloads a file from the API with the given token

```rust
use waifuvault::ApiCaller;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    // Download a file with no password
    let content = caller.download_file("https://waifuvault.moe/f/some-file.ext", None).await?;
    let mut f = std::fs::File::create("downloaded_file.txt")?;
    f.write_all(&content)?;

    // Download a file with no password
    let content = caller.download_file("https://waifuvault.moe/f/some-other-file.ext", Some("password".to_string())).await?;
    let mut f = std::fs::File::create("downloaded_file2.txt")?;
    f.write_all(&content)?;

    Ok(())
}
```

## Create a Bucket<a id="create-bucket"></a>

Creates a new bucket with the API to upload files to

```rust
use waifuvault::{ApiCaller, api::WaifuUploadRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    // Create a new bucket to upload files to
    let bucket = caller.create_bucket().await?;

    // You can now use the bucket token to upload files to the bucket

    let request = WaifuUploadRequest::new()
        .file("/some/file/path")
        .bucket(&bucket.token)
        .password("set a password")
        .one_time_download(true);
    let response = caller.upload_file(request).await?;

    // Do something with the response

    Ok(())
}
```

## Delete a Bucket<a id="delete-bucket"></a>

Delete a bucket and all the files contained within it.

The following parameters are required:

* `token`: The bucket token for the bucket to delete


```rust
use waifuvault::ApiCaller;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    let token = "some-bucket-token";

    // Delete the bucket and all files within
    caller.delete_bucket(token).await?;

    Ok(())
}
```

## Get Bucket Information<a id="get-bucket"></a>

Retrieve information about files contained within a bucket.

The following parameters are required:

* `token`: The bucket token for the bucket to inspect


```rust
use waifuvault::ApiCaller;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let caller = ApiCaller::new();

    let token = "some-bucket-token";

    // Get bucket information
    let info = caller.get_bucket(token).await?;

    // You can now get access to the file information for files inside the bucket
    for file in info.files.iter() {
        // Do something with the file information
    }

    Ok(())
}
```
