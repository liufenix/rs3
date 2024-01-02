

use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::{config, Client};
use crate::conf;


pub fn get_aws_client(config: &conf::AppConfig) -> aws_sdk_s3::Client {
	// get the id/secret from env
	let key_id = &config.access_key;
	let key_secret = &config.secret_key;
	let endpoint_url = &config.endpoint_url;
    let region = &config.region;
    let path_style = config.path_style;

	// build the aws cred
	let cred = Credentials::new(key_id, key_secret, None, None, "loaded-from-custom-env");

	// build the aws client
	let region = Region::new(region.clone());

	let conf_builder = config::Builder::new().behavior_version_latest()
		.endpoint_url(endpoint_url)
		.force_path_style(path_style)
		.region(region)
		.credentials_provider(cred);
	let conf = conf_builder.build();

	// build aws client
	Client::from_conf(conf)
	
}