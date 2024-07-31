use std::collections::VecDeque;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_stream::{Stream, StreamExt};

use crate::application::common::file_storage_manager::{
    FileStorageManager, 
    FileStorageReader, 
    FileStorageRemover, 
    FileStorageWriter
};
use crate::domain::models::file_stream::FileStream;

struct ByteStream {
    file: File,
    buffer: [u8; 1],
}

impl ByteStream {
    fn new(file: File) -> Self {
        Self {
            file,
            buffer: [0],
        }
    }
}

impl Stream for ByteStream {
    type Item = io::Result<u8>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        let buf = &mut self.buffer;
        let fut = self.file.read(buf);

        tokio::pin!(fut);

        match fut.poll(cx) {
            Poll::Ready(Ok(0)) => Poll::Ready(None),  // EOF
            Poll::Ready(Ok(_)) => Poll::Ready(Some(Ok(buf[0]))),
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl FileStream for ByteStream {}


struct BufferPool {
    pool: Mutex<VecDeque<Vec<u8>>>,
    buffer_size: usize,
}

impl BufferPool {
    fn new(buffer_size: usize, pool_size: usize) -> Self {
        let mut pool = VecDeque::with_capacity(pool_size);
        for _ in 0..pool_size {
            pool.push_back(vec![0; buffer_size]);
        }
        BufferPool {
            pool: Mutex::new(pool),
            buffer_size,
        }
    }

    fn get_buffer(&self) -> Vec<u8> {
        let mut pool = self.pool.lock().unwrap();
        pool.pop_front().unwrap_or_else(|| vec![0; self.buffer_size])
    }

    fn return_buffer(&self, buffer: Vec<u8>) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < pool.capacity() {
            pool.push_back(buffer);
        }
    }
}


pub struct FileStorage {
    path: Box<Path>
}

impl FileStorage {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
}

impl FileStorageReader for FileStorage {
    async fn read_file<T: Into<String>>(&self, filename: &T) -> Box<dyn FileStream> {
        let file = File::open(self.path.join(filename.into())).await.unwrap();
        Box::new(ByteStream::new(file))
    }
}

impl FileStorageWriter for FileStorage {
    async fn save_file<T: Into<String>>(
        &self, 
        filename: &T, 
        content_type: Option<&T>,
        size_range: Option<(u64, u64)>,
        bytes: &dyn FileStream
    ) -> Result<String, String> {
        let mut file = File::create(self.path.join(filename.into())).await.unwrap();
        let mut stream = bytes;
        let mut buf = Vec::new();  // Todo: add buffer pool
        while let Some(byte) = stream.next().await {
            // todo: check size range
            // todo: check content type
            buf.push(byte.unwrap());
        }
        file.write_all(&buf).await.unwrap();
        Ok("982d9e3eb996f559e633f4d194def3761d909f5a3b647d1a851fead67c32c9d1".to_string())  // Todo: calculate hash
    }
}

impl FileStorageRemover for FileStorage {
    async fn remove_file<T: Into<String>>(&self, filename: &T) {
        tokio::fs::remove_file(self.path.join(filename.into())).await.unwrap();
    }
}

impl FileStorageManager for FileStorage {}


#[cfg(test)]
mod tests {
    use tokio::fs::OpenOptions;
    use tokio::io::AsyncWriteExt;
    use tokio_stream::StreamExt;

    use super::*;

    #[tokio::test]
    async fn test_byte_stream() {
        let file_path = "test_byte_stream.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .await
            .unwrap();

        file.write_all(b"hello world").await.unwrap();

        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .await
            .unwrap();

        let mut stream = ByteStream::new(file);

        let mut buf = Vec::new();
        while let Some(byte) = stream.next().await {
            buf.push(byte.unwrap());
        }

        assert_eq!(buf, b"hello world");

        tokio::fs::remove_file(file_path).await.unwrap();
    }
}