use std::fmt::Display;
use std::str::FromStr;

use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};
use crate::messages::{
    MessagesResponseBody, StopReason, StopSequence, StreamError,
    TextContentBlock, TextDeltaContentBlock,
};

/// The stream chunk of messages.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamChunk {
    MessageStart(MessageStart),
    ContentBlockStart(ContentBlockStart),
    Ping(Ping),
    ContentBlockDelta(ContentBlockDelta),
    ContentBlockStop(ContentBlockStop),
    MessageDelta(MessageDelta),
    MessageStop(MessageStop),
}

impl Display for StreamChunk {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StreamChunk::MessageStart(message_start) => {
                write!(f, "{}", message_start)
            },
            | StreamChunk::ContentBlockStart(content_block_start) => {
                write!(f, "{}", content_block_start)
            },
            | StreamChunk::Ping(ping) => write!(f, "{}", ping),
            | StreamChunk::ContentBlockDelta(content_block_delta) => {
                write!(f, "{}", content_block_delta)
            },
            | StreamChunk::ContentBlockStop(content_block_stop) => {
                write!(f, "{}", content_block_stop)
            },
            | StreamChunk::MessageDelta(message_delta) => {
                write!(f, "{}", message_delta)
            },
            | StreamChunk::MessageStop(message_stop) => {
                write!(f, "{}", message_stop)
            },
        }
    }
}

impl StreamChunk {
    pub(crate) fn parse(source: &str) -> Result<StreamChunk, StreamError> {
        let lines = source
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();
        if lines.len() != 2 {
            return Err(StreamError::ParseChunkStringError(
                format!(
                    "Chunk must be two lines but not: {}",
                    source
                ),
            ));
        }

        let first_line = lines[0];
        let event = first_line
            .strip_prefix("event: ")
            .ok_or_else(|| {
                StreamError::ParseChunkStringError(format!(
                    "First line must start with 'event: ', but not: {}",
                    source
                ))
            })?;
        let chunk_type = StreamChunkType::from_str(event)
            .map_err(StreamError::ParseChunkStringError)?;

        let second_line = lines[1];
        let data = second_line
            .strip_prefix("data: ")
            .ok_or_else(|| {
                StreamError::ParseChunkStringError(format!(
                    "Second line must start with 'data: ', but not: {}",
                    source
                ))
            })?;
        match chunk_type {
            | StreamChunkType::MessageStart => {
                let message = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::MessageStart(message))
            },
            | StreamChunkType::ContentBlockStart => {
                let content_block = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::ContentBlockStart(
                    content_block,
                ))
            },
            | StreamChunkType::Ping => {
                let ping = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::Ping(ping))
            },
            | StreamChunkType::ContentBlockDelta => {
                let delta = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::ContentBlockDelta(delta))
            },
            | StreamChunkType::ContentBlockStop => {
                let stop = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::ContentBlockStop(stop))
            },
            | StreamChunkType::MessageDelta => {
                let delta = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::MessageDelta(delta))
            },
            | StreamChunkType::MessageStop => {
                let stop = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(StreamChunk::MessageStop(stop))
            },
        }
    }
}

/// The type of stream chunk.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StreamChunkType {
    /// message_start
    MessageStart,
    /// content_block_start
    ContentBlockStart,
    /// ping
    Ping,
    /// content_block_delta
    ContentBlockDelta,
    /// content_block_stop
    ContentBlockStop,
    /// message_delta
    MessageDelta,
    /// message_stop
    MessageStop,
}

impl Display for StreamChunkType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StreamChunkType::MessageStart => write!(f, "message_start"),
            | StreamChunkType::ContentBlockStart => {
                write!(f, "content_block_start")
            },
            | StreamChunkType::Ping => write!(f, "ping"),
            | StreamChunkType::ContentBlockDelta => {
                write!(f, "content_block_delta")
            },
            | StreamChunkType::ContentBlockStop => {
                write!(f, "content_block_stop")
            },
            | StreamChunkType::MessageDelta => write!(f, "message_delta"),
            | StreamChunkType::MessageStop => write!(f, "message_stop"),
        }
    }
}

impl FromStr for StreamChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "message_start" => Ok(StreamChunkType::MessageStart),
            | "content_block_start" => Ok(StreamChunkType::ContentBlockStart),
            | "ping" => Ok(StreamChunkType::Ping),
            | "content_block_delta" => Ok(StreamChunkType::ContentBlockDelta),
            | "content_block_stop" => Ok(StreamChunkType::ContentBlockStop),
            | "message_delta" => Ok(StreamChunkType::MessageDelta),
            | "message_stop" => Ok(StreamChunkType::MessageStop),
            | _ => Err(format!(
                "Unknown stream chunk type: {}",
                s
            )),
        }
    }
}

impl_enum_string_serialization!(
    StreamChunkType,
    MessageStart => "message_start",
    ContentBlockStart => "content_block_start",
    Ping => "ping",
    ContentBlockDelta => "content_block_delta",
    ContentBlockStop => "content_block_stop",
    MessageDelta => "message_delta",
    MessageStop => "message_stop"
);

/// The message start chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageStart {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
    /// The start message.
    pub message: MessagesResponseBody,
}

impl Default for MessageStart {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::MessageStart,
            message: Default::default(),
        }
    }
}

impl_display_for_serialize!(MessageStart);

/// The content block start chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockStart {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
    /// The index.
    pub index: u32,
    /// The text content block of start.
    pub content_block: TextContentBlock,
}

impl Default for ContentBlockStart {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::ContentBlockStart,
            index: Default::default(),
            content_block: Default::default(),
        }
    }
}

impl_display_for_serialize!(ContentBlockStart);

/// The ping chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ping {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
}

impl Default for Ping {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::Ping,
        }
    }
}

impl_display_for_serialize!(Ping);

/// The content block delta chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockDelta {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
    /// The index.
    pub index: u32,
    /// The text delta content block.
    pub delta: TextDeltaContentBlock,
}

impl Default for ContentBlockDelta {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::ContentBlockDelta,
            index: Default::default(),
            delta: Default::default(),
        }
    }
}

impl_display_for_serialize!(ContentBlockDelta);

/// The content block stop chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockStop {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
    /// The index.
    pub index: u32,
}

impl Default for ContentBlockStop {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::ContentBlockStop,
            index: Default::default(),
        }
    }
}

impl_display_for_serialize!(ContentBlockStop);

/// The message delta chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageDelta {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
    /// The result of this stream.
    pub delta: StreamResult,
    /// The billing and rate-limit usage of this stream.
    pub usage: StreamUsage,
}

impl Default for MessageDelta {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::MessageDelta,
            delta: Default::default(),
            usage: Default::default(),
        }
    }
}

impl_display_for_serialize!(MessageDelta);

/// The message stop chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageStop {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: StreamChunkType,
}

impl Default for MessageStop {
    fn default() -> Self {
        Self {
            _type: StreamChunkType::MessageStop,
        }
    }
}

impl_display_for_serialize!(MessageStop);

/// The stream result.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct StreamResult {
    /// The stop reason of this stream.
    pub stop_reason: Option<StopReason>,
    /// The stop sequence of this stream.
    pub stop_sequence: Option<StopSequence>,
}

impl_display_for_serialize!(StreamResult);

/// The result usage of the stream.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct StreamUsage {
    /// The number of output tokens which were used.
    pub output_tokens: u32,
}

impl_display_for_serialize!(StreamUsage);
