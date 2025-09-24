use aws_sdk_s3::{

    primitives::ByteStream,
    Client,
    config::{Credentials, Region, BehaviorVersion},
};

use dotenvy::dotenv;
use std::env;
use std::path::Path;

static MINIO_PORT: &str = "localhost:9000";
static REGION: &str = "sa-east-1";


pub async fn upload_to_minio(file_path: &str, image_uuid: &str, file_extension: &str) ->  Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let access_key: String = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY Required in .env");
    let secret_access_key: String = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY Required in .env");
    let bucket_name: String = env::var("MINIO_NAME").expect("MINIO_NAME Required in .env");
    let minio_region = Region::new(REGION);

    let minio_config = aws_sdk_s3::config::Builder::new()
        .region(minio_region)
        .credentials_provider(Credentials::new(
            access_key,
            secret_access_key,
            None,
            None,
            "minio",
        ))
        .endpoint_url(MINIO_PORT)
        .behavior_version(BehaviorVersion::v2025_08_07())
        .force_path_style(true)
        .build();

    let minio_client = Client::from_conf(minio_config);
    let body = ByteStream::from_path(Path::new(file_path)).await?;
    let image_out = format!("uploads/{}.{}", image_uuid, file_extension);

    minio_client.put_object().bucket(&bucket_name)
        .key(&image_out)
        .body(body)
        .send()
        .await?;

    let public_url = format!("{}/{}/{}", MINIO_PORT, bucket_name, image_out);

    Ok(public_url)
}