pub mod conf;
pub mod client;
pub mod object;
pub mod bucket;
mod error;

use std::{ffi::{OsString, OsStr}, path::PathBuf};

use anyhow::{Result};
use aws_smithy_types::date_time::Format;
use config::{Config, File};
use conf::AppConfig;

// clap 引用
use clap::{Args, Parser, Subcommand, ValueEnum};
use crate::error::Error;

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
    ListBucket,
    /// Clones repos
    #[command(arg_required_else_help = true)]
    CreateBucket {
        /// The remote to clone
        bucket: String,
    },
    DeleteBucket {
        /// The remote to clone
        bucket: String,
    },
    /// Compare two commits
    Diff {
        #[arg(value_name = "COMMIT")]
        base: Option<OsString>,
        #[arg(value_name = "COMMIT")]
        head: Option<OsString>,
        #[arg(last = true)]
        path: Option<OsString>,
        #[arg(
            long,
            require_equals = true,
            value_name = "WHEN",
            num_args = 0..=1,
            default_value_t = ColorWhen::Auto,
            default_missing_value = "always",
            value_enum
        )]
        color: ColorWhen,
    },
    /// pushes things
    #[command(arg_required_else_help = true)]
    Push {
        /// The remote to target
        remote: String,
    },
    /// adds things
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    Stash(StashArgs),
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommands>,

    #[command(flatten)]
    push: StashPushArgs,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
}

#[derive(Debug, Args)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    println!("Hello, world!");

    let local_config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("构建配置错误");

    let config: AppConfig = local_config.try_deserialize().expect("反序列化配置文件错误");

    let s3_client = client::get_aws_client(&config);

    let args = Cli::parse();

    match args.command {
        Commands::ListBucket => {
            let resp = bucket::list_bucket(&s3_client).await?;
            let buckets = resp.buckets();
            println!("{:25}  {}", "桶名称", "创建时间");
            for bucket in buckets {
                println!("{:25}  {}", bucket.name().unwrap_or_default(), bucket.creation_date().unwrap().fmt(Format::DateTime).unwrap())
            }
        }
        Commands::CreateBucket { bucket } => {
            println!("CreateBucket -> {bucket}");
            let resp = bucket::create_bucket(&s3_client, &bucket).await;
            println!("result -> {}", resp.is_ok())
        }
        Commands::DeleteBucket { bucket } => {
            println!("DeleteBucket -> {bucket}");
            // let resp = bucket::delete_bucket(&s3_client, &bucket).await;
            // if resp.is_err(){
            //     println!("error -> {:?}", resp.unwrap_err().into_service_error())
            // }
            bucket::delete_bucket(&s3_client, &bucket).await?;
        }
        Commands::Diff {
            mut base,
            mut head,
            mut path,
            color,
        } => {
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("stage");
            let head = head
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("worktree");
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            println!(
                "Diffing {}..{} {} (color={})",
                base,
                head,
                path.to_string_lossy(),
                color
            );
        }
        Commands::Push { remote } => {
            println!("Pushing to {remote}");
        }
        Commands::Add { path } => {
            println!("Adding {path:?}");
        }
        Commands::Stash(stash) => {
            let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
            match stash_cmd {
                StashCommands::Push(push) => {
                    println!("Pushing {push:?}");
                }
                StashCommands::Pop { stash } => {
                    println!("Popping {stash:?}");
                }
                StashCommands::Apply { stash } => {
                    println!("Applying {stash:?}");
                }
            }
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }

    

    // let keys = object::list_keys(&s3_client, "test").await?;
    // println!("List:\n{}", keys.join("\n"));

    Ok(())
}
