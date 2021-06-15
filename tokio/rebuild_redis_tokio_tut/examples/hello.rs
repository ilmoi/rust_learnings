use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a conn
    let mut client = client::connect("127.0.0.1:6379").await?;
    // set
    client.set("hello", "world2222".into()).await?;
    // get
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);
    Ok(())
}