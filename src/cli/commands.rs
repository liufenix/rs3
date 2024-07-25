use anyhow::Result;
use chrono::SecondsFormat;
use log::info;

use crate::s3::client::S3Client;

pub enum Command {
    ListBuckets,
    CreateBucket {
        name: String,
    },
    DeleteBucket {
        name: String,
    },
    ListObjects {
        bucket: String,
        prefix: Option<String>,
    },
    UploadObject {
        bucket: String,
        key: String,
        file_path: String,
    },
    DownloadObject {
        bucket: String,
        key: String,
        file_path: String,
    },
    DeleteObject {
        bucket: String,
        key: String,
    },
}

impl Command {
    pub async fn execute(&self, client: &S3Client) -> Result<()> {
        match self {
            Command::ListBuckets => {
                info!("Listing buckets");
                let buckets = client.list_buckets().await?;
                for bucket in buckets {
                    println!("{:30}   {}",
                             bucket.name,
                             bucket.creation_date
                                 .map(|d| d.to_rfc3339_opts(SecondsFormat::Secs, true))
                                 .unwrap_or_else(|| "Unknown".to_string()))
                }
            }
            Command::CreateBucket { name } => {
                info!("Creating bucket: {}", name);
                client.create_bucket(name).await?;
                println!("Bucket '{}' created successfully", name);
            }
            Command::DeleteBucket { name } => {
                info!("Deleting bucket: {}", name);
                client.delete_bucket(name).await?;
                println!("Bucket '{}' deleted successfully", name);
            }
            Command::ListObjects { bucket, prefix } => {
                info!("Listing objects in bucket: {}", bucket);
                let objects = client.list_objects(bucket, prefix.as_deref()).await?;
                for object in objects {
                    println!("{}", object);
                }
            }
            Command::UploadObject {
                bucket,
                key,
                file_path,
            } => {
                info!("Uploading object to bucket: {}", bucket);
                client.upload_object(bucket, key, file_path).await?;
                println!(
                    "Object '{}' uploaded successfully to bucket '{}'",
                    key, bucket
                );
            }
            Command::DownloadObject {
                bucket,
                key,
                file_path,
            } => {
                info!("Downloading object from bucket: {}", bucket);
                client.download_object(bucket, key, file_path).await?;
                println!(
                    "Object '{}' downloaded successfully from bucket '{}'",
                    key, bucket
                );
            }
            Command::DeleteObject { bucket, key } => {
                info!("Deleting object from bucket: {}", bucket);
                client.delete_object(bucket, key).await?;
                println!(
                    "Object '{}' deleted successfully from bucket '{}'",
                    key, bucket
                );
            }
        }
        Ok(())
    }
}
