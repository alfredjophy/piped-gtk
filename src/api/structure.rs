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
    url: String,
    title: String,
    thumbnail: String,
    uploader_name: String,
    uploader_url: String,
    uploader_avatar: String,
    uploaded_date: String,
    short_description: Option<String>,
    duration: i64,
    views: i64,
    uploaded: i64,
    uploader_verified: bool
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    bitrate: i32, 
    codec: Option<String>, 
    format: String, 
    index_end: i32, 
    index_start: i32, 
    init_start: i32, 
    init_end: i32, 
    mime_type: String, 
    quality: String,
    url: String,
    video_only: bool
}


#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    bitrate: i32, 
    codec: Option<String>, 
    format: String, 
    index_end: i32, 
    index_start: i32, 
    init_start: i32, 
    init_end: i32, 
    mime_type: String, 
    quality: String,
    url: String,
    video_only: bool,
    width:i16,
    height:i16,
    fps:i16
}

#[derive (Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct Subtitle{
    auto_generated: bool, 
    code: String, 
    mime_type: String,
    name: String,
    url: String
}

#[derive (Deserialize,Debug)]
#[serde(rename_all="camelCase")]
pub struct VideoStreamDetail{
    audio_streams : Vec<AudioStream>,
    dash:Option<String>,
    description:String,
    dislikes:i64,
    duration:i64,
    hls:Option<String>,
    lbry_id:Option<String>,
    likes:i64,
    livestream:bool,
    proxy_url:Option<String>,
    related_streams:Vec<VideoDetail>,
    subtitles:Vec<Subtitle>,
    thumbnail_url:String,
    title:String,
    upload_date:String,
    uploader:String,
    uploader_url:String,
    uploader_verified:bool,
    video_streams:Vec<VideoStream>,
    views:i64
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub author : String,
    pub comment_id:String,
    pub comment_text:String,
    pub commented_time:String,
    pub commenor_url:String,
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
    id:String,
    name:String,
    avatar_url:String,
    banner_url:String,
    nextpage:Option<String>,
    subscriber_count : i32,
    verified: bool,
    related_streams:Vec<VideoDetail>
}
