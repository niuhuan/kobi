use crate::{
    database::download::{download_comic, download_comic_chapter, download_comic_page},
    downloading::{get_cover_path, get_image_path},
    udto::ExportsType,
    utils::{allowed_file_name, create_dir_if_not_exists, join_paths},
};
use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use async_zip::{tokio::write::ZipFileWriter, ZipEntryBuilder};
use futures_util::{lock::Mutex, AsyncWriteExt};
use tokio::io::AsyncReadExt;

pub(crate) async fn exports(
    uuid_list: Vec<String>,
    export_to_folder: String,
    exports_type: ExportsType,
) -> Result<()> {
    let download_comics = download_comic::find_by_uuid_list(uuid_list.as_slice()).await?;
    for ele in &download_comics {
        if ele.download_status != download_comic::STATUS_DOWNLOAD_SUCCESS {
            return Err(anyhow::anyhow!("comic not downloaded"));
        }
    }
    for download_comic in &download_comics {
        let name = download_comic.name.as_str();
        let mut exporter = match exports_type {
            ExportsType::Folder => FolderExporter::on_start(&export_to_folder, &name).await?,
            ExportsType::Zip => ZipExporter::on_start(&export_to_folder, &name).await?,
        };
        let chapters =
            download_comic_chapter::find_by_comic_path_word(download_comic.path_word.as_str())
                .await?;
        let download_cover_path = get_cover_path(download_comic);
        exporter
            .on_cover(
                download_cover_path.as_str(),
                download_comic.cover_format.as_str(),
            )
            .await?;
        for chapter in &chapters {
            exporter.on_chapter(&chapter.name).await?;
            let pages = download_comic_page::find_by_comic_path_word_and_chapter_uuid(
                download_comic.path_word.as_str(),
                chapter.uuid.as_str(),
            )
            .await?;
            for page in &pages {
                let download_comic_path = get_image_path(&page);
                exporter
                    .on_page(&download_comic_path, &page.format, page.image_index)
                    .await?;
            }
        }
        exporter.finish().await?;
    }
    Ok(())
}

#[async_trait]
trait Exporter {
    async fn on_cover(&mut self, source: &str, format: &str) -> Result<()>;
    async fn on_chapter(&mut self, name: &str) -> Result<()>;
    async fn on_page(&mut self, source: &str, format: &str, index: i32) -> Result<()>;
    async fn finish(mut self: Box<Self>) -> Result<()>;
}

struct FolderExporter {
    comic_folder: String,
    chaper_folder: Mutex<Option<String>>,
}

impl FolderExporter {
    async fn on_start(export_to_folder: &str, name: &str) -> Result<Box<dyn Exporter>> {
        let comic_folder = join_paths(vec![export_to_folder, allowed_file_name(name).as_str()]);
        create_dir_if_not_exists(comic_folder.as_str());
        Ok(Box::new(Self {
            comic_folder,
            chaper_folder: Mutex::new(None),
        }))
    }
}

#[async_trait]
impl Exporter for FolderExporter {
    async fn on_cover(&mut self, source: &str, format: &str) -> Result<()> {
        let cover_path = join_paths(vec![
            self.comic_folder.as_str(),
            format!("cover.{}", format).as_str(),
        ]);
        tokio::fs::copy(source, cover_path.as_str()).await?;
        Ok(())
    }

    async fn on_chapter(&mut self, name: &str) -> Result<()> {
        let path = join_paths(vec![
            self.comic_folder.as_str(),
            allowed_file_name(name).as_str(),
        ]);
        create_dir_if_not_exists(path.as_str());
        let mut lock = self.chaper_folder.lock().await;
        *lock = Some(path);
        Ok(())
    }

    async fn on_page(&mut self, source: &str, format: &str, index: i32) -> Result<()> {
        let chapter_folder = self.chaper_folder.lock().await;
        let cf = chapter_folder
            .as_ref()
            .with_context(|| "chapter folder not found")?;
        let page_path = join_paths(vec![
            cf.as_str(),
            format!("{:04}.{}", index, format).as_str(),
        ]);
        tokio::fs::copy(source, page_path.as_str()).await?;
        Ok(())
    }

    async fn finish(mut self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

struct ZipExporter {
    writer: ZipFileWriter<tokio::fs::File>,
    chaper_folder: Mutex<Option<String>>,
}

impl ZipExporter {
    async fn on_start(export_to_folder: &str, name: &str) -> Result<Box<dyn Exporter>> {
        let comic_folder = join_paths(vec![export_to_folder, allowed_file_name(name).as_str()]);
        let file = tokio::fs::File::create(format!("{}.zip", comic_folder).as_str()).await?;
        let writer = ZipFileWriter::with_tokio(file);
        let comic_folder = join_paths(vec![export_to_folder, allowed_file_name(name).as_str()]);
        create_dir_if_not_exists(comic_folder.as_str());
        Ok(Box::new(Self {
            writer,
            chaper_folder: Mutex::new(None),
        }))
    }

    async fn push_file(&mut self, source: &str, target: &str) -> Result<()> {
        let mut file = tokio::fs::File::open(source).await?;
        let builder = ZipEntryBuilder::new(target.into(), async_zip::Compression::Deflate);
        let mut entry_writer = self.writer.write_entry_stream(builder).await?;
        let mut buf = vec![0; 1 << 8];
        let mut a;
        while {
            a = file.read(buf.as_mut_slice()).await?;
            a > 0
        } {
            entry_writer.write_all(&buf[0..a]).await?;
        }
        // tokio::io::copy(&mut file, &mut entry_writer).await?;
        entry_writer.close().await?;
        Ok(())
    }
}

#[async_trait]
impl Exporter for ZipExporter {
    async fn on_cover(&mut self, source: &str, format: &str) -> Result<()> {
        let cover_path = format!("cover.{}", format);
        self.push_file(source, cover_path.as_str()).await?;
        Ok(())
    }

    async fn on_chapter(&mut self, name: &str) -> Result<()> {
        let path = allowed_file_name(name);
        create_dir_if_not_exists(path.as_str());
        let mut lock = self.chaper_folder.lock().await;
        *lock = Some(path);
        // todo: add folder entry
        Ok(())
    }

    async fn on_page(&mut self, source: &str, format: &str, index: i32) -> Result<()> {
        let chapter_folder = self.chaper_folder.lock().await;
        let cf = chapter_folder
            .as_ref()
            .with_context(|| "chapter folder not found")?;
        let page_path = join_paths(vec![
            cf.as_str(),
            format!("{:04}.{}", index, format).as_str(),
        ]);
        drop(chapter_folder);
        self.push_file(source, page_path.as_str()).await?;
        Ok(())
    }

    async fn finish(mut self: Box<Self>) -> Result<()> {
        self.writer.close().await?;
        Ok(())
    }
}
