// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

#![allow(missing_docs)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use nostr::key::XOnlyPublicKey;
use nostr::url::Url;
use nostr::{ClientMessage, Contact, Event, EventId, Keys, Metadata, SubscriptionFilter, Tag};
use tokio::sync::broadcast;

use super::{Error, Options};
use crate::client::Entity;
use crate::relay::pool::RelayPoolNotification;
use crate::relay::Relay;
use crate::RUNTIME;

#[derive(Debug, Clone)]
pub struct Client {
    client: super::Client,
}

impl Client {
    pub fn new(keys: &Keys) -> Self {
        Self {
            client: super::Client::new(keys),
        }
    }

    pub fn new_with_opts(keys: &Keys, opts: Options) -> Self {
        Self {
            client: super::Client::new_with_opts(keys, opts),
        }
    }

    /// Get current [`Keys`]
    pub fn keys(&self) -> Keys {
        self.client.keys()
    }

    pub fn notifications(&self) -> broadcast::Receiver<RelayPoolNotification> {
        self.client.notifications()
    }

    /// Get relays
    pub fn relays(&self) -> HashMap<Url, Relay> {
        RUNTIME.block_on(async { self.client.relays().await })
    }

    /// Add multiple relays
    pub fn add_relays<S>(&self, relays: Vec<(S, Option<SocketAddr>)>) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.add_relays(relays).await })
    }

    pub fn add_relay<S>(&self, url: S, proxy: Option<SocketAddr>) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.add_relay(url, proxy).await })
    }

    pub fn remove_relay<S>(&self, url: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.remove_relay(url).await })
    }

    pub fn connect_relay<S>(&self, url: S, wait_for_connection: bool) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.connect_relay(url, wait_for_connection).await })
    }

    pub fn disconnect_relay<S>(&self, url: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.disconnect_relay(url).await })
    }

    pub fn connect(&self) {
        RUNTIME.block_on(async {
            self.client.connect().await;
        })
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        RUNTIME.block_on(async { self.client.disconnect().await })
    }

    pub fn subscribe(&self, filters: Vec<SubscriptionFilter>) -> Result<(), Error> {
        RUNTIME.block_on(async { self.client.subscribe(filters).await })
    }

    pub fn get_events_of(&self, filters: Vec<SubscriptionFilter>) -> Result<Vec<Event>, Error> {
        RUNTIME.block_on(async { self.client.get_events_of(filters).await })
    }

    pub fn req_events_of(&self, filters: Vec<SubscriptionFilter>, timeout: Duration) {
        RUNTIME.block_on(async {
            self.client.req_events_of(filters, timeout).await;
        })
    }

    #[deprecated]
    pub fn send_client_msg(&self, msg: ClientMessage, wait: bool) -> Result<(), Error> {
        #[allow(deprecated)]
        RUNTIME.block_on(async { self.client.send_client_msg(msg, wait).await })
    }

    pub fn send_msg(&self, msg: ClientMessage) -> Result<(), Error> {
        RUNTIME.block_on(async { self.client.send_msg(msg).await })
    }

    pub fn send_msg_to<S>(&self, url: S, msg: ClientMessage) -> Result<(), Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.send_msg_to(url, msg).await })
    }

    /// Send event
    pub fn send_event(&self, event: Event) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.send_event(event).await })
    }

    pub fn send_event_to<S>(&self, url: S, event: Event) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.send_event_to(url, event).await })
    }

    pub fn update_profile(&self, metadata: Metadata) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.update_profile(metadata).await })
    }

    pub fn publish_text_note<S>(&self, content: S, tags: &[Tag]) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.publish_text_note(content, tags).await })
    }

    #[cfg(feature = "nip13")]
    pub fn publish_pow_text_note<S>(
        &self,
        content: S,
        tags: &[Tag],
        difficulty: u8,
    ) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async {
            self.client
                .publish_pow_text_note(content, tags, difficulty)
                .await
        })
    }

    pub fn add_recommended_relay<S>(&self, url: S) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.add_recommended_relay(url).await })
    }

    pub fn set_contact_list(&self, list: Vec<Contact>) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.set_contact_list(list).await })
    }

    pub fn get_contact_list(&self) -> Result<Vec<Contact>, Error> {
        RUNTIME.block_on(async { self.client.get_contact_list().await })
    }

    #[cfg(feature = "nip04")]
    pub fn send_direct_msg<S>(&self, receiver: XOnlyPublicKey, msg: S) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.send_direct_msg(receiver, msg).await })
    }

    pub fn repost_event(
        &self,
        event_id: EventId,
        public_key: XOnlyPublicKey,
    ) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.repost_event(event_id, public_key).await })
    }

    pub fn delete_event<S>(&self, event_id: EventId, reason: Option<S>) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.delete_event(event_id, reason).await })
    }

    pub fn like(&self, event_id: EventId, public_key: XOnlyPublicKey) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.like(event_id, public_key).await })
    }

    pub fn dislike(&self, event_id: EventId, public_key: XOnlyPublicKey) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.dislike(event_id, public_key).await })
    }

    pub fn reaction<S>(
        &self,
        event_id: EventId,
        public_key: XOnlyPublicKey,
        content: S,
    ) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.reaction(event_id, public_key, content).await })
    }

    pub fn new_channel(&self, metadata: Metadata) -> Result<EventId, Error> {
        RUNTIME.block_on(async { self.client.new_channel(metadata).await })
    }

    pub fn update_channel(
        &self,
        channel_id: EventId,
        relay_url: Option<Url>,
        metadata: Metadata,
    ) -> Result<EventId, Error> {
        RUNTIME.block_on(async {
            self.client
                .update_channel(channel_id, relay_url, metadata)
                .await
        })
    }

    pub fn send_channel_msg<S>(
        &self,
        channel_id: EventId,
        relay_url: Option<Url>,
        msg: S,
    ) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async {
            self.client
                .send_channel_msg(channel_id, relay_url, msg)
                .await
        })
    }

    pub fn hide_channel_msg<S>(
        &self,
        message_id: EventId,
        reason: Option<S>,
    ) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.hide_channel_msg(message_id, reason).await })
    }

    pub fn mute_channel_user<S>(
        &self,
        pubkey: XOnlyPublicKey,
        reason: Option<S>,
    ) -> Result<EventId, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.mute_channel_user(pubkey, reason).await })
    }

    pub fn get_channels(&self) -> Result<Vec<Event>, Error> {
        RUNTIME.block_on(async { self.client.get_channels().await })
    }

    pub fn get_entity_of<S>(&self, entity: S) -> Result<Entity, Error>
    where
        S: Into<String>,
    {
        RUNTIME.block_on(async { self.client.get_entity_of(entity).await })
    }

    pub fn handle_notifications<F>(&self, func: F) -> Result<(), Error>
    where
        F: Fn(RelayPoolNotification) -> Result<(), Error>,
    {
        RUNTIME.block_on(async { self.client.handle_notifications(func).await })
    }
}
