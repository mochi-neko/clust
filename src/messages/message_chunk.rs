use serde_json_fmt::JsonFormat;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};
use crate::messages::{
    ContentType, MessageChunkTypeError, MessagesResponseBody, StopReason,
    StopSequence, StreamError, TextContentBlock,
};

use super::ToolUseContentBlock;

/// The stream chunk of messages.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageChunk {
    /// Message start chunk.
    MessageStart(MessageStartChunk),
    /// Content block start chunk.
    ContentBlockStart(ContentBlockStartChunk),
    /// Ping chunk.
    Ping(PingChunk),
    /// Content block delta chunk.
    ContentBlockDelta(ContentBlockDeltaChunk),
    /// Content block stop chunk.
    ContentBlockStop(ContentBlockStopChunk),
    /// Message delta chunk.
    MessageDelta(MessageDeltaChunk),
    /// Message stop chunk.
    MessageStop(MessageStopChunk),
}

impl Display for MessageChunk {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let json_format = JsonFormat::new()
            .comma(", ")
            .map_err(|_| std::fmt::Error)?
            .colon(": ")
            .map_err(|_| std::fmt::Error)?;

        match self {
            | MessageChunk::MessageStart(message_start) => {
                let json = json_format
                    .format_to_string(&message_start)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    message_start._type, json
                )
            },
            | MessageChunk::ContentBlockStart(content_block_start) => {
                let json = json_format
                    .format_to_string(&content_block_start)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    content_block_start._type, json
                )
            },
            | MessageChunk::Ping(ping) => {
                let json = json_format
                    .format_to_string(&ping)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    ping._type, json
                )
            },
            | MessageChunk::ContentBlockDelta(content_block_delta) => {
                let json = json_format
                    .format_to_string(&content_block_delta)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    content_block_delta._type, json
                )
            },
            | MessageChunk::ContentBlockStop(content_block_stop) => {
                let json = json_format
                    .format_to_string(&content_block_stop)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    content_block_stop._type, json
                )
            },
            | MessageChunk::MessageDelta(message_delta) => {
                let json = json_format
                    .format_to_string(&message_delta)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    message_delta._type, json
                )
            },
            | MessageChunk::MessageStop(message_stop) => {
                let json = json_format
                    .format_to_string(&message_stop)
                    .map_err(|_| std::fmt::Error)?;

                write!(
                    f,
                    "event: {}\ndata: {}",
                    message_stop._type, json
                )
            },
        }
    }
}

impl MessageChunk {
    pub(crate) fn parse(source: &str) -> Result<MessageChunk, StreamError> {
        let lines = source
            .lines()
            .collect::<Vec<&str>>();

        // Check length
        if lines.len() != 2 {
            return Err(StreamError::ParseChunkStringError(
                format!(
                    "Chunk must be two lines but not: {}",
                    source
                ),
            ));
        }

        // Parse the event segment to the chunk type.
        let first_line = lines[0];
        let event = first_line
            .strip_prefix("event: ")
            .ok_or_else(|| {
                StreamError::ParseChunkStringError(format!(
                    "First line must start with 'event: ', but not: {}",
                    source
                ))
            })?;
        let chunk_type = MessageChunkType::from_str(event)
            .map_err(StreamError::MessageChunkTypeError)?;

        // Parse the data segment to the chunk data.
        let second_line = lines[1];
        let data = second_line
            .strip_prefix("data: ")
            .ok_or_else(|| {
                StreamError::ParseChunkStringError(format!(
                    "Second line must start with 'data: ', but not: {}",
                    source
                ))
            })?;

        // Deserialize the chunk data.
        match chunk_type {
            | MessageChunkType::MessageStart => {
                let message = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::MessageStart(message))
            },
            | MessageChunkType::ContentBlockStart => {
                let content_block = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::ContentBlockStart(
                    content_block,
                ))
            },
            | MessageChunkType::Ping => {
                let ping = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::Ping(ping))
            },
            | MessageChunkType::ContentBlockDelta => {
                let delta = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::ContentBlockDelta(delta))
            },
            | MessageChunkType::ContentBlockStop => {
                let stop = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::ContentBlockStop(stop))
            },
            | MessageChunkType::MessageDelta => {
                let delta = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::MessageDelta(delta))
            },
            | MessageChunkType::MessageStop => {
                let stop = serde_json::from_str(data)
                    .map_err(StreamError::ChunkDataDeserializationError)?;
                Ok(MessageChunk::MessageStop(stop))
            },
        }
    }
}

/// The type of message chunk.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageChunkType {
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

