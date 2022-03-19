use std::io::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use tokio::fs::{DirEntry, File, read_dir, ReadDir};
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use crate::fs::{FsOperation, MDirReader, MFile, MLineReader};
use async_trait::async_trait;
use anyhow::Result;

pub struct TokioLineReader(Lines<BufReader<File>>);

#[async_trait]
impl MLineReader for TokioLineReader {
    async fn read_line(&mut self) -> Option<String> {
        let next_line = self.0.next_line().await;
        match next_line {
            Ok(content) => content,
            Err(err) => None,
        }
    }
}

pub struct TokioDirReader(ReadDir);

#[async_trait]
impl MDirReader for TokioDirReader {
    async fn next_file(&mut self) -> Option<MFile<PathBuf>> {
        let query = self.0.next_entry().await;

        let result = query.unwrap();

        if let Some(entry) = result {
            Some(
                MFile {
                    path: PathBuf::from(entry.path()),
                    filename: entry.file_name().to_str().unwrap().to_string(),
                    is_dir: entry.path().is_dir(),
                }
            )
        } else {
            None
        }
    }
}

pub struct TokioFsOperation();

#[async_trait]
impl FsOperation for TokioFsOperation {
    type DirReaderType = TokioDirReader;

    async fn read_dir(&self, path: &Path) -> anyhow::Result<Self::DirReaderType, Error> {
        let dirs = read_dir(path).await?;
        Ok(TokioDirReader(dirs))
    }

    type LineReaderType = TokioLineReader;

    async fn read_lines(&self, path: &Path) -> anyhow::Result<Self::LineReaderType, Error> {
        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        Ok(TokioLineReader(reader.lines()))
    }
}