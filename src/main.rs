mod api;
use reqwest_middleware::Error;

#[tokio::main]
async fn main() -> Result<(),Error> {
    let piped = api::PIPED::new(); 
    let instances=piped.get_instances().await?;    
    //let trending:Vec<api::VideoDetail> = piped.trending().await?;
    //let stream = piped.stream("tgB1wUcmbbw").await?;
    //let comments = piped.comments("tgB1wUcmbbw").await?;
    //let suggestion = piped.suggestion("Hello worl").await?;
    //let result = piped.login("ddd","ffff").await?;
    //let feed = piped.feed().await?;
    print!("{:?}",instances);
    //let image = piped.get_resource("https://pipedproxy-ams-2.kavin.rocks/vi/I8p1yqnuk8Y/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBxWF_5FYR5KO6FXzqI8FFT2NqBvA&host=i.ytimg.com").await?;
    //print!("{:?}",image);
    Ok(())
}
