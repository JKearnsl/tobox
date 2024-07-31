use tokio::io;
use tokio_stream::Stream;

pub trait FileStream: Stream<Item = io::Result<u8>> + Unpin + Send {}
