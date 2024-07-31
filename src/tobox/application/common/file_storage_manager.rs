use async_trait::async_trait;
use crate::domain::models::file_stream::FileStream;

#[async_trait]
pub trait FileStorageReader {
    async fn read_file<T: Into<String>>(&self, filename: &T) -> dyn FileStream;
}

#[async_trait]
pub trait FileStorageWriter {
    
    ///  Save a file to the storage
    ///  * filename: the name of the file
    ///  * content_type: the content type of the file
    ///  * size_range: the range of the file size
    ///  * bytes: the file content
    ///  * return: the file hash sha256
    /// 
    /// Content-type and size-range, if set, will be used to check for consistency 
    /// with the downloaded stream and will throw an exception if it does not match.
    /// 
    /// The file will be saved in the storage and the hash sha256 will be returned.
    /// 
    /// Content-type, filesize and hash are calculated during file upload, it justifies the 
    /// existence of this function
    async fn save_file<T: Into<String>>(
        &self, 
        filename: &T,
        content_type: Option<&T>,
        size_range: Option<(u64, u64)>,
        bytes: &dyn FileStream
    ) -> String;
}

#[async_trait]
pub trait FileStorageRemover {
    async fn remove_file<T: Into<String>>(&self, filename: &T);
}

pub trait FileStorageManager: FileStorageReader + FileStorageWriter + FileStorageRemover {}
