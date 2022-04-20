use reqwest::Error;

use crate::api::structure::*;

use std::collections::HashMap;

pub struct API{
    api_url:String,
    region:String,
    client:reqwest::Client
}

impl API{
    pub fn new() -> API{
        API{
            api_url:"https://pipedapi.kavin.rocks".to_string(),
            region:"US".to_string(),
            client:reqwest::Client::new()
        }
    }
    
    fn create_url_from_endpoint(&self,endpoint:&str)->String{
        format!("{}/{}",self.api_url,endpoint)
    }
    pub async fn login(&self,username:&str,password:&str)->Result<AuthResponse,Error>{
        let url=self.create_url_from_endpoint("/login");
        let mut body = HashMap::new();
        body.insert("username",username);
        body.insert("password",password);
        let response = self.client.post(url).json(&body).send().await?;
        let result = response.json::<AuthResponse>().await?;
        Ok(result)
    }
    pub async fn register(&self,username:&str,password:&str)->Result<AuthResponse,Error>{
        let url=self.create_url_from_endpoint("/register");
        let mut body = HashMap::new();
        body.insert("username",username);
        body.insert("password",password);
        let response = self.client.post(url).json(&body).send().await?;
        let result = response.json::<AuthResponse>().await?;
        Ok(result)
    }

    pub async fn get_instances(&self) -> Result< Vec<PipedInstance>, Error> {
        let request_url = "https://raw.githubusercontent.com/wiki/TeamPiped/Piped-Frontend/Instances.md";
        let response = self.client.get(request_url).send().await?;
        let data=response.text().await?;
        
        let lines=data.split('\n').collect::<Vec<&str>>();
    
        fn parse_line(line:&str) -> Option<PipedInstance>{
            let split = line.split('|').collect::<Vec<&str>>();
            let _instance:PipedInstance;
            if split.len() > 4 {
                _instance = PipedInstance{
                    name: split[0].trim().to_string(),
                    url: split[1].trim().to_string(),
                    cdn: (split[3].trim()=="Yes"),
                    locations: split[2].trim().to_string()
                };
                Some(_instance)
            }
            else{
                None
            }
        }
    
        print!("{}",lines.len());
        let mut parsed_lines:Vec<PipedInstance> = Vec::new();
    
        for i in lines{
            let x = parse_line(i);
            if x.is_some(){
                parsed_lines.push(x.unwrap());
            }
        }
        Ok(parsed_lines)
    }

    pub async fn trending(&self)->Result< Vec<VideoDetail>,Error>{
        let url = self.create_url_from_endpoint("/trending");
        let response = self.client.get(url).query(&[("region",&self.region)]).send().await?;
        let data = response.json::<Vec<VideoDetail>>().await?;
        print!("{:?}",data);
        Ok(data)
    }

    pub async fn stream(&self,video_id:&str)->Result<VideoStreamDetail,Error>{
        let url = self.create_url_from_endpoint(&format!("/streams/{}",video_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<VideoStreamDetail>().await?;
        Ok(data)
        
    }

    pub async fn comments(&self,video_id:&str)->Result< Comments,Error >{
        let url = self.create_url_from_endpoint(&format!("/comments/{}",video_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<Comments>().await?;
        Ok(data)
    }

    pub async fn channel_from_id(&self,channel_id:&str) -> Result<Channel,Error>{
        let url = self.create_url_from_endpoint(&format!("/channel/{}",channel_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<Channel>().await?;
        Ok(data)
    }
    // broken 
    //pub async fn channel_from_name(&self,channel_name:&str)->Result<Channel,Error>{
        //let url = self.create_url_from_endpoint(&format!("/c/{}",channel_name));
        //let response = self.client.get(url).send().await?;
        //let data = response.json::<Channel>().await?;
        //Ok(data)
    //}
    pub async fn channel_from_username(&self,username:&str)->Result<Channel,Error>{
        let url = self.create_url_from_endpoint(&format!("/user/{}",username));
        let response = self.client.get(url).send().await?;
        let data = response.json::<Channel>().await?;
        Ok(data)
    }
    pub async fn suggestion(&self,query:&str) ->Result< Vec<String>,Error >{
        let url = self.create_url_from_endpoint("/suggestions");
        let response = self.client.get(url).query(&[("query",query)]).send().await?;
        let data = response.json::<Vec<String>>().await?;
        Ok(data)
    }

}

