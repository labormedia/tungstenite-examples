use std::error::Error;
use redis::{
    streams::{StreamRangeReply, StreamId}
    ,AsyncCommands
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await?;
    
    con.set("key1", b"foo").await?;
    
    redis::cmd("XADD").arg("mystream").arg("*").arg(&["key2", "bar"]).arg(&["key3", "foobar"]).query_async(&mut con).await?;

    redis::cmd("XADD").arg("mystream").arg("*").arg(&["key7", "flores"]).arg(&["key11", "margaritas"]).query_async(&mut con).await?;
    
    let xrange_result: Option<StreamRangeReply> = con.xrange("mystream", "-", "+").await?;
    println!("Received: {:?} ", xrange_result);
    println!("Processed: {:?} ",
        xrange_result
        .clone()
        .unwrap()
        .ids
        .iter()
        .map( |x| String::from(x.clone().id) + " " )
        .collect::<String>()
    );
    // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

    con.del("mystream").await?;

    Ok(())
}