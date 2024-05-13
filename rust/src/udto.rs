use crate::copy_client::{
    Author, Browse, ChapterAndContents, ChapterComicInfo, ChapterData, ChapterImage, ClassifyItem,
    CollectedComic, Comic, ComicChapter, ComicData, ComicInExplore, ComicInList, ComicInSearch,
    Group, LastChapter, MemberInfo, Page, RankItem, RecommendItem, RegisterResult,
    SexualOrientation, Tag,
};
use crate::database::active::comic_view_log;
use crate::database::download::download_comic_page::Model;
use crate::utils::join_paths;
use crate::{downloading, get_image_cache_dir};
use serde_derive::{Deserialize, Serialize};

//////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIViewLog {
    pub comic_path_word: String,
    pub comic_name: String,
    pub comic_authors: String,
    pub comic_cover: String,
    pub chapter_uuid: String,
    pub chapter_name: String,
    pub chapter_ordered: i64,
    pub chapter_size: i64,
    pub chapter_count: i64,
    pub page_rank: i32,
    pub view_time: i64,
}

impl From<comic_view_log::Model> for UIViewLog {
    fn from(model: comic_view_log::Model) -> Self {
        Self {
            comic_path_word: model.comic_path_word,
            comic_name: model.comic_name,
            comic_authors: model.comic_authors,
            comic_cover: model.comic_cover,
            chapter_uuid: model.chapter_uuid,
            chapter_name: model.chapter_name,
            chapter_ordered: model.chapter_ordered,
            chapter_size: model.chapter_size,
            chapter_count: model.chapter_count,
            page_rank: model.page_rank,
            view_time: model.view_time,
        }
    }
}

//////////////////////////////////////

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
    pub comic: UIComicInList,
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
            comic: UIComicInList::from(item.comic),
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
pub struct UIComicInList {
    pub author: Vec<Author>,
    pub cover: String,
    pub img_type: i64,
    pub name: String,
    pub path_word: String,
    pub popular: i64,
    pub females: Vec<SexualOrientation>,
    pub males: Vec<SexualOrientation>,
}

impl From<ComicInList> for UIComicInList {
    fn from(comic: ComicInList) -> Self {
        Self {
            author: comic.author,
            cover: comic.cover,
            img_type: comic.img_type,
            name: comic.name,
            path_word: comic.path_word,
            popular: comic.popular,
            males: comic.males,
            females: comic.females,
        }
    }
}