impl Display for MessageChunkType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | MessageChunkType::MessageStart => write!(f, "message_start"),
            | MessageChunkType::ContentBlockStart => {
                write!(f, "content_block_start")
            },
            | MessageChunkType::Ping => write!(f, "ping"),
            | MessageChunkType::ContentBlockDelta => {
                write!(f, "content_block_delta")
            },
            | MessageChunkType::ContentBlockStop => {
                write!(f, "content_block_stop")
            },
            | MessageChunkType::MessageDelta => write!(f, "message_delta"),
            | MessageChunkType::MessageStop => write!(f, "message_stop"),
        }
    }
}

impl FromStr for MessageChunkType {
    type Err = MessageChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "message_start" => Ok(MessageChunkType::MessageStart),
            | "content_block_start" => Ok(MessageChunkType::ContentBlockStart),
            | "ping" => Ok(MessageChunkType::Ping),
            | "content_block_delta" => Ok(MessageChunkType::ContentBlockDelta),
            | "content_block_stop" => Ok(MessageChunkType::ContentBlockStop),
            | "message_delta" => Ok(MessageChunkType::MessageDelta),
            | "message_stop" => Ok(MessageChunkType::MessageStop),
            | _ => Err(MessageChunkTypeError {
                chunk_type: s.to_string(),
            }),
        }
    }
}

impl_enum_string_serialization!(
    MessageChunkType,
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
pub struct MessageStartChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
    /// The start message.
    pub message: MessagesResponseBody,
}

impl Default for MessageStartChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::MessageStart,
            message: Default::default(),
        }
    }
}

impl_display_for_serialize!(MessageStartChunk);

impl MessageStartChunk {
    /// Creates a new `MessageStart` instance.
    pub fn new(message: MessagesResponseBody) -> Self {
        Self {
            _type: MessageChunkType::MessageStart,
            message,
        }
    }
}

/// The content block start chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockStartChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
    /// The index.
    pub index: u32,
    /// The text content block of start.
    pub content_block: ContentBlockStart,
}

impl Default for ContentBlockStartChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::ContentBlockStart,
            index: Default::default(),
            content_block: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ContentBlockStart {
    TextContentBlock(TextContentBlock),
    ToolUseContentBlock(ToolUseContentBlock),
}
impl Default for ContentBlockStart {
    fn default() -> Self {
        Self::TextContentBlock(Default::default())
    }
}

impl_display_for_serialize!(ContentBlockStartChunk);

impl ContentBlockStartChunk {
    /// Creates a new `ContentBlockStart` instance.
    pub fn new(
        index: u32,
        content_block: TextContentBlock,
    ) -> Self {
        Self {
            _type: MessageChunkType::ContentBlockStart,
            index,
            content_block: ContentBlockStart::TextContentBlock(content_block),
        }
    }
}

/// The ping chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PingChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
}

impl Default for PingChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::Ping,
        }
    }
}

impl_display_for_serialize!(PingChunk);

impl PingChunk {
    /// Creates a new `Ping` instance.
    pub fn new() -> Self {
        Self {
            _type: MessageChunkType::Ping,
        }
    }
}

/// The content block delta chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockDeltaChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
    /// The index.
    pub index: u32,
    /// The text delta content block.
    pub delta: ContentBlockDelta,
}

impl Default for ContentBlockDeltaChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::ContentBlockDelta,
            index: Default::default(),
            delta: Default::default(),
        }
    }
}

impl_display_for_serialize!(ContentBlockDeltaChunk);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ContentBlockDelta  {
    TextDeltaContentBlock(TextDeltaContentBlock),
    InputJsonDeltaBlock(InputJsonDeltaBlock)
}

impl Default for ContentBlockDelta {
    fn default() -> Self {
        ContentBlockDelta::TextDeltaContentBlock(Default::default())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputJsonDeltaBlock {
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The partial json delta content.
    pub partial_json: String,
}
impl ContentBlockDeltaChunk {
    /// Creates a new `ContentBlockDelta` instance.
    pub fn new(
        index: u32,
        delta: TextDeltaContentBlock,
    ) -> Self {
        Self {
            _type: MessageChunkType::ContentBlockDelta,
            index,
            delta: ContentBlockDelta::TextDeltaContentBlock(delta),
        }
    }
}

/// The content block stop chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentBlockStopChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
    /// The index.
    pub index: u32,
}

impl Default for ContentBlockStopChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::ContentBlockStop,
            index: Default::default(),
        }
    }
}

impl_display_for_serialize!(ContentBlockStopChunk);

