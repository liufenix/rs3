use std::panic::panic_any;
use anyhow::{anyhow, bail, Context, Result};
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::create_bucket::{CreateBucketError, CreateBucketOutput};
use aws_sdk_s3::operation::delete_bucket::{DeleteBucketError, DeleteBucketOutput};
use aws_sdk_s3::operation::list_buckets::{ListBucketsError, ListBucketsOutput};


pub async fn create_bucket(client: &Client, bucket_name: &str) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
	// BUILD - aws request
	client.create_bucket().bucket(bucket_name).send().await
}

pub async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<DeleteBucketOutput, SdkError<DeleteBucketError>> {
	client.delete_bucket().bucket(bucket_name).send().await
}

pub async fn list_buckets(client: &Client) -> Result<ListBucketsOutput, SdkError<ListBucketsError>> {
	client.list_buckets().send().await
}