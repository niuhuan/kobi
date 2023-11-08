use crate::copy_client::{
    Author, ChapterAndContents, ChapterComicInfo, ChapterData, ChapterImage, ClassifyItem, Comic,
    ComicChapter, ComicData, ComicInRank, Group, LastChapter, Page, RankItem, Tag,
};
use crate::get_image_cache_dir;
use crate::utils::join_paths;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageRankItem {
    pub list: Vec<UIRankItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<Page<RankItem>> for UIPageRankItem {
    fn from(page: Page<RankItem>) -> Self {
        Self {
            list: page.list.into_iter().map(|x| UIRankItem::from(x)).collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIRankItem {
    pub comic: UIComicInRank,
    pub date_type: i64,
    pub popular: i64,
    pub rise_num: i64,
    pub rise_sort: i64,
    pub sort: i64,
    pub sort_last: i64,
}

impl From<RankItem> for UIRankItem {
    fn from(item: RankItem) -> Self {
        Self {
            comic: UIComicInRank::from(item.comic),
            date_type: item.date_type,
            popular: item.popular,
            rise_num: item.rise_num,
            rise_sort: item.rise_sort,
            sort: item.sort,
            sort_last: item.sort_last,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComicInRank {
    pub author: Vec<Author>,
    pub cover: String,
    pub img_type: i64,
    pub name: String,
    pub path_word: String,
    pub popular: i64,
}

impl From<ComicInRank> for UIComicInRank {
    fn from(comic: ComicInRank) -> Self {
        Self {
            author: comic.author,
            cover: comic.cover,
            img_type: comic.img_type,
            name: comic.name,
            path_word: comic.path_word,
            popular: comic.popular,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComicData {
    pub comic: UIComic,
    pub groups: Vec<Group>,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
    pub popular: i64,
}

impl From<ComicData> for UIComicData {
    fn from(comic: ComicData) -> Self {
        Self {
            comic: UIComic::from(comic.comic),
            groups: comic.groups.into_iter().map(|(_, v)| v).collect(),
            is_lock: comic.is_lock,
            is_login: comic.is_login,
            is_mobile_bind: comic.is_mobile_bind,
            is_vip: comic.is_vip,
            popular: comic.popular,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComic {
    pub alias: Option<String>,
    pub author: Vec<Author>,
    pub b_404: bool,
    pub b_hidden: bool,
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub cover: String,
    pub datetime_updated: String,
    pub free_type: ClassifyItem,
    pub img_type: i64,
    pub last_chapter: LastChapter,
    pub name: String,
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

impl From<Comic> for UIComic {
    fn from(comic: Comic) -> Self {
        Self {
            alias: comic.alias,
            author: comic.author,
            b_404: comic.b_404,
            b_hidden: comic.b_hidden,
            ban: comic.ban,
            brief: comic.brief,
            close_comment: comic.close_comment,
            close_roast: comic.close_roast,
            cover: comic.cover,
            datetime_updated: comic.datetime_updated,
            free_type: comic.free_type,
            img_type: comic.img_type,
            last_chapter: comic.last_chapter,
            name: comic.name,
            path_word: comic.path_word,
            popular: comic.popular,
            reclass: comic.reclass,
            region: comic.region,
            restrict: comic.restrict,
            seo_baidu: comic.seo_baidu,
            status: comic.status,
            theme: comic.theme,
            uuid: comic.uuid,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UICacheImage {
    pub abs_path: String,
    pub cache_key: String,
    pub cache_time: i64,
    pub url: String,
    pub useful: String,
    pub extends_field_first: Option<String>,
    pub extends_field_second: Option<String>,
    pub extends_field_third: Option<String>,
    pub local_path: String,
    pub image_format: String,
    pub image_width: u32,
    pub image_height: u32,
}

impl From<crate::database::cache::image_cache::Model> for UICacheImage {
    fn from(model: crate::database::cache::image_cache::Model) -> Self {
        Self {
            abs_path: join_paths(vec![
                get_image_cache_dir().as_str(),
                model.local_path.as_str(),
            ]),
            cache_key: model.cache_key,
            cache_time: model.cache_time,
            url: model.url,
            useful: model.useful,
            extends_field_first: model.extends_field_first,
            extends_field_second: model.extends_field_second,
            extends_field_third: model.extends_field_third,
            local_path: model.local_path,
            image_format: model.image_format,
            image_width: model.image_width,
            image_height: model.image_height,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageComicChapter {
    pub list: Vec<UIComicChapter>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<Page<ComicChapter>> for UIPageComicChapter {
    fn from(page: Page<ComicChapter>) -> Self {
        Self {
            list: page
                .list
                .into_iter()
                .map(|x| UIComicChapter::from(x))
                .collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComicChapter {
    pub comic_id: String,
    pub comic_path_word: String,
    pub count: i64,
    pub datetime_created: String,
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

impl From<ComicChapter> for UIComicChapter {
    fn from(chapter: ComicChapter) -> Self {
        Self {
            comic_id: chapter.comic_id,
            comic_path_word: chapter.comic_path_word,
            count: chapter.count,
            datetime_created: chapter.datetime_created,
            group_path_word: chapter.group_path_word,
            img_type: chapter.img_type,
            index: chapter.index,
            name: chapter.name,
            news: chapter.news,
            next: chapter.next,
            ordered: chapter.ordered,
            prev: chapter.prev,
            size: chapter.size,
            type_field: chapter.type_field,
            uuid: chapter.uuid,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComicQuery {
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
}

impl From<crate::copy_client::ComicQuery> for UIComicQuery {
    fn from(query: crate::copy_client::ComicQuery) -> Self {
        Self {
            is_lock: query.is_lock,
            is_login: query.is_login,
            is_mobile_bind: query.is_mobile_bind,
            is_vip: query.is_vip,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIChapterData {
    pub chapter: UIChapterAndContents,
    pub comic: ChapterComicInfo,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
    pub show_app: bool,
}

impl From<ChapterData> for UIChapterData {
    fn from(value: ChapterData) -> Self {
        Self {
            chapter: UIChapterAndContents::from(value.chapter),
            comic: value.comic,
            is_lock: value.is_lock,
            is_login: value.is_login,
            is_mobile_bind: value.is_mobile_bind,
            is_vip: value.is_vip,
            show_app: value.show_app,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIChapterAndContents {
    pub comic_id: String,
    pub comic_path_word: String,
    pub contents: Vec<ChapterImage>,
    pub count: i64,
    pub datetime_created: String,
    pub group_path_word: String,
    pub img_type: i64,
    pub index: i64,
    pub is_long: bool,
    pub name: String,
    pub news: String,
    pub next: String,
    pub ordered: i64,
    pub prev: Option<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub uuid: String,
    pub words: Vec<i64>,
}

impl From<ChapterAndContents> for UIChapterAndContents {
    fn from(value: ChapterAndContents) -> Self {
        Self {
            comic_id: value.comic_id,
            comic_path_word: value.comic_path_word,
            contents: value.contents,
            count: value.count,
            datetime_created: value.datetime_created,
            group_path_word: value.group_path_word,
            img_type: value.img_type,
            index: value.index,
            is_long: value.is_long,
            name: value.name,
            news: value.news,
            next: value.next,
            ordered: value.ordered,
            prev: value.prev,
            size: value.size,
            type_field: value.type_field,
            uuid: value.uuid,
            words: value.words,
        }
    }
}
