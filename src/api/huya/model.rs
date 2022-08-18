// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct HuyaResp {
    pub data: Vec<Datum>,
    pub count: i64,
    #[serde(rename = "vMultiStreamInfo")]
    pub v_multi_stream_info: Vec<VMultiStreamInfo>,
    #[serde(rename = "iWebDefaultBitRate")]
    pub i_web_default_bit_rate: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "gameLiveInfo")]
    pub game_live_info: GameLiveInfo,
    #[serde(rename = "gameStreamInfoList")]
    pub game_stream_info_list: Vec<GameStreamInfoList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameLiveInfo {
    pub uid: i64,
    pub sex: i64,
    #[serde(rename = "gameFullName")]
    pub game_full_name: String,
    #[serde(rename = "gameHostName")]
    pub game_host_name: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "activityId")]
    pub activity_id: i64,
    pub level: i64,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
    #[serde(rename = "roomName")]
    pub room_name: String,
    #[serde(rename = "isSecret")]
    pub is_secret: i64,
    #[serde(rename = "cameraOpen")]
    pub camera_open: i64,
    #[serde(rename = "liveChannel")]
    pub live_channel: i64,
    #[serde(rename = "bussType")]
    pub buss_type: i64,
    pub yyid: i64,
    pub screenshot: String,
    #[serde(rename = "activityCount")]
    pub activity_count: i64,
    #[serde(rename = "privateHost")]
    pub private_host: String,
    #[serde(rename = "recommendStatus")]
    pub recommend_status: i64,
    pub nick: String,
    #[serde(rename = "shortChannel")]
    pub short_channel: i64,
    pub avatar180: String,
    pub gid: i64,
    pub channel: i64,
    pub introduction: String,
    #[serde(rename = "profileHomeHost")]
    pub profile_home_host: String,
    #[serde(rename = "liveSourceType")]
    pub live_source_type: i64,
    #[serde(rename = "screenType")]
    pub screen_type: i64,
    #[serde(rename = "bitRate")]
    pub bit_rate: i64,
    #[serde(rename = "gameType")]
    pub game_type: i64,
    #[serde(rename = "attendeeCount")]
    pub attendee_count: i64,
    #[serde(rename = "multiStreamFlag")]
    pub multi_stream_flag: i64,
    #[serde(rename = "codecType")]
    pub codec_type: i64,
    #[serde(rename = "liveCompatibleFlag")]
    pub live_compatible_flag: i64,
    #[serde(rename = "profileRoom")]
    pub profile_room: i64,
    #[serde(rename = "liveId")]
    pub live_id: f64,
    #[serde(rename = "recommendTagName")]
    pub recommend_tag_name: String,
    #[serde(rename = "contentIntro")]
    pub content_intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStreamInfoList {
    #[serde(rename = "sCdnType")]
    pub s_cdn_type: String,
    #[serde(rename = "iIsMaster")]
    pub i_is_master: i64,
    #[serde(rename = "lChannelId")]
    pub l_channel_id: i64,
    #[serde(rename = "lSubChannelId")]
    pub l_sub_channel_id: i64,
    #[serde(rename = "lPresenterUid")]
    pub l_presenter_uid: i64,
    #[serde(rename = "sStreamName")]
    pub s_stream_name: String,
    #[serde(rename = "sFlvUrl")]
    pub s_flv_url: String,
    #[serde(rename = "sFlvUrlSuffix")]
    pub s_flv_url_suffix: String,
    #[serde(rename = "sFlvAntiCode")]
    pub s_flv_anti_code: String,
    #[serde(rename = "sHlsUrl")]
    pub s_hls_url: String,
    #[serde(rename = "sHlsUrlSuffix")]
    pub s_hls_url_suffix: String,
    #[serde(rename = "sHlsAntiCode")]
    pub s_hls_anti_code: String,
    #[serde(rename = "iLineIndex")]
    pub i_line_index: i64,
    #[serde(rename = "iIsMultiStream")]
    pub i_is_multi_stream: i64,
    #[serde(rename = "iPCPriorityRate")]
    pub i_pc_priority_rate: i64,
    #[serde(rename = "iWebPriorityRate")]
    pub i_web_priority_rate: i64,
    #[serde(rename = "iMobilePriorityRate")]
    pub i_mobile_priority_rate: i64,
    #[serde(rename = "vFlvIPList")]
    pub v_flv_ip_list: IpList,
    #[serde(rename = "iIsP2PSupport")]
    pub i_is_p2_p_support: i64,
    #[serde(rename = "sP2pUrl")]
    pub s_p2_p_url: String,
    #[serde(rename = "sP2pUrlSuffix")]
    pub s_p2_p_url_suffix: String,
    #[serde(rename = "sP2pAntiCode")]
    pub s_p2_p_anti_code: String,
    #[serde(rename = "lFreeFlag")]
    pub l_free_flag: i64,
    #[serde(rename = "iIsHEVCSupport")]
    pub i_is_hevc_support: i64,
    #[serde(rename = "vP2pIPList")]
    pub v_p2_p_ip_list: IpList,
    #[serde(rename = "mpExtArgs")]
    pub mp_ext_args: MpExtArgs,
    #[serde(rename = "lTimespan")]
    pub l_timespan: i64,
    #[serde(rename = "_classname")]
    pub classname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpExtArgs {
    #[serde(rename = "_kproto")]
    pub kproto: Proto,
    #[serde(rename = "_vproto")]
    pub vproto: Proto,
    #[serde(rename = "_bKey")]
    pub b_key: i64,
    #[serde(rename = "_bValue")]
    pub b_value: i64,
    pub value: Value,
    #[serde(rename = "_classname")]
    pub classname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proto {
    #[serde(rename = "_classname")]
    pub classname: KprotoClassname,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpList {
    #[serde(rename = "_proto")]
    pub proto: Proto,
    #[serde(rename = "_bValue")]
    pub b_value: i64,
    pub value: Vec<Option<serde_json::Value>>,
    #[serde(rename = "_classname")]
    pub classname: VFlvIpListClassname,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMultiStreamInfo {
    #[serde(rename = "sDisplayName")]
    pub s_display_name: String,
    #[serde(rename = "iBitRate")]
    pub i_bit_rate: i64,
    #[serde(rename = "iCodecType")]
    pub i_codec_type: i64,
    #[serde(rename = "iCompatibleFlag")]
    pub i_compatible_flag: i64,
    #[serde(rename = "iHEVCBitRate")]
    pub i_hevc_bit_rate: i64,
    #[serde(rename = "_classname")]
    pub classname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KprotoClassname {
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VFlvIpListClassname {
    #[serde(rename = "list<string>")]
    ListString,
}