impl ContentBlockStopChunk {
    /// Creates a new `ContentBlockStop` instance.
    pub fn new(index: u32) -> Self {
        Self {
            _type: MessageChunkType::ContentBlockStop,
            index,
        }
    }
}

/// The message delta chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageDeltaChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
    /// The result of this stream.
    pub delta: StreamStop,
    /// The billing and rate-limit usage of this stream.
    pub usage: DeltaUsage,
}

impl Default for MessageDeltaChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::MessageDelta,
            delta: Default::default(),
            usage: Default::default(),
        }
    }
}

impl_display_for_serialize!(MessageDeltaChunk);

impl MessageDeltaChunk {
    /// Creates a new `MessageDelta` instance.
    pub fn new(
        delta: StreamStop,
        usage: DeltaUsage,
    ) -> Self {
        Self {
            _type: MessageChunkType::MessageDelta,
            delta,
            usage,
        }
    }
}

/// The message stop chunk.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageStopChunk {
    /// The type of stream chunk.
    #[serde(rename = "type")]
    pub _type: MessageChunkType,
}

impl Default for MessageStopChunk {
    fn default() -> Self {
        Self {
            _type: MessageChunkType::MessageStop,
        }
    }
}

impl_display_for_serialize!(MessageStopChunk);

impl MessageStopChunk {
    /// Creates a new `MessageStop` instance.
    pub fn new() -> Self {
        Self {
            _type: MessageChunkType::MessageStop,
        }
    }
}

/// The text delta content block.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextDeltaContentBlock {
    /// The content type. It is always `text_delta`.
    #[serde(rename = "type")]
    pub _type: ContentType,
    /// The text delta content.
    pub text: String,
}

impl Default for TextDeltaContentBlock {
    fn default() -> Self {
        Self {
            _type: ContentType::TextDelta,
            text: String::new(),
        }
    }
}

impl_display_for_serialize!(TextDeltaContentBlock);

impl From<String> for TextDeltaContentBlock {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for TextDeltaContentBlock {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl TextDeltaContentBlock {
    /// Creates a new text delta content block.
    pub(crate) fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            _type: ContentType::TextDelta,
            text: text.into(),
        }
    }
}

/// The stream stop information.
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct StreamStop {
    /// The stop reason of this stream.
    pub stop_reason: Option<StopReason>,
    /// The stop sequence of this stream.
    pub stop_sequence: Option<StopSequence>,
}

impl_display_for_serialize!(StreamStop);

/// The delta usage of the stream.
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
pub struct DeltaUsage {
    /// The number of output tokens which were used.
    pub output_tokens: u32,
}

