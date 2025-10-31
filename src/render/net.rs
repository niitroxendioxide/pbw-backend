use aws_sdk_s3::{
    error::DisplayErrorContext,
    primitives::ByteStream,
    Client,
    config::{Credentials, Region, BehaviorVersion},
};

use dotenvy::dotenv;
use std::env;
use std::path::Path;

fn get_minio_endpoint() -> String {
    dotenv().ok();
    std::env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "/minio/".to_string())
}
static REGION: &str = "sa-east-1";


pub async fn upload_to_minio(file_path: &str, image_uuid: &str, file_extension: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    
    let access_key: String = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY Required in .env");
    let secret_access_key: String = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY Required in .env");
    let bucket_name: &str = "images"; // env::var("MINIO_NAME").expect("MINIO_NAME Required in .env");
    let minio_region = Region::new(REGION);

        let minio_endpoint = get_minio_endpoint();
        let minio_config = aws_sdk_s3::config::Builder::new()
            .region(minio_region)
            .credentials_provider(Credentials::new(
                access_key,
                secret_access_key,
                None,
                None,
                "minio",
            ))
            .endpoint_url(&minio_endpoint)
            .behavior_version(BehaviorVersion::v2025_08_07())
            .force_path_style(true)
            .build();

    let minio_client = Client::from_conf(minio_config);
    match ByteStream::from_path(Path::new(file_path)).await {
        Ok(body) => {
            let image_out = format!("{}{}", image_uuid, file_extension);
        
            // Send the request and handle potential errors with more detail
            let send_result = minio_client.put_object()
                .bucket(bucket_name)
                .key(&image_out)
                .body(body)
                .send()
                .await;
        
            match send_result {
                Ok(_) => {
                        let public_url = format!("{}/{}/{}", minio_endpoint, bucket_name, image_out);
                    Ok(public_url)
                },
                Err(e) => {
                    eprintln!("Detailed upload error: {}", DisplayErrorContext(&e));
                    Err(Box::new(e))
                }
            }
        },
        Err(e) => {
            eprintln!("Detailed file read error: {}", DisplayErrorContext(&e));
            Err(Box::new(e))
        }
    }
}
