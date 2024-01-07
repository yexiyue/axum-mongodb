use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    axum_example::start().await?;
    Ok(())
}