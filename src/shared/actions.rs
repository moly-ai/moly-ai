use makepad_widgets::{ActionDefaultRef, DefaultNone};
use moly_kit::prelude::*;
use moly_protocol::data::FileId;

use crate::data::chats::chat::ChatId;

#[derive(Clone, DefaultNone, Debug)]
pub enum ChatAction {
    // Start a new chat, no entity specified
    StartWithoutEntity,
    // Start a new chat with a given entity
    Start(BotId),
    // Select a chat from the chat history
    ChatSelected(ChatId),
    None,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum DownloadAction {
    Play(FileId),
    Pause(FileId),
    Cancel(FileId),
    None,
}
