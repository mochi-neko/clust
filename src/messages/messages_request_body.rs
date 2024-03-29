use crate::macros::impl_display_for_serialize;
use crate::messages::{
    ClaudeModel, MaxTokens, Message, Metadata, StopSequence, StreamOption,
    SystemPrompt, Temperature, TopK, TopP,
};

/// The request body for the Messages API.
///
/// See also [the messages API reference](https://docs.anthropic.com/claude/reference/messages_post).
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct MessagesRequestBody {
    /// The model that will complete your prompt.
    ///
    /// See [models](https://docs.anthropic.com/claude/docs/models-overview) for additional details and options.
    pub model: ClaudeModel,
    /// Input messages.
    ///
    /// Our models are trained to operate on alternating user and assistant conversational turns. When creating a new Message, you specify the prior conversational turns with the messages parameter, and the model then generates the next Message in the conversation.
    ///
    /// See [examples](https://docs.anthropic.com/claude/reference/messages-examples) for more input examples.
    ///
    /// Note that if you want to include a [system prompt](https://docs.anthropic.com/claude/docs/system-prompts), you can use the top-level system parameter — there is no "system" role for input messages in the Messages API.
    pub messages: Vec<Message>,
    /// System prompt.
    ///
    /// A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role. See our [guide to system prompts](https://docs.anthropic.com/claude/docs/system-prompts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPrompt>,
    /// The maximum number of tokens to generate before stopping.
    ///
    /// Note that our models may stop before reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
    ///
    /// Different models have different maximum values for this parameter. See [models](https://docs.anthropic.com/claude/docs/models-overview) for details.
    pub max_tokens: MaxTokens,
    /// An object describing metadata about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Custom text sequences that will cause the model to stop generating.
    ///
    /// Our models will normally stop when they have naturally completed their turn, which will result in a response stop_reason of "end_turn".
    ///
    /// If you want the model to stop generating when it encounters custom strings of text, you can use the stop_sequences parameter. If the model encounters one of the custom sequences, the response stop_reason value will be "stop_sequence" and the response stop_sequence value will contain the matched stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<StopSequence>>,
    /// Whether to incrementally stream the response using server-sent events.
    ///
    /// See [streaming](https://docs.anthropic.com/claude/reference/messages-streaming) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<StreamOption>,
    /// Amount of randomness injected into the response.
    ///
    /// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to 0.0 for analytical / multiple choice, and closer to 1.0 for creative and generative tasks.
    ///
    /// Note that even with temperature of 0.0, the results will not be fully deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    /// Use nucleus sampling.
    ///
    /// In nucleus sampling, we compute the cumulative distribution over all the options for each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by top_p. You should either alter temperature or top_p, but not both.
    ///
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,
    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses. [Learn more technical details here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
    ///
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<TopK>,
}

impl_display_for_serialize!(MessagesRequestBody);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let messages_request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            messages: vec![],
            max_tokens: MaxTokens::new(16, ClaudeModel::Claude3Sonnet20240229)
                .unwrap(),
            ..Default::default()
        };
        assert_eq!(
            messages_request_body.model,
            ClaudeModel::Claude3Sonnet20240229
        );
        assert_eq!(messages_request_body.messages, vec![]);
        assert_eq!(messages_request_body.system, None);
        assert_eq!(
            messages_request_body.max_tokens,
            MaxTokens::new(16, ClaudeModel::Claude3Sonnet20240229).unwrap()
        );
        assert_eq!(messages_request_body.metadata, None);
        assert_eq!(
            messages_request_body.stop_sequences,
            None
        );
        assert_eq!(messages_request_body.stream, None);
        assert_eq!(messages_request_body.temperature, None);
        assert_eq!(messages_request_body.top_p, None);
        assert_eq!(messages_request_body.top_k, None);
    }

    #[test]
    fn default() {
        let messages_request_body = MessagesRequestBody::default();
        assert_eq!(
            messages_request_body.model,
            ClaudeModel::default()
        );
        assert_eq!(messages_request_body.messages, vec![]);
        assert_eq!(messages_request_body.system, None);
        assert_eq!(
            messages_request_body.max_tokens,
            MaxTokens::default()
        );
        assert_eq!(messages_request_body.metadata, None);
        assert_eq!(
            messages_request_body.stop_sequences,
            None
        );
        assert_eq!(messages_request_body.stream, None);
        assert_eq!(messages_request_body.temperature, None);
        assert_eq!(messages_request_body.top_p, None);
        assert_eq!(messages_request_body.top_k, None);
    }

    #[test]
    fn display() {
        let messages_request_body = MessagesRequestBody::default();
        assert_eq!(
            messages_request_body.to_string(),
            "{\n  \"model\": \"claude-3-sonnet-20240229\",\n  \"messages\": [],\n  \"max_tokens\": 4096\n}"
        );
    }

    #[test]
    fn serialize() {
        let messages_request_body = MessagesRequestBody::default();
        assert_eq!(
            serde_json::to_string(&messages_request_body).unwrap(),
            "{\"model\":\"claude-3-sonnet-20240229\",\"messages\":[],\"max_tokens\":4096}"
        );

        let messages_request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            messages: vec![],
            max_tokens: MaxTokens::new(16, ClaudeModel::Claude3Sonnet20240229)
                .unwrap(),
            system: Some(SystemPrompt::new("system-prompt")),
            metadata: Some(Metadata {
                user_id: "metadata".into(),
            }),
            stop_sequences: Some(vec![StopSequence::new(
                "stop-sequence",
            )]),
            stream: Some(StreamOption::ReturnOnce),
            temperature: Some(Temperature::new(0.5).unwrap()),
            top_p: Some(TopP::new(0.5).unwrap()),
            top_k: Some(TopK::new(50)),
        };
        assert_eq!(
            serde_json::to_string(&messages_request_body).unwrap(),
            "{\"model\":\"claude-3-sonnet-20240229\",\"messages\":[],\"system\":\"system-prompt\",\"max_tokens\":16,\"metadata\":{\"user_id\":\"metadata\"},\"stop_sequences\":[\"stop-sequence\"],\"stream\":false,\"temperature\":0.5,\"top_p\":0.5,\"top_k\":50}"
        );
    }

    #[test]
    fn deserialize() {
        let messages_request_body = MessagesRequestBody::default();
        assert_eq!(
            serde_json::from_str::<MessagesRequestBody>("{\"model\":\"claude-3-sonnet-20240229\",\"messages\":[],\"max_tokens\":4096}").unwrap(),
            messages_request_body
        );

        let messages_request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            messages: vec![],
            max_tokens: MaxTokens::new(16, ClaudeModel::Claude3Sonnet20240229)
                .unwrap(),
            system: Some(SystemPrompt::new("system-prompt")),
            metadata: Some(Metadata {
                user_id: "metadata".into(),
            }),
            stop_sequences: Some(vec![StopSequence::new(
                "stop-sequence",
            )]),
            stream: Some(StreamOption::ReturnOnce),
            temperature: Some(Temperature::new(0.5).unwrap()),
            top_p: Some(TopP::new(0.5).unwrap()),
            top_k: Some(TopK::new(50)),
        };
        assert_eq!(
            serde_json::from_str::<MessagesRequestBody>("{\"model\":\"claude-3-sonnet-20240229\",\"messages\":[],\"system\":\"system-prompt\",\"max_tokens\":16,\"metadata\":{\"user_id\":\"metadata\"},\"stop_sequences\":[\"stop-sequence\"],\"stream\":false,\"temperature\":0.5,\"top_p\":0.5,\"top_k\":50}").unwrap(),
            messages_request_body
        );
    }
}
