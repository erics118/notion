
use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()>{
let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
println!("body = {:?}", body);
Ok(())
}