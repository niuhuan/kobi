use linked_hash_map::LinkedHashMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    pub comic: ComicInRank,
    pub date_type: i64,
    pub popular: i64,
    pub rise_num: i64,
    pub rise_sort: i64,
    pub sort: i64,
    pub sort_last: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInRank {
    pub author: Vec<Author>,
    pub cover: String,
    pub females: Vec<Value>,
    pub img_type: i64,
    pub males: Vec<Value>,
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
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub clubs: Vec<Value>,
    pub cover: String,
    pub datetime_updated: String,
    pub females: Vec<Value>,
    pub free_type: FreeType,
    pub img_type: i64,
    pub last_chapter: LastChapter,
    pub males: Vec<Value>,
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
pub struct FreeType {
    pub display: String,
    pub value: i64,
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
    pub browse: Value,
    pub collect: Value,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
}