impl From<ComicInSearch> for UIComicInList {
    fn from(comic: ComicInSearch) -> Self {
        Self {
            author: comic.author,
            cover: comic.cover,
            img_type: comic.img_type,
            name: comic.name,
            path_word: comic.path_word,
            popular: comic.popular,
            males: vec![],
            females: vec![],
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
    pub females: Vec<SexualOrientation>,
    pub males: Vec<SexualOrientation>,
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
            females: comic.females,
            males: comic.males,
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

impl From<crate::database::download::download_comic_page::Model> for UICacheImage {
    fn from(model: Model) -> Self {
        Self {
            abs_path: downloading::get_image_path(&model),
            cache_key: model.cache_key,
            cache_time: 0,
            url: model.url,
            useful: "".to_string(),
            extends_field_first: None,
            extends_field_second: None,
            extends_field_third: None,
            local_path: "".to_string(),
            image_format: model.format,
            image_width: model.width,
            image_height: model.height,
        }
    }
}

// form cover
impl From<crate::database::download::download_comic::Model> for UICacheImage {
    fn from(model: crate::database::download::download_comic::Model) -> Self {
        Self {
            abs_path: downloading::get_cover_path(&model),
            cache_key: model.cover_cache_key,
            cache_time: 0,
            url: model.cover,
            useful: "COMIC_COVER".to_string(),
            extends_field_first: None,
            extends_field_second: None,
            extends_field_third: None,
            local_path: "cover".to_string(),
            image_format: model.cover_format,
            image_width: model.cover_width,
            image_height: model.cover_height,
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
    pub browse: Option<Browse>,
    pub collect: Option<i64>,
    pub is_lock: bool,
    pub is_login: bool,
    pub is_mobile_bind: bool,
    pub is_vip: bool,
}

impl From<crate::copy_client::ComicQuery> for UIComicQuery {
    fn from(query: crate::copy_client::ComicQuery) -> Self {
        Self {
            browse: query.browse,
            collect: query.collect,
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
    pub next: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageUIComicInList {
    pub list: Vec<UIComicInList>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<Page<RecommendItem>> for UIPageUIComicInList {
    fn from(page: Page<RecommendItem>) -> Self {
        Self {
            list: page
                .list
                .into_iter()
                .map(|x| UIComicInList::from(x))
                .collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

impl From<RecommendItem> for UIComicInList {
    fn from(comic: RecommendItem) -> Self {
        let comic = comic.comic;
        Self {
            author: comic.author,
            cover: comic.cover,
            img_type: comic.img_type,
            name: comic.name,
            path_word: comic.path_word,
            popular: comic.popular,
            males: comic.males,
            females: comic.females,
        }
    }
}

impl From<Page<ComicInSearch>> for UIPageUIComicInList {
    fn from(page: Page<ComicInSearch>) -> Self {
        Self {
            list: page
                .list
                .into_iter()
                .map(|x| UIComicInList::from(x))
                .collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UITags {
    pub ordering: Vec<Tag>,
    pub theme: Vec<UITheme>,
    pub top: Vec<Tag>,
}

impl From<crate::copy_client::Tags> for UITags {
    fn from(tags: crate::copy_client::Tags) -> Self {
        Self {
            ordering: tags.ordering,
            theme: tags.theme.into_iter().map(|x| UITheme::from(x)).collect(),
            top: tags.top,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UITheme {
    pub count: i64,
    pub initials: i64,
    pub name: String,
    pub path_word: String,
}

impl From<crate::copy_client::Theme> for UITheme {
    fn from(theme: crate::copy_client::Theme) -> Self {
        Self {
            count: theme.count,
            initials: theme.initials,
            name: theme.name,
            path_word: theme.path_word,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageComicInExplore {
    pub list: Vec<UIComicInExplore>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<Page<ComicInExplore>> for UIPageComicInExplore {
    fn from(page: Page<ComicInExplore>) -> Self {
        Self {
            list: page
                .list
                .into_iter()
                .map(|x| UIComicInExplore::from(x))
                .collect(),
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIComicInExplore {
    pub name: String,
    pub path_word: String,
    pub free_type: ClassifyItem,
    pub author: Vec<Author>,
    pub cover: String,
    pub popular: i64,
    pub datetime_updated: Option<String>,
    pub females: Vec<SexualOrientation>,
    pub males: Vec<SexualOrientation>,
}

impl From<ComicInExplore> for UIComicInExplore {
    fn from(comic: ComicInExplore) -> Self {
        Self {
            name: comic.name,
            path_word: comic.path_word,
            free_type: comic.free_type,
            author: comic.author,
            cover: comic.cover,
            popular: comic.popular,
            datetime_updated: comic.datetime_updated,
            females: comic.females,
            males: comic.males,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageUIViewLog {
    pub list: Vec<UIViewLog>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIQueryDownloadComic {
    pub path_word: String,
    pub alias: Option<String>,
    pub author: String,
    pub b_404: bool,
    pub b_hidden: bool,
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub cover: String,
    pub datetime_updated: String,
    pub females: String,
    pub free_type: String,
    pub img_type: i64,
    pub males: String,
    pub name: String,
    pub popular: i64,
    pub reclass: String,
    pub region: String,
    pub restrict1: String,
    pub seo_baidu: String,
    pub status: String,
    pub theme: String,
    pub uuid: String,
    pub groups: Vec<UIQueryDownloadComicGroup>,
    pub chapters: Vec<UIQueryDownloadComicChapter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIQueryDownloadComicGroup {
    pub comic_path_word: String,
    pub group_path_word: String,
    pub count: i64,
    pub name: String,
    pub group_rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIQueryDownloadComicChapter {
    pub comic_path_word: String,
    pub uuid: String,
    pub comic_id: String,
    pub count: i64,
    pub datetime_created: String,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIDownloadComic {
    pub path_word: String,
    pub alias: Option<String>,
    pub author: String,
    pub b_404: bool,
    pub b_hidden: bool,
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub cover: String,
    pub datetime_updated: String,
    pub females: String,
    pub free_type: String,
    pub img_type: i64,
    pub males: String,
    pub name: String,
    pub popular: i64,
    pub reclass: String,
    pub region: String,
    pub restrict: String,
    pub seo_baidu: String,
    pub status: String,
    pub theme: String,
    pub uuid: String,
    pub cover_cache_key: String,
    pub cover_download_status: i64,
    pub cover_format: String,
    pub cover_width: u32,
    pub cover_height: u32,
    pub image_count: i64,
    pub image_count_success: i64,
    pub download_status: i64,
}

impl From<crate::database::download::download_comic::Model> for UIDownloadComic {
    fn from(model: crate::database::download::download_comic::Model) -> Self {
        Self {
            path_word: model.path_word,
            alias: model.alias,
            author: model.author,
            b_404: model.b_404,
            b_hidden: model.b_hidden,
            ban: model.ban,
            brief: model.brief,
            close_comment: model.close_comment,
            close_roast: model.close_roast,
            cover: model.cover,
            datetime_updated: model.datetime_updated,
            females: model.females,
            free_type: model.free_type,
            img_type: model.img_type,
            males: model.males,
            name: model.name,
            popular: model.popular,
            reclass: model.reclass,
            region: model.region,
            restrict: model.restrict,
            seo_baidu: model.seo_baidu,
            status: model.status,
            theme: model.theme,
            uuid: model.uuid,
            cover_cache_key: model.cover_cache_key,
            cover_download_status: model.cover_download_status,
            cover_format: model.cover_format,
            cover_width: model.cover_width,
            cover_height: model.cover_height,
            image_count: model.image_count,
            image_count_success: model.image_count_success,
            download_status: model.download_status,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UILoginState {
    pub state: i64, // 0:NO_SET , 1:LOGIN_SUCCESS , 2:LOGIN_FAIL
    pub message: String,
    pub member: Option<MemberInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIRegisterResult {
    pub state: i64, //, 1:SUCCESS , 2:FAIL
    pub message: String,
    pub member: Option<RegisterResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UILocalCollect {
    pub path_word: String,
    pub alias: Option<String>,
    pub author: String,
    pub b_404: bool,
    pub b_hidden: bool,
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub cover: String,
    pub datetime_updated: String,
    pub females: String,
    pub free_type: String,
    pub img_type: i64,
    pub males: String,
    pub name: String,
    pub popular: i64,
    pub reclass: String,
    pub region: String,
    pub restrict: String,
    pub seo_baidu: String,
    pub status: String,
    pub theme: String,
    pub uuid: String,
    pub append_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIDownloadComicGroup {
    pub comic_path_word: String,
    pub group_path_word: String,
    pub count: i64,
    pub name: String,
    pub group_rank: i64,
}

impl From<crate::database::download::download_comic_group::Model> for UIDownloadComicGroup {
    fn from(model: crate::database::download::download_comic_group::Model) -> Self {
        Self {
            comic_path_word: model.comic_path_word,
            group_path_word: model.group_path_word,
            count: model.count,
            name: model.name,
            group_rank: model.group_rank,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIDownloadComicChapter {
    pub comic_path_word: String,
    pub uuid: String,
    pub comic_id: String,
    pub count: i64,
    pub datetime_created: String,
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
    pub download_status: i64,
}

impl From<crate::database::download::download_comic_chapter::Model> for UIDownloadComicChapter {
    fn from(model: crate::database::download::download_comic_chapter::Model) -> Self {
        Self {
            comic_path_word: model.comic_path_word,
            uuid: model.uuid,
            comic_id: model.comic_id,
            count: model.count,
            datetime_created: model.datetime_created,
            group_path_word: model.group_path_word,
            img_type: model.img_type,
            index: model.index,
            is_long: model.is_long,
            name: model.name,
            news: model.news,
            next: model.next,
            ordered: model.ordered,
            prev: model.prev,
            size: model.size,
            type_field: model.type_field,
            download_status: model.download_status,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIDownloadComicPage {
    pub comic_path_word: String,
    pub chapter_uuid: String,
    pub image_index: i32,
    pub url: String,
    pub cache_key: String,
    pub download_status: i64,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

impl From<crate::database::download::download_comic_page::Model> for UIDownloadComicPage {
    fn from(model: crate::database::download::download_comic_page::Model) -> Self {
        Self {
            comic_path_word: model.comic_path_word,
            chapter_uuid: model.chapter_uuid,
            image_index: model.image_index,
            url: model.url,
            cache_key: model.cache_key,
            download_status: model.download_status,
            width: model.width,
            height: model.height,
            format: model.format,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIPageCollectedComic {
    pub list: Vec<CollectedComic>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<Page<CollectedComic>> for UIPageCollectedComic {
    fn from(page: Page<CollectedComic>) -> Self {
        Self {
            list: page.list,
            total: page.total,
            limit: page.limit,
            offset: page.offset,
        }
    }
}

pub enum ExportsType {
    Folder,
    Zip,
}