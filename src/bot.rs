use std::{convert::TryFrom, time::Duration};

use crate::{cfg::BotConfig, handler::MyEventHandler};
use anyhow::Result;
use matrix_sdk::{
    deserialized_responses::SyncResponse, identifiers::UserId, Client, ClientConfig, LoopCtrl,
    Session, SyncSettings,
};
use url::Url;

use tracing::{debug, error, info};

pub async fn run(config: BotConfig) -> Result<()> {
    let client_config = ClientConfig::default();

    let homeserver_url = Url::parse(config.server.url.as_str())?;

    let domain = homeserver_url.domain().unwrap().to_owned();

    let client = Client::new_with_config(homeserver_url, client_config)?;

    match (
        &config.credentials.access_token,
        &config.credentials.password,
    ) {
        (Some(token), _) => {
            client
                .restore_login(Session {
                    access_token: token.clone(),
                    user_id: UserId::try_from(format!(
                        "@{}:{}",
                        config.credentials.username, domain
                    ))?,
                    device_id: config.credentials.device_id.as_str().into(),
                })
                .await?;
        }
        (None, Some(password)) => {
            let response = client
                .login(
                    config.credentials.username.as_str(),
                    password,
                    Some(config.credentials.device_id.as_str()),
                    Some(config.credentials.device_name.as_str()),
                )
                .await?;

            debug!(
                "TestBot user_id: {} and access_token: {}",
                response.user_id, response.access_token
            );
        }
        _ => {
            error!("Missing token or password from the config file.");
            return Err(anyhow::anyhow!(
                "Missing token or password from the config file."
            ));
        }
    }

    info!(
        "🚀 TestBot connected to server: {} with username: {}",
        config.server.url, config.credentials.username
    );

    let sync_settings = SyncSettings::new().timeout(Duration::from_secs(config.sync.interval));

    client.sync_once(SyncSettings::default()).await?;

    client
        .set_event_handler(Box::new(MyEventHandler::new(
            client.clone(),
            config.clone(),
        )))
        .await;

    client.sync_with_callback(sync_settings, callback).await;

    Ok(())
}

async fn callback(response: SyncResponse) -> LoopCtrl {
    info!("{:?}", response);
    LoopCtrl::Continue
}
