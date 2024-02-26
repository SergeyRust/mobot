use mobot_derive::BotRequest;
use serde::{Deserialize, Serialize};
use crate::API;
use crate::api::{InlineKeyboardMarkup, ReplyMarkup, SendChatActionRequest};


/*
    chat_id	Integer	Yes	Unique identifier for the target chat
    message_thread_id	Integer	Optional	Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    game_short_name	String	Yes	Short name of the game, serves as the unique identifier for the game. Set up your games via @BotFather.
    disable_notification	Boolean	Optional	Sends the message silently. Users will receive a notification with no sound.
    protect_content	Boolean	Optional	Protects the contents of the sent message from forwarding and saving
    reply_parameters	ReplyParameters	Optional	Description of the message to reply to
    reply_markup	InlineKeyboardMarkup	Optional	A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
 */
#[derive(Default, Debug, Clone, Serialize, Deserialize, BotRequest)]
pub struct SendGameRequest {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    //pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup_game: Option<ReplyMarkup>,
}

impl SendGameRequest {
    pub fn new(
        chat_id: i64,
        game_short_name: String,
        reply_markup: Option<ReplyMarkup>) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            game_short_name,
            disable_notification: Some(false),
            protect_content: Some(false),
            reply_markup_game: reply_markup
        }
    }
}

impl API {
    /// Send game.
    pub async fn send_game_request(&self, req: &SendGameRequest) -> anyhow::Result<bool> {
        self.client.post("sendGame", req).await
    }
}

