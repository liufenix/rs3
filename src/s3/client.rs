use crate::config::app_config::AppConfig;
use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_s3::{Client as AwsS3Client, Client, Config};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::config::{BehaviorVersion, Builder, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_s3::types::Bucket;
use chrono::{DateTime, TimeZone, Utc};

pub struct S3Client {
    client: AwsS3Client,
}

pub struct BucketInfo {
    pub name: String,
    pub creation_date: Option<DateTime<Utc>>,
}

impl From<&Bucket> for BucketInfo {
    fn from(bucket: &Bucket) -> Self {
        BucketInfo {
            name: bucket.name().unwrap_or_default().to_string(),
            creation_date: bucket.creation_date().and_then(|dt| {
                Utc.timestamp_opt(dt.secs(), dt.subsec_nanos()).single()
            }),
        }
    }
}

impl S3Client {
    pub fn new(config: &AppConfig) -> Result<Self> {
        let cred = Credentials::new(
            &config.access_key,
            &config.secret_key,
            None,
            None,
            "rs3",
        );
        let client_config = Builder::new().behavior_version_latest()
            .endpoint_url(&config.endpoint_url)
            .force_path_style(config.path_style)
            .region(Region::new(config.region.clone()))
            .credentials_provider(cred).build();
        let client = Client::from_conf(client_config);

        // 另一种初始化方式：无法配置path_style
        // let sdk_config = SdkConfig::builder()
        //     .behavior_version(BehaviorVersion::latest())
        //     .region(Region::new(config.region.clone()))
        //     .endpoint_url(config.endpoint_url.clone())
        //     .credentials_provider(SharedCredentialsProvider::new(cred))
        //     .build();
        // let s3_config = Config::new(&sdk_config);
        // let client = AwsS3Client::from_conf(client_config);


        Ok(Self { client })
    }

    pub async fn list_buckets(&self) -> Result<Vec<BucketInfo>> {
        let resp = self.client.list_buckets().send().await?;
        Ok(resp
            .buckets()
            // .unwrap_or_default()
            .iter()
            .map(BucketInfo::from)
            // .filter_map(|b| b.name().map(String::from))
            .collect())
    }

    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        self.client.create_bucket().bucket(name).send().await?;
        Ok(())
    }

    pub async fn delete_bucket(&self, name: &str) -> Result<()> {
        self.client.delete_bucket().bucket(name).send().await?;
        Ok(())
    }

    pub async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<String>> {
        let mut req = self.client.list_objects_v2().bucket(bucket);
        if let Some(p) = prefix {
            req = req.prefix(p);
        }
        let resp = req.send().await?;
        Ok(resp
            .contents()
            // .unwrap_or_default()
            .iter()
            .filter_map(|obj| obj.key().map(String::from))
            .collect())
    }

    pub async fn upload_object(&self, bucket: &str, key: &str, file_path: &str) -> Result<()> {
        let body =
            ByteStream::from_path(std::path::Path::new(file_path)).await?;
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn download_object(&self, bucket: &str, key: &str, file_path: &str) -> Result<()> {
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        let body = resp.body.collect().await?;
        std::fs::write(file_path, body.into_bytes())?;
        Ok(())
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }
}
