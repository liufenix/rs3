
use anyhow::{anyhow, bail, Context, Result}; 
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_smithy_types::date_time::Format;

pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<Vec<String>, SdkError<ListObjectsV2Error>> {
	// BUILD - aws request
	let req = client.list_objects_v2()
		.bucket(bucket_name)
		.prefix("");

	// EXECUTE
	let res = req.send().await?;

	// COLLECT
	let objects = res.contents();
	println!("{:<98} | {:<26} | {:<30}", "对象key", "对象大小", "最后修改时间");
	for object in objects {
		println!("{:<100} | {:<30} | {:<30}", object.key().unwrap(), object.size().unwrap(), object.last_modified().unwrap().fmt(Format::DateTime).unwrap() )
	}

	let keys = objects
		.iter()
		.filter_map(|o| o.key.as_ref())
		.map(|s| s.to_string())
		.collect::<Vec<_>>();

	Ok(keys)
}
