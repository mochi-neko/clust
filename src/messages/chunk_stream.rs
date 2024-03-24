use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BytesMut};
use futures_core::Stream;
use pin_project::pin_project;

use crate::messages::{MessageChunk, StreamError};

/// The stream of message chunks with `tokio` backend.
#[pin_project]
pub(crate) struct ChunkStream<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    #[pin]
    stream: S,
    buffer: BytesMut,
}

impl<S> ChunkStream<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    /// Create a new chunk stream.
    pub fn new(stream: S) -> Self {
        ChunkStream {
            stream,
            buffer: BytesMut::new(),
        }
    }
}

impl<S> Stream for ChunkStream<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<MessageChunk, StreamError>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            if let Some(position) = this
                .buffer
                .iter()
                .position(|b| *b == b'\n')
            {
                if let Some(end) = this.buffer[position + 1..]
                    .iter()
                    .position(|b| *b == b'\n')
                {
                    let chunk_end = position + end + 2;
                    let chunk = this
                        .buffer
                        .split_to(chunk_end)
                        .to_vec();

                    // Skip the newline.
                    this.buffer.advance(1);

                    // Check if the chunk is not empty.
                    if chunk
                        .iter()
                        .any(|b| *b != b'\n')
                    {
                        let chunk = String::from_utf8(chunk)
                            .map_err(StreamError::StringDecodingError)?;

                        let chunk = MessageChunk::parse(&chunk)?;
                        return Poll::Ready(Some(Ok(chunk)));
                    }
                }
            }

            match this
                .stream
                .as_mut()
                .poll_next(cx)
            {
                // The stream has more data.
                | Poll::Ready(Some(Ok(chunk))) => {
                    this.buffer.extend(&chunk);
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
                    return if this.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let remaining = this.buffer.split_off(0);
                        let remaining =
                            String::from_utf8(remaining.to_vec())
                                .map_err(StreamError::StringDecodingError)?;
                        let chunk = MessageChunk::parse(&remaining)?;
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
    async fn next_by_futures_util() {
        use futures_util::StreamExt;

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
            | MessageChunk::MessageStart(message_start) => {
                assert_eq!(
                    message_start,
                    MessageStartChunk::new(MessagesResponseBody {
                        id: "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY"
                            .to_string(),
                        _type: MessageObjectType::Message,
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
            | MessageChunk::ContentBlockStart(content_block_start) => {
                assert_eq!(
                    content_block_start,
                    ContentBlockStartChunk::new(0, "".into()),
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
            | MessageChunk::Ping(ping) => {
                assert_eq!(ping, PingChunk::new());
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | MessageChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDeltaChunk::new(0, "Hello".into()),
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
            | MessageChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDeltaChunk::new(0, "!".into()),
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
            | MessageChunk::ContentBlockStop(content_block_stop) => {
                assert_eq!(
                    content_block_stop,
                    ContentBlockStopChunk::new(0),
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
            | MessageChunk::MessageDelta(message_delta) => {
                assert_eq!(
                    message_delta,
                    MessageDeltaChunk::new(
                        StreamStop {
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

    #[tokio::test]
    async fn next_by_tokio_stream() {
        use tokio_stream::StreamExt;

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

        let input_stream = tokio_stream::iter(vec![Ok(
            bytes::Bytes::from(source),
        )]);

        let mut chunk_stream = ChunkStream::new(input_stream);

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | MessageChunk::MessageStart(message_start) => {
                assert_eq!(
                    message_start,
                    MessageStartChunk::new(MessagesResponseBody {
                        id: "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY"
                            .to_string(),
                        _type: MessageObjectType::Message,
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
            | MessageChunk::ContentBlockStart(content_block_start) => {
                assert_eq!(
                    content_block_start,
                    ContentBlockStartChunk::new(0, "".into()),
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
            | MessageChunk::Ping(ping) => {
                assert_eq!(ping, PingChunk::new());
            },
            | _ => panic!("unexpected chunk type"),
        }

        let chunk = chunk_stream
            .next()
            .await
            .unwrap()
            .unwrap();
        match chunk {
            | MessageChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDeltaChunk::new(0, "Hello".into()),
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
            | MessageChunk::ContentBlockDelta(content_block_delta) => {
                assert_eq!(
                    content_block_delta,
                    ContentBlockDeltaChunk::new(0, "!".into()),
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
            | MessageChunk::ContentBlockStop(content_block_stop) => {
                assert_eq!(
                    content_block_stop,
                    ContentBlockStopChunk::new(0),
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
            | MessageChunk::MessageDelta(message_delta) => {
                assert_eq!(
                    message_delta,
                    MessageDeltaChunk::new(
                        StreamStop {
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
