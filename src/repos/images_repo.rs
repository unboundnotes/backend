use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use log::info;
use s3::{creds::Credentials, Bucket, Region};

use super::traits::ImagesRepo;

pub struct S3ImagesRepo {
    base_path: String,
    bucket: Bucket,
}

impl S3ImagesRepo {
    pub fn new(bucket_name: &String, endpoint: &String) -> Result<Self> {
        let mut region = Region::from_str(endpoint)?;
        if let Region::Custom { ref endpoint, .. } = region {
            region = Region::Custom {
                region: "custom".to_string(),
                endpoint: endpoint.clone(),
            };
        }
        let bucket = Bucket::new(bucket_name, region, Credentials::default()?)?.with_path_style();
        Ok(Self {
            base_path: bucket.url(),
            bucket,
        })
    }
}

#[async_trait]
impl ImagesRepo for S3ImagesRepo {
    async fn upload_image(&self, path: &str, image: &[u8]) -> Result<String> {
        self.bucket.put_object(path, image).await?;
        Ok(format!("{}/{}", self.base_path, path))
    }

    async fn delete_image(&self, path: &str) -> Result<()> {
        let path = path.split(&format!("{}/", self.base_path)).last().unwrap();
        info!("Deleting image: {}", path);
        self.bucket.delete_object(path).await?;
        Ok(())
    }
}
