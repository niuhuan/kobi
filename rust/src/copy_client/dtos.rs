use linked_hash_map::LinkedHashMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub results: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    pub ordering: Vec<Tag>,
    pub theme: Vec<Theme>,
    pub top: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub color_h5: Value,
    pub count: i64,
    pub initials: i64,
    pub logo: Value,
    pub name: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInSearch {
    pub name: String,
    pub alias: Option<String>,
    pub path_word: String,
    pub cover: String,
    pub ban: i64,
    pub img_type: i64,
    pub author: Vec<Author>,
    pub popular: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub alias: Option<String>,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankItem {
    pub comic: ComicInList,
    pub date_type: i64,
    pub popular: i64,
    pub rise_num: i64,
    pub rise_sort: i64,
    pub sort: i64,
    pub sort_last: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInList {
    pub author: Vec<Author>,
    pub cover: String,
    pub females: Vec<SexualOrientation>,
    pub img_type: i64,
    pub males: Vec<SexualOrientation>,
    pub name: String,
    pub path_word: String,
    pub popular: i64,
    pub theme: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicData {
    pub comic: Comic,
    pub groups: LinkedHashMap<String, Group>,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
    pub popular: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comic {
    pub alias: Option<String>,
    pub author: Vec<Author>,
    pub b_404: bool,
    pub b_hidden: bool,
    #[serde(default)]
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub clubs: Vec<Value>,
    pub cover: String,
    pub datetime_updated: String,
    pub females: Vec<SexualOrientation>,
    pub free_type: ClassifyItem,
    pub img_type: i64,
    pub last_chapter: LastChapter,
    pub males: Vec<SexualOrientation>,
    pub name: String,
    pub parodies: Vec<Value>,
    pub path_word: String,
    pub popular: i64,
    pub reclass: ClassifyItem,
    pub region: ClassifyItem,
    pub restrict: ClassifyItem,
    pub seo_baidu: String,
    pub status: ClassifyItem,
    pub theme: Vec<Tag>,
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastChapter {
    pub name: String,
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassifyItem {
    pub display: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub count: i64,
    pub name: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicChapter {
    pub comic_id: String,
    pub comic_path_word: String,
    pub count: i64,
    pub datetime_created: String,
    pub group_id: Value,
    pub group_path_word: String,
    pub img_type: i64,
    pub index: i64,
    pub name: String,
    pub news: String,
    pub next: Option<String>,
    pub ordered: i64,
    pub prev: Option<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicQuery {
    pub browse: Option<Browse>,
    pub collect: Option<i64>,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Browse {
    pub chapter_id: String,
    pub chapter_name: String,
    pub chapter_uuid: String,
    pub comic_id: String,
    pub comic_uuid: String,
    pub path_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterData {
    pub chapter: ChapterAndContents,
    pub comic: ChapterComicInfo,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
    pub show_app: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterAndContents {
    pub comic_id: String,
    pub comic_path_word: String,
    pub contents: Vec<ChapterImage>,
    pub count: i64,
    pub datetime_created: String,
    pub group_id: Value,
    pub group_path_word: String,
    pub img_type: i64,
    pub index: i64,
    pub is_long: bool,
    pub name: String,
    pub news: String,
    pub next: Option<String>,
    pub ordered: i64,
    pub prev: Option<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub uuid: String,
    pub words: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterImage {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterComicInfo {
    pub name: String,
    pub path_word: String,
    pub restrict: ClassifyItem,
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecommendItem {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub comic: ComicInList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInExplore {
    pub name: String,
    pub path_word: String,
    pub free_type: ClassifyItem,
    pub females: Vec<SexualOrientation>,
    pub males: Vec<SexualOrientation>,
    pub author: Vec<Author>,
    pub theme: Vec<Value>,
    pub cover: String,
    pub popular: i64,
    pub datetime_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SexualOrientation {
    pub name: String,
    pub path_word: String,
    pub gender: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterResult {
    pub user_id: String,
    pub uuid: String,
    pub datetime_created: String,
    pub token: Option<String>,
    pub nickname: String,
    pub avatar: String,
    pub invite_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    #[serde(default)]
    pub is_authenticated: bool,
    pub datetime_created: String,
    #[serde(default)]
    pub b_verify_email: bool,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub mobile_region: Option<String>,
    #[serde(default)]
    pub point: i64,
    #[serde(default)]
    pub comic_vip: i64,
    pub comic_vip_end: Option<String>,
    pub comic_vip_start: Option<String>,
    #[serde(default)]
    pub cartoon_vip: i64,
    pub cartoon_vip_end: Option<String>,
    pub cartoon_vip_start: Option<String>,
    pub ads_vip_end: Option<String>,
    #[serde(default)]
    pub close_report: bool,
    #[serde(default)]
    pub downloads: i64,
    #[serde(default)]
    pub vip_downloads: i64,
    #[serde(default)]
    pub reward_downloads: i64,
    pub invite_code: Option<String>,
    pub invited: Option<String>,
    #[serde(default)]
    pub b_sstv: bool,
    #[serde(default)]
    pub scy_answer: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberInfo {
    pub user_id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    #[serde(default)]
    pub is_authenticated: bool,
    pub datetime_created: String,
    #[serde(default)]
    pub b_verify_email: bool,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub mobile_region: Option<String>,
    #[serde(default)]
    pub point: i64,
    #[serde(default)]
    pub comic_vip: i64,
    pub comic_vip_end: Option<String>,
    pub comic_vip_start: Option<String>,
    #[serde(default)]
    pub cartoon_vip: i64,
    pub cartoon_vip_end: Option<String>,
    pub cartoon_vip_start: Option<String>,
    pub ads_vip_end: Option<String>,
    #[serde(default)]
    pub close_report: bool,
    #[serde(default)]
    pub downloads: i64,
    #[serde(default)]
    pub vip_downloads: i64,
    #[serde(default)]
    pub reward_downloads: i64,
    pub invite_code: Option<String>,
    pub invited: Option<String>,
    #[serde(default)]
    pub b_sstv: bool,
    #[serde(default)]
    pub scy_answer: bool,
    pub day_downloads_refresh: String,
    #[serde(default)]
    pub day_downloads: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectedComic {
    pub uuid: i64,
    pub name: Option<String>,
    pub b_folder: bool,
    pub folder_id: Option<String>,
    pub last_browse: Option<LastBrowse>,
    pub comic: CollectedComicInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastBrowse {
    pub last_browse_id: String,
    pub last_browse_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectedComicInfo {
    pub uuid: String,
    pub b_display: bool,
    pub name: String,
    pub path_word: String,
    pub females: Vec<SexualOrientation>,
    pub males: Vec<SexualOrientation>,
    pub author: Vec<Author>,
    pub theme: Vec<Tag>,
    pub cover: String,
    pub status: i64,
    pub popular: i64,
    pub datetime_updated: String,
    pub last_chapter_id: String,
    pub last_chapter_name: String,
}
