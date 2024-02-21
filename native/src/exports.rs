
use crate::{database::download::{download_comic, download_comic_chapter, download_comic_page}, downloading::{get_cover_path, get_image_path}, udto::ExportsType, utils::{allowed_file_name, create_dir_if_not_exists, join_paths}};
use anyhow::{Ok, Result};

pub(crate) async fn exports(uuid_list: Vec<String>, export_to_folder: String, exports_type: ExportsType) -> Result<()> {
    let download_comics = download_comic::find_by_uuid_list(uuid_list.as_slice()).await?;
    for ele in &download_comics {
        if ele.download_status != download_comic::STATUS_DOWNLOAD_SUCCESS {
            return Err(anyhow::anyhow!("comic not downloaded"));
        }
    }
    for download_comic in &download_comics {
        let name = allowed_file_name(download_comic.name.as_str());
        let chapters = download_comic_chapter::find_by_comic_path_word(download_comic.path_word.as_str()).await?;
        let comic_folder = join_paths(vec![
            export_to_folder.as_str(),
            name.as_str(),
        ]);
        create_dir_if_not_exists(comic_folder.as_str());
        // copy cover
        let cover_path = join_paths(vec![
            comic_folder.as_str(),
           format!("cover.{}", download_comic.cover_format).as_str(),
        ]);
        let download_cover_path = get_cover_path(download_comic);
        tokio::fs::copy(download_cover_path.as_str(), cover_path.as_str()).await?;
        for chapter in &chapters {
            let chapter_folder = join_paths(vec![
                comic_folder.as_str(),
                chapter.name.as_str(),
            ]);
            create_dir_if_not_exists(chapter_folder.as_str());
            let pages = download_comic_page::find_by_comic_path_word_and_chapter_uuid(download_comic.path_word.as_str(), chapter.uuid.as_str()).await?;
            for page in &pages {
                let page_path = join_paths(vec![
                    chapter_folder.as_str(),
                    format!("{}.{}", page.image_index, page.format).as_str(),
                ]);
                let download_comic_path = get_image_path(&page);
                tokio::fs::copy(download_comic_path.as_str(), page_path.as_str()).await?;
            }
        }
    }
    Ok(())
}