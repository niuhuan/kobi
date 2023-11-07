use crate::copy_client::{Author, ComicInRank, Page, RankItem};
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
