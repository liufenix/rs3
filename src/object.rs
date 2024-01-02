
use anyhow::{anyhow, bail, Context, Result}; 
use aws_sdk_s3::Client;

pub async fn list_keys(client: &Client, bucket_name: &str) -> Result<Vec<String>> {
	// BUILD - aws request
	let req = client.list_objects_v2().prefix("").bucket(bucket_name);

	// EXECUTE
	let res = req.send().await?;

	// COLLECT
	let keys = res.contents();
	let keys = keys
		.iter()
		.filter_map(|o| o.key.as_ref())
		.map(|s| s.to_string())
		.collect::<Vec<_>>();

	Ok(keys)
}