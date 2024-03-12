use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BytesMut};
use futures_util::{Stream, StreamExt};

use crate::messages::{StreamChunk, StreamError};

/// The result type as stream item for the chunk stream.
pub type ChunkStreamResult = Result<StreamChunk, StreamError>;

/// The stream item of the reqwest response.
type ReqwestStreamItem = Result<bytes::Bytes, reqwest::Error>;

/// The stream of message chunks.
pub struct ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem>,
{
    stream: S,
    buffer: BytesMut,
}

impl<S> ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem>,
{
    pub(crate) fn new(stream: S) -> Self {
        ChunkStream {
            stream,
            buffer: BytesMut::new(),
        }
    }
}

impl<S> Stream for ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem> + Unpin,
{
    type Item = ChunkStreamResult;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(position) = self
                .buffer
                .iter()
                .position(|b| *b == b'\n')
            {
                if let Some(end) = self.buffer[position + 1..]
                    .iter()
                    .position(|b| *b == b'\n')
                {
                    let chunk_end = position + end + 2;
                    let chunk = self
                        .buffer
                        .split_to(chunk_end)
                        .to_vec();
                    self.buffer.advance(1); // Skip the newline.

                    // Check if the chunk is not empty.
                    if chunk
                        .iter()
                        .any(|b| *b != b'\r' && *b != b'\n')
                    {
                        let chunk = String::from_utf8(chunk)
                            .map_err(StreamError::StringDecodingError)?;

                        let chunk = StreamChunk::parse(&chunk)?;
                        return Poll::Ready(Some(Ok(chunk)));
                    }
                }
            }

            match self
                .stream
                .poll_next_unpin(cx)
            {
                // The stream has more data.
                | Poll::Ready(Some(Ok(chunk))) => {
                    self.buffer.extend(&chunk);
                    // Continue to the next iteration of the loop.
                },
                // The stream has an error.
                | Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Some(Err(StreamError::ReqwestError(
                        error,
                    ))));
                },
                // The stream has no more data.
                | Poll::Ready(None) => {
                    return if self.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let remaining = self.buffer.split_off(0);
                        let remaining =
                            String::from_utf8(remaining.to_vec())
                                .map_err(StreamError::StringDecodingError)?;
                        let chunk = StreamChunk::parse(&remaining)?;
                        Poll::Ready(Some(Ok(chunk)))
                    };
                },
                // The stream has no more data for now.
                | Poll::Pending => return Poll::Pending,
            }
        }
    }
}
