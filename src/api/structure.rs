use serde::Deserialize;

#[derive (Debug)]
pub struct PipedInstance { 
    pub name: String,
    pub url: String,
    pub cdn:bool,
    pub locations:String
}

#[derive (Debug,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthResponse{
    Token(String),
    Error(String)
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
    pub uploaded_date: Option<String>,
    pub short_description: Option<String>,
    pub duration: i64,
    pub views: i64,
    pub uploaded: i64,
    pub uploader_verified: bool
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    pub bitrate: i64, 
    pub codec: Option<String>, 
    pub format: String, 
    pub index_end: i64, 
    pub index_start: i64, 
    pub init_start: i64, 
    pub init_end: i64, 
    pub mime_type: String, 
    pub quality: String,
    pub url: String,
    pub video_only: bool
}

#[derive (Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    pub bitrate: i64, 
    pub codec: Option<String>, 
    pub format: String, 
    pub index_end: i64, 
    pub index_start: i64, 
    pub init_start: i64, 
    pub init_end: i64, 
    pub mime_type: String, 
    pub quality: String,
    pub url: String,
    pub video_only: bool,
    pub width:i64,
    pub height:i64,
    pub fps:i64
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
    pub subscriber_count : i64,
    pub verified: bool,
    pub related_streams:Vec<VideoDetail>
}


#[derive (Debug,Deserialize)]
#[serde(untagged)]
pub enum FeedResponse{
    Feed(Vec<VideoDetail>),
    Error { error: String }
}

#[derive (Debug,Deserialize)]
#[serde(untagged)]
pub enum VideoStreamResponse{
    Stream(VideoStream),
    Error {error: String}
}

#[derive (Debug,Deserialize)]
#[serde(untagged)]
pub enum ChannelResponse{
    Stream(Channel),
    Error {error: String}
}

#[derive(Debug, Clone, Copy)]
pub enum SearchFilters{
    All,
    Videos,
    Channels,
    Playlists,
    MusicSongs,
    MusicVideos,
    MusicAlbums,
    MusicPlaylists
}

impl SearchFilters {
	pub fn to_string(&self) -> &str {
		match *self {
            SearchFilters::All             => "all",
            SearchFilters::Videos          => "videos",
            SearchFilters::Channels        => "channels",
            SearchFilters::Playlists       => "playlists",
            SearchFilters::MusicSongs      => "music_songs",
            SearchFilters::MusicVideos     => "music_videos",
            SearchFilters::MusicAlbums     => "music_albums",
            SearchFilters::MusicPlaylists  => "music_playlists"
		}
	}
}

#[derive (Debug,Deserialize)]
pub struct SearchResponse{
    pub items:Vec<VideoDetail>,
    pub nextpage:String,
    pub suggestions:Option<String>,
    pub corrected:bool

}