impl_display_for_serialize!(DeltaUsage);

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn from_str_chunk_type() {
        assert_eq!(
            MessageChunkType::from_str("message_start").unwrap(),
            MessageChunkType::MessageStart
        );
        assert_eq!(
            MessageChunkType::from_str("content_block_start").unwrap(),
            MessageChunkType::ContentBlockStart
        );
        assert_eq!(
            MessageChunkType::from_str("ping").unwrap(),
            MessageChunkType::Ping
        );
        assert_eq!(
            MessageChunkType::from_str("content_block_delta").unwrap(),
            MessageChunkType::ContentBlockDelta
        );
        assert_eq!(
            MessageChunkType::from_str("content_block_stop").unwrap(),
            MessageChunkType::ContentBlockStop
        );
        assert_eq!(
            MessageChunkType::from_str("message_delta").unwrap(),
            MessageChunkType::MessageDelta
        );
        assert_eq!(
            MessageChunkType::from_str("message_stop").unwrap(),
            MessageChunkType::MessageStop
        );
    }

    #[test]
    fn display_chunk_type() {
        assert_eq!(
            MessageChunkType::MessageStart.to_string(),
            "message_start"
        );
        assert_eq!(
            MessageChunkType::ContentBlockStart.to_string(),
            "content_block_start"
        );
        assert_eq!(
            MessageChunkType::Ping.to_string(),
            "ping"
        );
        assert_eq!(
            MessageChunkType::ContentBlockDelta.to_string(),
            "content_block_delta"
        );
        assert_eq!(
            MessageChunkType::ContentBlockStop.to_string(),
            "content_block_stop"
        );
        assert_eq!(
            MessageChunkType::MessageDelta.to_string(),
            "message_delta"
        );
        assert_eq!(
            MessageChunkType::MessageStop.to_string(),
            "message_stop"
        );
    }

    #[test]
    fn serialize_chunk_type() {
        assert_eq!(
            serde_json::to_string(&MessageChunkType::MessageStart).unwrap(),
            r#""message_start""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::ContentBlockStart)
                .unwrap(),
            r#""content_block_start""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::Ping).unwrap(),
            r#""ping""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::ContentBlockDelta)
                .unwrap(),
            r#""content_block_delta""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::ContentBlockStop).unwrap(),
            r#""content_block_stop""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::MessageDelta).unwrap(),
            r#""message_delta""#
        );
        assert_eq!(
            serde_json::to_string(&MessageChunkType::MessageStop).unwrap(),
            r#""message_stop""#
        );
    }

    #[test]
    fn deserialize_chunk_type() {
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(r#""message_start""#)
                .unwrap(),
            MessageChunkType::MessageStart
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(
                r#""content_block_start""#
            )
            .unwrap(),
            MessageChunkType::ContentBlockStart
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(r#""ping""#).unwrap(),
            MessageChunkType::Ping
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(
                r#""content_block_delta""#
            )
            .unwrap(),
            MessageChunkType::ContentBlockDelta
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(r#""content_block_stop""#)
                .unwrap(),
            MessageChunkType::ContentBlockStop
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(r#""message_delta""#)
                .unwrap(),
            MessageChunkType::MessageDelta
        );
        assert_eq!(
            serde_json::from_str::<MessageChunkType>(r#""message_stop""#)
                .unwrap(),
            MessageChunkType::MessageStop
        );
    }

    #[test]
    fn default_delta_usage() {
        assert_eq!(
            DeltaUsage::default(),
            DeltaUsage {
                output_tokens: Default::default(),
            }
        );
    }

    #[test]
    fn display_delta_usage() {
        let usage = DeltaUsage {
            output_tokens: 1,
        };
        assert_eq!(
            usage.to_string(),
            "{\n  \"output_tokens\": 1\n}"
        );
    }

    #[test]
    fn serialize_delta_usage() {
        let usage = DeltaUsage {
            output_tokens: 1,
        };
        assert_eq!(
            serde_json::to_string(&usage).unwrap(),
            "{\"output_tokens\":1}"
        );
    }

    #[test]
    fn deserialize_delta_usage() {
        let usage = DeltaUsage {
            output_tokens: 1,
        };
        assert_eq!(
            serde_json::from_str::<DeltaUsage>(r#"{"output_tokens":1}"#)
                .unwrap(),
            usage
        );
    }

    #[test]
    fn default_stream_stop() {
        assert_eq!(
            StreamStop::default(),
            StreamStop {
                stop_reason: Default::default(),
                stop_sequence: Default::default(),
            }
        );
    }

    #[test]
    fn display_stream_stop() {
        let stop = StreamStop {
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
        };
        assert_eq!(
            stop.to_string(),
            "{\n  \"stop_reason\": \"end_turn\",\n  \"stop_sequence\": \"stop_sequence\"\n}"
        );
    }

    #[test]
    fn serialize_stream_stop() {
        let stop = StreamStop {
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
        };
        assert_eq!(
            serde_json::to_string(&stop).unwrap(),
            r#"{"stop_reason":"end_turn","stop_sequence":"stop_sequence"}"#
        );
    }

    #[test]
    fn deserialize_stream_stop() {
        let stop = StreamStop {
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
        };
        assert_eq!(
            serde_json::from_str::<StreamStop>(
                r#"{"stop_reason":"end_turn","stop_sequence":"stop_sequence"}"#
            )
            .unwrap(),
            stop
        );
    }

    #[test]
    fn default_message_start() {
        assert_eq!(
            MessageStartChunk::default(),
            MessageStartChunk {
                _type: MessageChunkType::MessageStart,
                message: Default::default(),
            }
        );
    }

    #[test]
    fn display_message_start() {
        let message_start = MessageStartChunk {
            _type: MessageChunkType::MessageStart,
            message: MessagesResponseBody {
                id: "id".to_string(),
                _type: MessageObjectType::Message,
                role: Role::Assistant,
                content: "content".into(),
                model: ClaudeModel::Claude3Sonnet20240229,
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
                usage: Usage {
                    input_tokens: 1,
                    output_tokens: 2,
                },
            },
        };
        assert_eq!(
            message_start.to_string(),
            "{\n  \"type\": \"message_start\",\n  \"message\": {\n    \"id\": \"id\",\n    \"type\": \"message\",\n    \"role\": \"assistant\",\n    \"content\": \"content\",\n    \"model\": \"claude-3-sonnet-20240229\",\n    \"stop_reason\": \"end_turn\",\n    \"stop_sequence\": \"stop_sequence\",\n    \"usage\": {\n      \"input_tokens\": 1,\n      \"output_tokens\": 2\n    }\n  }\n}"
        );
    }

    #[test]
    fn serialize_message_start() {
        let message_start = MessageStartChunk {
            _type: MessageChunkType::MessageStart,
            message: MessagesResponseBody {
                id: "id".to_string(),
                _type: MessageObjectType::Message,
                role: Role::Assistant,
                content: "content".into(),
                model: ClaudeModel::Claude3Sonnet20240229,
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
                usage: Usage {
                    input_tokens: 1,
                    output_tokens: 2,
                },
            },
        };
        assert_eq!(
            serde_json::to_string(&message_start).unwrap(),
            r#"{"type":"message_start","message":{"id":"id","type":"message","role":"assistant","content":"content","model":"claude-3-sonnet-20240229","stop_reason":"end_turn","stop_sequence":"stop_sequence","usage":{"input_tokens":1,"output_tokens":2}}}"#
        );
    }

    #[test]
    fn deserialize_message_start() {
        let message_start = MessageStartChunk {
            _type: MessageChunkType::MessageStart,
            message: MessagesResponseBody {
                id: "id".to_string(),
                _type: MessageObjectType::Message,
                role: Role::Assistant,
                content: "content".into(),
                model: ClaudeModel::Claude3Sonnet20240229,
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
                usage: Usage {
                    input_tokens: 1,
                    output_tokens: 2,
                },
            },
        };
        assert_eq!(
            serde_json::from_str::<MessageStartChunk>(
                r#"{"type":"message_start","message":{"id":"id","type":"message","role":"assistant","content":"content","model":"claude-3-sonnet-20240229","stop_reason":"end_turn","stop_sequence":"stop_sequence","usage":{"input_tokens":1,"output_tokens":2}}}"#
            )
            .unwrap(),
            message_start
        );
    }

    #[test]
    fn default_content_block_start() {
        assert_eq!(
            ContentBlockStartChunk::default(),
            ContentBlockStartChunk {
                _type: MessageChunkType::ContentBlockStart,
                index: Default::default(),
                content_block: Default::default(),
            }
        );
    }

    #[test]
    fn display_content_block_start() {
        let content_block_start = ContentBlockStartChunk {
            _type: MessageChunkType::ContentBlockStart,
            index: 1,
            content_block: ContentBlockStart::TextContentBlock(TextContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            content_block_start.to_string(),
            "{\n  \"type\": \"content_block_start\",\n  \"index\": 1,\n  \"content_block\": {\n    \"type\": \"text\",\n    \"text\": \"text\"\n  }\n}"
        );
    }

    #[test]
    fn serialize_content_block_start() {
        let content_block_start = ContentBlockStartChunk {
            _type: MessageChunkType::ContentBlockStart,
            index: 1,
            content_block: ContentBlockStart::TextContentBlock(TextContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::to_string(&content_block_start).unwrap(),
            "{\"type\":\"content_block_start\",\"index\":1,\"content_block\":{\"type\":\"text\",\"text\":\"text\"}}"
        );
    }

    #[test]
    fn deserialize_content_block_start() {
        let content_block_start = ContentBlockStartChunk {
            _type: MessageChunkType::ContentBlockStart,
            index: 1,
            content_block: ContentBlockStart::TextContentBlock(TextContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::from_str::<ContentBlockStartChunk>(
                "{\"type\":\"content_block_start\",\"index\":1,\"content_block\":{\"type\":\"text\",\"text\":\"text\"}}"
            )
            .unwrap(),
            content_block_start
        );
    }

    #[test]
    fn default_ping() {
        assert_eq!(
            PingChunk::default(),
            PingChunk {
                _type: MessageChunkType::Ping,
            }
        );
    }

    #[test]
    fn display_ping() {
        let ping = PingChunk::default();
        assert_eq!(
            ping.to_string(),
            "{\n  \"type\": \"ping\"\n}"
        );
    }

    #[test]
    fn serialize_ping() {
        let ping = PingChunk::default();
        assert_eq!(
            serde_json::to_string(&ping).unwrap(),
            r#"{"type":"ping"}"#
        );
    }

    #[test]
    fn deserialize_ping() {
        let ping = PingChunk::default();
        assert_eq!(
            serde_json::from_str::<PingChunk>(r#"{"type":"ping"}"#).unwrap(),
            ping
        );
    }

    #[test]
    fn default_content_block_delta() {
        assert_eq!(
            ContentBlockDeltaChunk::default(),
            ContentBlockDeltaChunk {
                _type: MessageChunkType::ContentBlockDelta,
                index: Default::default(),
                delta: Default::default(),
            }
        );
    }

    #[test]
    fn display_content_block_delta() {
        let content_block_delta = ContentBlockDeltaChunk {
            _type: MessageChunkType::ContentBlockDelta,
            index: 1,
            delta: ContentBlockDelta::TextDeltaContentBlock(TextDeltaContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            content_block_delta.to_string(),
            "{\n  \"type\": \"content_block_delta\",\n  \"index\": 1,\n  \"delta\": {\n    \"type\": \"text_delta\",\n    \"text\": \"text\"\n  }\n}"
        );
    }

    #[test]
    fn serialize_content_block_delta() {
        let content_block_delta = ContentBlockDeltaChunk {
            _type: MessageChunkType::ContentBlockDelta,
            index: 1,
            delta: ContentBlockDelta::TextDeltaContentBlock(TextDeltaContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::to_string(&content_block_delta).unwrap(),
            "{\"type\":\"content_block_delta\",\"index\":1,\"delta\":{\"type\":\"text_delta\",\"text\":\"text\"}}"
        );
    }

    #[test]
    fn deserialize_content_block_delta() {
        let content_block_delta = ContentBlockDeltaChunk {
            _type: MessageChunkType::ContentBlockDelta,
            index: 1,
            delta: ContentBlockDelta::TextDeltaContentBlock(TextDeltaContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(
            serde_json::from_str::<ContentBlockDeltaChunk>(
                "{\"type\":\"content_block_delta\",\"index\":1,\"delta\":{\"type\":\"text_delta\",\"text\":\"text\"}}"
            )
            .unwrap(),
            content_block_delta
        );
    }

    #[test]
    fn default_content_block_stop() {
        assert_eq!(
            ContentBlockStopChunk::default(),
            ContentBlockStopChunk {
                _type: MessageChunkType::ContentBlockStop,
                index: Default::default(),
            }
        );
    }

    #[test]
    fn display_content_block_stop() {
        let content_block_stop = ContentBlockStopChunk {
            _type: MessageChunkType::ContentBlockStop,
            index: 1,
        };
        assert_eq!(
            content_block_stop.to_string(),
            "{\n  \"type\": \"content_block_stop\",\n  \"index\": 1\n}"
        );
    }

    #[test]
    fn serialize_content_block_stop() {
        let content_block_stop = ContentBlockStopChunk {
            _type: MessageChunkType::ContentBlockStop,
            index: 1,
        };
        assert_eq!(
            serde_json::to_string(&content_block_stop).unwrap(),
            r#"{"type":"content_block_stop","index":1}"#
        );
    }

    #[test]
    fn deserialize_content_block_stop() {
        let content_block_stop = ContentBlockStopChunk {
            _type: MessageChunkType::ContentBlockStop,
            index: 1,
        };
        assert_eq!(
            serde_json::from_str::<ContentBlockStopChunk>(
                r#"{"type":"content_block_stop","index":1}"#
            )
            .unwrap(),
            content_block_stop
        );
    }

    #[test]
    fn default_message_delta() {
        assert_eq!(
            MessageDeltaChunk::default(),
            MessageDeltaChunk {
                _type: MessageChunkType::MessageDelta,
                delta: Default::default(),
                usage: Default::default(),
            }
        );
    }

    #[test]
    fn display_message_delta() {
        let message_delta = MessageDeltaChunk {
            _type: MessageChunkType::MessageDelta,
            delta: StreamStop {
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
            },
            usage: DeltaUsage {
                output_tokens: 1,
            },
        };
        assert_eq!(
            message_delta.to_string(),
            "{\n  \"type\": \"message_delta\",\n  \"delta\": {\n    \"stop_reason\": \"end_turn\",\n    \"stop_sequence\": \"stop_sequence\"\n  },\n  \"usage\": {\n    \"output_tokens\": 1\n  }\n}"
        );
    }

    #[test]
    fn serialize_message_delta() {
        let message_delta = MessageDeltaChunk {
            _type: MessageChunkType::MessageDelta,
            delta: StreamStop {
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
            },
            usage: DeltaUsage {
                output_tokens: 1,
            },
        };
        assert_eq!(
            serde_json::to_string(&message_delta).unwrap(),
            r#"{"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":"stop_sequence"},"usage":{"output_tokens":1}}"#
        );
    }

    #[test]
    fn deserialize_message_delta() {
        let message_delta = MessageDeltaChunk {
            _type: MessageChunkType::MessageDelta,
            delta: StreamStop {
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
            },
            usage: DeltaUsage {
                output_tokens: 1,
            },
        };
        assert_eq!(
            serde_json::from_str::<MessageDeltaChunk>(
                r#"{"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":"stop_sequence"},"usage":{"output_tokens":1}}"#
            )
            .unwrap(),
            message_delta
        );
    }

    #[test]
    fn default_message_stop() {
        assert_eq!(
            MessageStopChunk::default(),
            MessageStopChunk {
                _type: MessageChunkType::MessageStop,
            }
        );
    }

    #[test]
    fn display_message_stop() {
        let message_stop = MessageStopChunk::default();
        assert_eq!(
            message_stop.to_string(),
            "{\n  \"type\": \"message_stop\"\n}"
        );
    }

    #[test]
    fn serialize_message_stop() {
        let message_stop = MessageStopChunk::default();
        assert_eq!(
            serde_json::to_string(&message_stop).unwrap(),
            r#"{"type":"message_stop"}"#
        );
    }

    #[test]
    fn deserialize_message_stop() {
        let message_stop = MessageStopChunk::default();
        assert_eq!(
            serde_json::from_str::<MessageStopChunk>(
                r#"{"type":"message_stop"}"#
            )
            .unwrap(),
            message_stop
        );
    }

    #[test]
    fn display_stream_chunk() {
        let message_start = MessageStartChunk {
            _type: MessageChunkType::MessageStart,
            message: MessagesResponseBody {
                id: "id".to_string(),
                _type: MessageObjectType::Message,
                role: Role::Assistant,
                content: "content".into(),
                model: ClaudeModel::Claude3Sonnet20240229,
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
                usage: Usage {
                    input_tokens: 1,
                    output_tokens: 2,
                },
            },
        };
        let content_block_start = ContentBlockStartChunk {
            _type: MessageChunkType::ContentBlockStart,
            index: 1,
            content_block: ContentBlockStart::TextContentBlock(TextContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        let ping = PingChunk::default();
        let content_block_delta = ContentBlockDeltaChunk {
            _type: MessageChunkType::ContentBlockDelta,
            index: 1,
            delta: ContentBlockDelta::TextDeltaContentBlock(TextDeltaContentBlock {
                text: "text".to_string(),
                ..Default::default()
            }),
        };
        let content_block_stop = ContentBlockStopChunk {
            _type: MessageChunkType::ContentBlockStop,
            index: 1,
        };
        let message_delta = MessageDeltaChunk {
            _type: MessageChunkType::MessageDelta,
            delta: StreamStop {
                stop_reason: Some(StopReason::EndTurn),
                stop_sequence: Some(StopSequence::new("stop_sequence")),
            },
            usage: DeltaUsage {
                output_tokens: 1,
            },
        };
        let message_stop = MessageStopChunk::default();

        assert_eq!(
            MessageChunk::MessageStart(message_start).to_string(),
            "event: message_start\ndata: {\"type\": \"message_start\", \"message\": {\"id\": \"id\", \"type\": \"message\", \"role\": \"assistant\", \"content\": \"content\", \"model\": \"claude-3-sonnet-20240229\", \"stop_reason\": \"end_turn\", \"stop_sequence\": \"stop_sequence\", \"usage\": {\"input_tokens\": 1, \"output_tokens\": 2}}}"
        );

        assert_eq!(
            MessageChunk::ContentBlockStart(content_block_start).to_string(),
            "event: content_block_start\ndata: {\"type\": \"content_block_start\", \"index\": 1, \"content_block\": {\"type\": \"text\", \"text\": \"text\"}}"
        );

        assert_eq!(
            MessageChunk::Ping(ping).to_string(),
            "event: ping\ndata: {\"type\": \"ping\"}",
        );

        assert_eq!(
            MessageChunk::ContentBlockDelta(content_block_delta).to_string(),
            "event: content_block_delta\ndata: {\"type\": \"content_block_delta\", \"index\": 1, \"delta\": {\"type\": \"text_delta\", \"text\": \"text\"}}"
        );

        assert_eq!(
            MessageChunk::ContentBlockStop(content_block_stop).to_string(),
            "event: content_block_stop\ndata: {\"type\": \"content_block_stop\", \"index\": 1}"
        );

        assert_eq!(
            MessageChunk::MessageDelta(message_delta).to_string(),
            "event: message_delta\ndata: {\"type\": \"message_delta\", \"delta\": {\"stop_reason\": \"end_turn\", \"stop_sequence\": \"stop_sequence\"}, \"usage\": {\"output_tokens\": 1}}"
        );

        assert_eq!(
            MessageChunk::MessageStop(message_stop).to_string(),
            "event: message_stop\ndata: {\"type\": \"message_stop\"}"
        );
    }

    #[test]
    fn parse_stream_chunk() {
        assert_eq!(
            MessageChunk::parse(
                r#"event: message_start
data: {"type": "message_start", "message": {"id": "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY", "type": "message", "role": "assistant", "content": [], "model": "claude-3-opus-20240229", "stop_reason": null, "stop_sequence": null, "usage": {"input_tokens": 25, "output_tokens": 1}}}"#
            )
            .unwrap(),
            MessageChunk::MessageStart(MessageStartChunk {
                _type: MessageChunkType::MessageStart,
                message: MessagesResponseBody {
                    id: "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY".to_string(),
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
                },
            })
        );

        assert_eq!(
            MessageChunk::parse(r#"event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}"#).unwrap(),
            MessageChunk::ContentBlockStart(ContentBlockStartChunk {
                _type: MessageChunkType::ContentBlockStart,
                index: 0,
                content_block: ContentBlockStart::TextContentBlock(TextContentBlock {
                    text: "".to_string(),
                    ..Default::default()
                }),
            })
        );

        assert_eq!(
            MessageChunk::parse(
                r#"event: ping
data: {"type": "ping"}"#
            )
            .unwrap(),
            MessageChunk::Ping(PingChunk::default())
        );

        assert_eq!(
            MessageChunk::parse(
                r#"event: content_block_delta
data: {"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": "Hello"}}"#
            )
            .unwrap(),
            MessageChunk::ContentBlockDelta(ContentBlockDeltaChunk {
                _type: MessageChunkType::ContentBlockDelta,
                index: 0,
                delta: ContentBlockDelta::TextDeltaContentBlock(TextDeltaContentBlock {
                    text: "Hello".to_string(),
                    ..Default::default()
                }),
            })
        );

        assert_eq!(
            MessageChunk::parse(
                r#"event: content_block_stop
data: {"type": "content_block_stop", "index": 0}"#
            )
            .unwrap(),
            MessageChunk::ContentBlockStop(ContentBlockStopChunk {
                _type: MessageChunkType::ContentBlockStop,
                index: 0,
            })
        );

        assert_eq!(
            MessageChunk::parse(
               r#"event: message_delta
data: {"type": "message_delta", "delta": {"stop_reason": "end_turn", "stop_sequence": null}, "usage": {"output_tokens": 15}}"#
            )
            .unwrap(),
            MessageChunk::MessageDelta(MessageDeltaChunk {
                _type: MessageChunkType::MessageDelta,
                delta: StreamStop {
                    stop_reason: Some(StopReason::EndTurn),
                    stop_sequence: None,
                },
                usage: DeltaUsage {
                    output_tokens: 15,
                },
            })
        );

        assert_eq!(
            MessageChunk::parse(
                r#"event: message_stop
data: {"type": "message_stop"}"#
            )
            .unwrap(),
            MessageChunk::MessageStop(MessageStopChunk::default())
        );

        assert!(MessageChunk::parse("event: unknown\ndata: {}").is_err());
    }

    #[test]
    fn new_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            text_delta_content_block,
            TextDeltaContentBlock {
                _type: ContentType::TextDelta,
                text: "text".to_string(),
            }
        );
    }

    #[test]
    fn default_text_delta_content_block() {
        assert_eq!(
            TextDeltaContentBlock::default(),
            TextDeltaContentBlock {
                _type: ContentType::TextDelta,
                text: String::new(),
            }
        );
    }

    #[test]
    fn display_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            text_delta_content_block.to_string(),
            "{\n  \"type\": \"text_delta\",\n  \"text\": \"text\"\n}"
        );
    }

    #[test]
    fn serialize_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::to_string(&text_delta_content_block).unwrap(),
            "{\"type\":\"text_delta\",\"text\":\"text\"}"
        );
    }

    #[test]
    fn deserialize_text_delta_content_block() {
        let text_delta_content_block =
            TextDeltaContentBlock::new("text".to_string());
        assert_eq!(
            serde_json::from_str::<TextDeltaContentBlock>(
                "{\"type\":\"text_delta\",\"text\":\"text\"}"
            )
            .unwrap(),
            text_delta_content_block
        );
    }

    #[test]
    fn from_text_delta_content_block() {
        assert_eq!(
            TextDeltaContentBlock::from("text"),
            TextDeltaContentBlock::new("text")
        );

        let content_block: TextDeltaContentBlock = "text".into();
        assert_eq!(
            content_block,
            TextDeltaContentBlock::new("text")
        );
    }
}
