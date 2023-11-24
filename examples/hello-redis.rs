use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //Open a Connection to the mini-redis Address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    //Set the key "Hello" with value "world"
    client.set("hello", "world".into()).await?;

    //Get key "hello"
    let result = client.get("hello").await?;

    println!("Got Value from the server; result= {:?}", result);

    Ok(())
}
