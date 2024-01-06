
use anyhow::{ Result};
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_smithy_types::date_time::Format;

pub async fn list_objects(client: &Client, bucket_name: &str, prefix: &str) -> Result<String, SdkError<ListObjectsV2Error>> {
	// BUILD - aws request
	let req = client.list_objects_v2()
		.bucket(bucket_name)
		.prefix(prefix);

	// EXECUTE
	let res = req.send().await?;

	// COLLECT
	let objects = res.contents();
	println!("{:<48} | {:<5} | {:<30}", "对象key", "对象大小", "最后修改时间");
	for object in objects {
		println!("{:<50} | {:<9} | {:<30}", object.key().unwrap(), format_size(object.size().unwrap() as f64), object.last_modified().unwrap().fmt(Format::DateTime).unwrap() )
	}

	Ok(String::from("OK"))
}

fn format_size(bytes: f64) -> String {
	let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
	let mut index = 0;
	let mut size = bytes;

	while size >= 1024.0 && index < units.len() - 1 {
		size /= 1024.0;
		index += 1;
	}

	format!("{:.2} {}", size, units[index])
}