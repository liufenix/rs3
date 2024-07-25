use super::commands::Command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs3")]
#[command(about = "AWS S3 CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ListBuckets,
    CreateBucket {
        name: String,
    },
    DeleteBucket {
        name: String,
    },
    ListObjects {
        bucket: String,
        #[arg(short, long)]
        prefix: Option<String>,
    },
    UploadObject {
        bucket: String,
        key: String,
        file_path: String,
    },
    DownloadObject {
        bucket: String,
        key: String,
        file_path: String,
    },
    DeleteObject {
        bucket: String,
        key: String,
    },
}

pub fn parse_cli() -> Command {
    let cli = Cli::parse();
    match cli.command {
        Commands::ListBuckets => Command::ListBuckets,
        Commands::CreateBucket { name } => Command::CreateBucket { name },
        Commands::DeleteBucket { name } => Command::DeleteBucket { name },
        Commands::ListObjects { bucket, prefix } => Command::ListObjects { bucket, prefix },
        Commands::UploadObject {
            bucket,
            key,
            file_path,
        } => Command::UploadObject {
            bucket,
            key,
            file_path,
        },
        Commands::DownloadObject {
            bucket,
            key,
            file_path,
        } => Command::DownloadObject {
            bucket,
            key,
            file_path,
        },
        Commands::DeleteObject { bucket, key } => Command::DeleteObject { bucket, key },
    }
}
