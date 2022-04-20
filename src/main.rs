use reqwest::Error;
mod api;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let data=api::get_instances().await?;    
    let piped = api::API::new(); 
    //let trending:Vec<api::VideoDetail> = piped.trending().await?;
    //let stream = piped.stream("tgB1wUcmbbw").await?;
    //let comments = piped.comments("tgB1wUcmbbw").await?;
    //let suggestion = piped.suggestion("Hello worl").await?;
    //let result = piped.login("ddd","ffff").await?;
    let feed = piped.feed().await?;
    print!("{:?}",feed);
    Ok(())
}
