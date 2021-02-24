use matrix_sdk::{
    events::{
        room::{
            member::MemberEventContent,
            message::{MessageEventContent, TextMessageEventContent},
        },
        AnyMessageEventContent, StrippedStateEvent, SyncMessageEvent,
    },
    Client, EventHandler, RoomState,
};

use crate::cfg::BotConfig;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

pub struct MyEventHandler {
    pub client: Client,
    pub config: BotConfig,
}

impl MyEventHandler {
    pub fn new(client: Client, config: BotConfig) -> Self {
        Self { client, config }
    }
}

impl MyEventHandler {
    async fn handle_message(
        &self,
        state: RoomState,
        event: &SyncMessageEvent<MessageEventContent>,
    ) -> anyhow::Result<()> {
        if let Some(room) = state.joined() {
            let msg_body = if let SyncMessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent { body: msg_body, .. }),
                ..
            } = event
            {
                msg_body.clone()
            } else {
                String::new()
            };

            match msg_body.to_ascii_lowercase().as_str() {
                "ping" => {}
                _ => {
                    let content = AnyMessageEventContent::RoomMessage(
                        MessageEventContent::text_plain("pong"),
                    );

                    self.client.room_send(room.room_id(), content, None).await?;
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for MyEventHandler {
    async fn on_room_message(
        &self,
        state: RoomState,
        event: &SyncMessageEvent<MessageEventContent>,
    ) {
        if let Err(err) = self.handle_message(state, event).await {
            error!("Error handling message {}", err);
        }
    }

    async fn on_stripped_state_member(
        &self,
        room: RoomState,
        room_member: &StrippedStateEvent<MemberEventContent>,
        _: Option<MemberEventContent>,
    ) {
        if room_member.state_key != self.client.user_id().await.unwrap() {
            return;
        }

        if let RoomState::Invited(room) = room {
            info!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = self.client.join_room_by_id(room.room_id()).await {
                error!(
                    "Failed to join room {} ({:?}), retrying in {}s",
                    room.room_id(),
                    err,
                    delay
                );

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    error!("Can't join room {} ({:?})", room.room_id(), err);
                    break;
                }
            }
            info!("Successfully joined room {}", room.room_id());
        }
    }
}
