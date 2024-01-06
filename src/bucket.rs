
use anyhow::{ Result};
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::create_bucket::{CreateBucketError};
use aws_sdk_s3::operation::delete_bucket::{DeleteBucketError};
use aws_sdk_s3::operation::list_buckets::{ListBucketsError};
use aws_smithy_types::date_time::Format;


pub async fn create_bucket(client: &Client, bucket_name: &str) -> Result<String, SdkError<CreateBucketError>> {
	// BUILD - aws request
	client.create_bucket().bucket(bucket_name).send().await?;
	println!("创建成功！");
	Ok(String::from("OK"))
}

pub async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<String, SdkError<DeleteBucketError>> {
	client.delete_bucket().bucket(bucket_name).send().await?;
	println!("删除成功！");
	Ok(String::from("OK"))

}

pub async fn list_buckets(client: &Client) -> Result<String, SdkError<ListBucketsError>> {
	let resp = client.list_buckets().send().await?;
	let buckets = resp.buckets();
	println!("{:25}  {}", "桶名称", "创建时间");
	for bucket in buckets {
		println!("{:25}  {}", bucket.name().unwrap_or_default(), bucket.creation_date().unwrap().fmt(Format::DateTime).unwrap())
	}
	Ok(String::from("OK"))
}