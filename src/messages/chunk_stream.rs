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

                    // Skip the newline.
                    self.buffer.advance(1);

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

#[cfg(test)]
mod tests {
    use super::super::super::messages::*;
    use super::*;

    #[tokio::test]
    async fn poll_next() {
        let source = r#"event: message_start
data: {"type": "message_start", "message": {"id": "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY", "type": "message", "role": "assistant", "content": [], "model": "claude-3-opus-20240229", "stop_reason": null, "stop_sequence": null, "usage": {"input_tokens": 25, "output_tokens": 1}}}

event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}

event: ping
data: {"type": "ping"}

event: content_block_delta
data: {"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": "Hello"}}

event: content_block_delta
data: {"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": "!"}}

event: content_block_stop
data: {"type": "content_block_stop", "index": 0}

event: message_delta
data: {"type": "message_delta", "delta": {"stop_reason": "end_turn", "stop_sequence": null}, "usage": {"output_tokens": 15}}

"#;

        let input_stream = futures_util::stream::iter(vec![Ok(
            bytes::Bytes::from(source),
        )]);

        let mut chunk_stream = ChunkStream::new(input_stream);

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::MessageStart(message_start) => {
                assert_eq!(
                    message_start,
                    MessageStart::new(MessagesResponseBody {
                        id: "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY"
                            .to_string(),
                        _type: "message".to_string(),
                        role: Role::Assistant,
                        content: vec![].into(),
                        model: ClaudeModel::Claude3Opus20240229,
                        stop_reason: None,
                        stop_sequence: None,
                        usage: Usage {
                            input_tokens: 25,
                            output_tokens: 1,
                        },
                    }),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::ContentBlockStart(content_block_start) => {
                assert_eq!(
                    content_block_start,
                    ContentBlockStart::new(0, "".into()),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::Ping(ping) => {
                assert_eq!(ping, Ping::new());
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDelta::new(0, "Hello".into()),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDelta::new(0, "!".into()),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::ContentBlockStop(content_block_stop) => {
                assert_eq!(
                    content_block_stop,
                    ContentBlockStop::new(0),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | StreamChunk::MessageDelta(message_delta) => {
                assert_eq!(
                    message_delta,
                    MessageDelta::new(
                        StreamResult {
                            stop_reason: Some(StopReason::EndTurn),
                            stop_sequence: None,
                        },
                        DeltaUsage {
                            output_tokens: 15
                        },
                    ),
                );
            },
            | _ => panic!("unexpected chunk type"),
        }

        assert!(chunk_stream
            .next()
            .await
            .is_none());
    }
}
