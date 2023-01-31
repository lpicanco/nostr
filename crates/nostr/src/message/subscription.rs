// Copyright (c) 2021 Paul Miller
// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

//! Subscription filters

#![allow(missing_docs)]

use bitcoin::hashes::sha256::Hash as Sha256Hash;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::XOnlyPublicKey;
use serde::{Deserialize, Serialize};

use crate::{EventId, Kind, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SubscriptionId(String);

impl SubscriptionId {
    pub fn new<S>(id: S) -> Self
    where
        S: Into<String>,
    {
        Self(id.into())
    }

    /// Generate new random [`SubscriptionId`]
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl ToString for SubscriptionId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<XOnlyPublicKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<Kind>>,
    #[serde(rename = "#e")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventId>>,
    #[serde(rename = "#p")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<XOnlyPublicKey>>,
    #[serde(rename = "#t")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<String>>,
    #[serde(rename = "#r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Default for SubscriptionFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionFilter {
    pub fn new() -> Self {
        Self {
            ids: None,
            kinds: None,
            events: None,
            pubkeys: None,
            hashtags: None,
            references: None,
            search: None,
            since: None,
            until: None,
            authors: None,
            limit: None,
        }
    }

    /// Set subscription id
    pub fn id(self, id: impl Into<String>) -> Self {
        Self {
            ids: Some(vec![id.into()]),
            ..self
        }
    }

    /// Set subscription ids
    pub fn ids(self, ids: impl Into<Vec<String>>) -> Self {
        Self {
            ids: Some(ids.into()),
            ..self
        }
    }

    /// Set author
    pub fn author(self, author: XOnlyPublicKey) -> Self {
        Self {
            authors: Some(vec![author]),
            ..self
        }
    }

    /// Set authors
    pub fn authors(self, authors: Vec<XOnlyPublicKey>) -> Self {
        Self {
            authors: Some(authors),
            ..self
        }
    }

    /// Set kind
    pub fn kind(self, kind: Kind) -> Self {
        Self {
            kinds: Some(vec![kind]),
            ..self
        }
    }

    /// Set kinds
    pub fn kinds(self, kinds: Vec<Kind>) -> Self {
        Self {
            kinds: Some(kinds),
            ..self
        }
    }

    /// Set event
    pub fn event(self, id: EventId) -> Self {
        Self {
            events: Some(vec![id]),
            ..self
        }
    }

    /// Set events
    pub fn events(self, ids: Vec<EventId>) -> Self {
        Self {
            events: Some(ids),
            ..self
        }
    }

    /// Set pubkey
    pub fn pubkey(self, pubkey: XOnlyPublicKey) -> Self {
        Self {
            pubkeys: Some(vec![pubkey]),
            ..self
        }
    }

    /// Set pubkeys
    pub fn pubkeys(self, pubkeys: Vec<XOnlyPublicKey>) -> Self {
        Self {
            pubkeys: Some(pubkeys),
            ..self
        }
    }

    /// Set hashtag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/12.md>
    pub fn hashtag(self, hashtag: impl Into<String>) -> Self {
        Self {
            hashtags: Some(vec![hashtag.into()]),
            ..self
        }
    }

    /// Set hashtags
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/12.md>
    pub fn hashtags(self, hashtags: impl Into<Vec<String>>) -> Self {
        Self {
            hashtags: Some(hashtags.into()),
            ..self
        }
    }

    /// Set reference
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/12.md>
    pub fn reference(self, v: impl Into<String>) -> Self {
        Self {
            references: Some(vec![v.into()]),
            ..self
        }
    }

    /// Set references
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/12.md>
    pub fn references(self, v: impl Into<Vec<String>>) -> Self {
        Self {
            references: Some(v.into()),
            ..self
        }
    }

    /// Set search field
    pub fn search<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            search: Some(value.into()),
            ..self
        }
    }

    /// Set since unix timestamp
    pub fn since(self, since: Timestamp) -> Self {
        Self {
            since: Some(since),
            ..self
        }
    }

    /// Set until unix timestamp
    pub fn until(self, until: Timestamp) -> Self {
        Self {
            until: Some(until),
            ..self
        }
    }

    /// Set limit
    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}
