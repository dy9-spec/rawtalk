use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};

#[tokio::main]
async fn main() {
    // gain AWS credentials via env vars
    let aws_creds = EnvironmentProvider::default().credentials().await.unwrap();
    let aws_access_key = aws_creds.aws_access_key_id();
    let aws_secret_access_key = aws_creds.aws_secret_access_key();

    println!("AWS access key: {}", aws_access_key);
    println!("AWS secret access key: {}", aws_secret_access_key);
}
