pub mod bucket;
pub mod client;
pub mod conf;
mod error;
pub mod object;

use anyhow::Result;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;
use conf::AppConfig;
use config::{Config, File};

// clap 引用
use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rs3")]
#[command(about = "aws s3 CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ListBuckets,
    /// Clones repos
    #[command(arg_required_else_help = true)]
    CreateBucket {
        /// The remote to clone
        bucket: String,
    },
    #[command(arg_required_else_help = true)]
    DeleteBucket {
        /// The remote to clone
        bucket: String,
    },
    ListObjects {
        /// the bucket
        #[arg(short, long)]
        bucket: String,

        /// the prefix
        #[arg(short, long,
        required = false,
        default_value_t = String::from(""),
        default_missing_value = "")]
        prefix: String,
    },
    PutObject {
        /// the bucket
        #[arg(short, long)]
        bucket: String,
        /// key
        #[arg(long)]
        prefix: String,
        /// local file path
        #[arg(short, long)]
        path: String,
    },
    DeleteObject {
        /// the bucket
        #[arg(short, long)]
        bucket: String,
        /// key
        #[arg(short, long)]
        key: String,
    },
    DownloadObject {
        /// the bucket
        #[arg(short, long)]
        bucket: String,
        /// key
        #[arg(short, long)]
        key: String,
        /// local file path
        #[arg(short, long)]
        dir: String,
    },
    HeadObject {
        /// the bucket
        #[arg(short, long)]
        bucket: String,
        /// the key
        #[arg(short, long)]
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let local_config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("构建配置错误");

    let config: AppConfig = local_config
        .try_deserialize()
        .expect("反序列化配置文件错误");

    let s3_client = client::get_aws_client(&config);

    let args = Cli::parse();

    match args.command {
        Commands::ListBuckets => {
            bucket::list_buckets(&s3_client).await?;
        }
        Commands::CreateBucket { bucket } => {
            bucket::create_bucket(&s3_client, &bucket).await?;
        }
        Commands::DeleteBucket { bucket } => {
            let resp = bucket::delete_bucket(&s3_client, &bucket).await;
            match resp {
                Ok(_) => {}
                Err(error) => {
                    let delete_error = error.into_service_error();
                    //println!("{:?}", &delete_error);
                    println!(
                        "删除失败：{} {}",
                        &delete_error.code().unwrap_or("403"),
                        &delete_error.message().unwrap_or("")
                    )
                }
            }
        }
        Commands::ListObjects { bucket, prefix } => {
            object::list_objects(&s3_client, &bucket, &prefix).await?;
        }
        Commands::PutObject {
            bucket,
            prefix,
            path,
        } => {
            object::put_object(&s3_client, &bucket, &prefix, &path).await?;
        }
        Commands::DeleteObject { bucket, key } => {
            object::delete_object(&s3_client, &bucket, &key).await?;
        }
        Commands::DownloadObject { bucket, key, dir } => {
            object::download_object(&s3_client, &bucket, &key, &dir).await?;
        }
        Commands::HeadObject { bucket, key } => {
            object::head_object(&s3_client, &bucket, &key).await?;
        }
    }

    Ok(())
}
