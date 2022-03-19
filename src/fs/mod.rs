pub mod tokio_fs_reader;

use std::path::{Path, PathBuf};
use anyhow::Result;
use async_trait::async_trait;
pub use tokio_fs_reader::TokioFsOperation;

#[derive(Debug)]
pub struct MFile<P: AsRef<Path>>{
    pub path: P,
    pub filename: String,
    pub is_dir: bool,
}

#[async_trait]
pub trait MLineReader{
    async fn read_line(&mut self) -> Option<String>;
}

#[async_trait]
pub trait MDirReader {
    // fn next(&mut self) -> Result<Option<MFile>>;
    async fn next_file(&mut self) -> Option<MFile<PathBuf>>;
}



#[async_trait]
pub trait FsOperation {
    type DirReaderType: MDirReader;
    async fn read_dir(&self, path: &Path) -> Result<Self::DirReaderType, std::io::Error>;

    type LineReaderType: MLineReader;
    async fn read_lines(&self, path: &Path) -> Result<Self::LineReaderType, std::io::Error>;
}


