use std::fs;
use std::path::Path;
use anyhow::{anyhow, bail, Result};
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

use aws_smithy_types::date_time::Format;

use async_recursion::async_recursion;


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

pub async fn put_object(client: &Client, bucket: &String, prefix: &String, path: &String) -> Result<()>{
	let path = Path::new(path);
	if !path.exists() {
		bail!("Path {} does not exists", path.display());
	}
	if path.is_dir() {
		upload_dir(client, bucket, &prefix, path).await?
	} else {
		upload_file(client, bucket, prefix, path).await?
	}

	Ok(())
}

pub async fn upload_file(client: &Client, bucket: &String, prefix: &String, path: &Path) -> Result<()> {
	// PREPARE
	let body = ByteStream::from_path(path).await?;
	let content_type = mime_guess::from_path(path).first_or_octet_stream().to_string();
	let mut key = prefix.clone();
	key.push_str( path.file_name().unwrap().to_str().unwrap().trim());
	client.put_object().bucket(bucket).key(key).body(body).content_type(content_type).send().await?;
	println!("upload {} SUCCESS", path.display());
	Ok(())
}

#[async_recursion]
pub async fn upload_dir(client: &Client, bucket: &String, prefix: &String, path: &Path) -> Result<()> {
	for entry in fs::read_dir(path)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			let mut prefix = prefix.clone();
			prefix.push_str(path.file_name().unwrap().to_str().unwrap().trim());
			prefix.push_str("/");
			upload_dir(client, bucket, &prefix, &path).await?;
		} else {
			upload_file(client, bucket, prefix, &path).await?;
		}
	}
	Ok(())
}

pub async fn delete_object(client: &Client, bucket: &String, key: &String) -> Result<String, SdkError<DeleteObjectError>> {
	client.delete_object().bucket(bucket)
		.key(key)
		.send().await?;
	println!("删除对象成功！");
	Ok(String::from("OK"))
}

pub async fn download_object(client: &Client, bucket: &String, key: &String, dir: &String) -> Result<()> {
	let dir = Path::new(dir);
	// VALIDATE
	if !dir.is_dir() {
		bail!("Path {} is not a directory", dir.display());
	}
	// create file path and parent dir(s)
	let file_path = dir.join(key);
	let parent_dir = file_path
		.parent()
		.ok_or_else(|| anyhow!("Invalid parent dir for {:?}", file_path))?;
	if !parent_dir.exists() {
		create_dir_all(parent_dir)?;
	}

	// BUILD - aws request
	let req = client.get_object().bucket(bucket).key(key);

	// EXECUTE
	let res = req.send().await?;

	// STREAM result to file
	let mut data: ByteStream = res.body;
	let file = File::create(&file_path)?;
	let mut buf_writer = BufWriter::new(file);
	while let Some(bytes) = data.try_next().await? {
		buf_writer.write(&bytes)?;
	}
	buf_writer.flush()?;
	println!("下载成功！");
	Ok(())
}

pub async fn head_object(client: &Client, bucket: &str, key: &str) -> Result<()>{
	let resp = client.head_object().bucket(bucket).key(key).send().await?;
	println!("head result \n\r{:?}", resp);
	Ok(())
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


