use serde::Deserialize;

#[derive (Debug)]
pub struct PipedInstance { 
    pub name: String,
    pub url: String,
    pub cdn:bool,
    pub locations:String
}


#[derive (Debug,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetail{
    pub url: String,
    pub title: String,
    pub thumbnail: String,
    pub uploader_name: String,
    pub uploader_url: String,
    pub uploader_avatar: Option<String>,
    pub uploaded_date: String,
    pub short_description: Option<String>,
    pub duration: i64,
    pub views: i64,
    pub uploaded: i64,
    pub uploader_verified: bool
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    pub bitrate: i32, 
    pub codec: Option<String>, 
    pub format: String, 
    pub index_end: i32, 
    pub index_start: i32, 
    pub init_start: i32, 
    pub init_end: i32, 
    pub mime_type: String, 
    pub quality: String,
    pub url: String,
    pub video_only: bool
}


#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    pub bitrate: i32, 
    pub codec: Option<String>, 
    pub format: String, 
    pub index_end: i32, 
    pub index_start: i32, 
    pub init_start: i32, 
    pub init_end: i32, 
    pub mime_type: String, 
    pub quality: String,
    pub url: String,
    pub video_only: bool,
    pub width:i16,
    pub height:i16,
    pub fps:i16
}

#[derive (Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct Subtitle{
    pub auto_generated: bool, 
    pub code: String, 
    pub mime_type: String,
    pub name: String,
    pub url: String
}

#[derive (Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct VideoStreamDetail{
    pub audio_streams : Vec<AudioStream>,
    pub dash:Option<String>,
    pub description:String,
    pub dislikes:i64,
    pub duration:i64,
    pub hls:Option<String>,
    pub lbry_id:Option<String>,
    pub likes:i64,
    pub livestream:bool,
    pub proxy_url:Option<String>,
    pub related_streams:Vec<VideoDetail>,
    pub subtitles:Vec<Subtitle>,
    pub thumbnail_url:String,
    pub title:String,
    pub upload_date:String,
    pub uploader:String,
    pub uploader_url:String,
    pub uploader_verified:bool,
    pub video_streams:Vec<VideoStream>,
    pub views:i64
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub author : String,
    pub comment_id:String,
    pub comment_text:String,
    pub commented_time:String,
    pub commentor_url:String,
    pub hearted:bool,
    pub like_count:i64,
    pub pinned:bool,
    pub thumbnail: String,
    pub verified:bool
}

#[derive (Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct Comments { 
    pub comments : Vec<Comment>,
    pub disabled:bool,
    pub nextpage:Option<String>
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel{
    pub id:String,
    pub name:String,
    pub avatar_url:Option<String>,
    pub banner_url:Option<String>,
    pub nextpage:Option<String>,
    pub subscriber_count : i32,
    pub verified: bool,
    pub related_streams:Vec<VideoDetail>
}
