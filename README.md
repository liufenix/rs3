# RS3: Rust S3 CLI

RS3 is a command-line interface (CLI) tool for interacting with Amazon S3 or S3-compatible storage services, written in
Rust.

## Features

- List buckets
- Create and delete buckets
- List objects in a bucket
- Upload and download objects
- Delete objects

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/rs3.git
   cd rs3
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. The binary will be available at `target/release/rs3`.

## Configuration

RS3 can be configured using a `config.toml` file or environment variables. Create a `config.toml` file in the project
root with the following content:

```toml
access_key = "your_access_key"
secret_key = "your_secret_key"
region = "your_region"
endpoint_url = "https://s3.amazonaws.com"
path_style = false
```

Alternatively, you can use environment variables:

```
export RS3_ACCESS_KEY=your_access_key
export RS3_SECRET_KEY=your_secret_key
export RS3_REGION=your_region
export RS3_ENDPOINT_URL=https://s3.amazonaws.com
export RS3_PATH_STYLE=false
```

## Usage

Here are some example commands:

```
# List buckets
rs3 list-buckets

# Create a bucket
rs3 create-bucket my-bucket

# List objects in a bucket
rs3 list-objects my-bucket

# Upload an object
rs3 upload-object my-bucket my-key /path/to/local/file

# Download an object
rs3 download-object my-bucket my-key /path/to/save/file

# Delete an object
rs3 delete-object my-bucket my-key

# Delete a bucket
rs3 delete-bucket my-bucket
```

## License

This project is licensed under the MIT License.
