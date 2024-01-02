pub mod conf;
pub mod client;
pub mod object;

use anyhow::{Result, Error};
use config::{Config, File};
use conf::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    println!("Hello, world!");

    let local_config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("构建配置错误");

    let config: AppConfig = local_config.try_deserialize().expect("反序列化配置文件错误");

    let s3_client = client::get_aws_client(&config);

    let keys = object::list_keys(&s3_client, "test").await?;
    println!("List:\n{}", keys.join("\n"));
    

    println!("endpoint_url:{}", config.endpoint_url);
    println!("region:{}", config.region);
    println!("access_key:{}", config.access_key);
    println!("secret_key:{}", config.secret_key);
    println!("path_style:{}", config.path_style);

    Ok(())
}
