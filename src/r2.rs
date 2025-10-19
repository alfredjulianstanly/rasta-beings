use aws_sdk_s3::{Client, Config};
use aws_credential_types::Credentials;
use std::env;

#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket_name: String,
    public_url: String,
}

impl R2Client {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let access_key = env::var("R2_ACCESS_KEY_ID")?;
        let secret_key = env::var("R2_SECRET_ACCESS_KEY")?;
        let endpoint = env::var("R2_ENDPOINT")?;
        let bucket_name = env::var("R2_BUCKET_NAME")?;
        let public_url = env::var("R2_PUBLIC_URL")?;
        
        println!("🔑 Initializing R2 client...");
        println!("  Endpoint: {}", endpoint);
        println!("  Bucket: {}", bucket_name);
        
        let credentials = Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "r2-credentials",
        );
        
        let config = Config::builder()
            .credentials_provider(credentials)
            .endpoint_url(endpoint)
            .region(aws_sdk_s3::config::Region::new("auto"))
            .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
            .build();
        
        let client = Client::from_conf(config);
        
        println!("✅ R2 client initialized!");
        
        Ok(Self {
            client,
            bucket_name,
            public_url,
        })
    }
    
    pub async fn upload_image(
        &self,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let file_name = format!("products/{}.jpg", uuid::Uuid::new_v4());
        
        println!("📤 Uploading {} bytes to R2: {}", data.len(), file_name);
        
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&file_name)
            .body(data.into())
            .content_type(content_type)
            .send()
            .await?;
        
        let public_url = format!("{}/{}", self.public_url, file_name);
        println!("✅ Upload successful: {}", public_url);
        
        Ok(public_url)
    }
    
    pub async fn delete_image(&self, url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(key) = url.strip_prefix(&format!("{}/", self.public_url)) {
            println!("🗑️ Deleting from R2: {}", key);
            self.client
                .delete_object()
                .bucket(&self.bucket_name)
                .key(key)
                .send()
                .await?;
            println!("✅ Deleted from R2");
        }
        Ok(())
    }
}
