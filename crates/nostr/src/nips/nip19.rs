// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

//! NIP19
//!
//! https://github.com/nostr-protocol/nips/blob/master/19.md

#![allow(missing_docs)]

use bitcoin::bech32::{self, FromBase32, ToBase32, Variant};
use bitcoin::secp256k1::{SecretKey, XOnlyPublicKey};
#[cfg(feature = "base")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "base")]
use crate::event::id::{self, EventId};
#[cfg(feature = "base")]
use crate::Profile;

pub const PREFIX_BECH32_SECRET_KEY: &str = "nsec";
pub const PREFIX_BECH32_PUBLIC_KEY: &str = "npub";
pub const PREFIX_BECH32_NOTE_ID: &str = "note";
pub const PREFIX_BECH32_PROFILE: &str = "nprofile";
pub const PREFIX_BECH32_EVENT: &str = "nevent";

/// `NIP19` error
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// Bech32 error.
    #[error(transparent)]
    Bech32(#[from] bech32::Error),
    /// Invalid bec32 secret key
    #[error("Invalid bech32 secret key")]
    Bech32SkParseError,
    /// Invalid bec32 public key
    #[error("Invalid bech32 public key")]
    Bech32PkParseError,
    /// Invalid bec32 note id
    #[error("Invalid bech32 note id")]
    Bech32NoteParseError,
    /// Invalid bec32 profile
    #[error("Invalid bech32 profile")]
    Bech32ProfileParseError,
    /// Invalid bec32 event
    #[error("Invalid bech32 event")]
    Bech32EventParseError,
    /// Secp256k1 error
    #[error(transparent)]
    Secp256k1(#[from] bitcoin::secp256k1::Error),
    /// Hash error
    #[error(transparent)]
    Hash(#[from] bitcoin::hashes::Error),
    /// EventId error
    #[cfg(feature = "base")]
    #[error(transparent)]
    EventId(#[from] id::Error),
}

pub trait FromBech32: Sized {
    type Err;
    fn from_bech32<S>(s: S) -> Result<Self, Self::Err>
    where
        S: Into<String>;
}

impl FromBech32 for SecretKey {
    type Err = Error;
    fn from_bech32<S>(secret_key: S) -> Result<Self, Self::Err>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&secret_key.into()).map_err(|_| Error::Bech32SkParseError)?;

        if hrp != PREFIX_BECH32_SECRET_KEY || checksum != Variant::Bech32 {
            return Err(Error::Bech32SkParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32SkParseError)?;
        SecretKey::from_slice(data.as_slice()).map_err(|_| Error::Bech32SkParseError)
    }
}

impl FromBech32 for XOnlyPublicKey {
    type Err = Error;
    fn from_bech32<S>(public_key: S) -> Result<Self, Self::Err>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&public_key.into()).map_err(|_| Error::Bech32PkParseError)?;

        if hrp != PREFIX_BECH32_PUBLIC_KEY || checksum != Variant::Bech32 {
            return Err(Error::Bech32PkParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32PkParseError)?;
        Ok(XOnlyPublicKey::from_slice(data.as_slice())?)
    }
}

#[cfg(feature = "base")]
impl FromBech32 for EventId {
    type Err = Error;
    fn from_bech32<S>(hash: S) -> Result<Self, Self::Err>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&hash.into()).map_err(|_| Error::Bech32NoteParseError)?;

        if hrp != PREFIX_BECH32_NOTE_ID || checksum != Variant::Bech32 {
            return Err(Error::Bech32NoteParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32NoteParseError)?;
        Ok(EventId::from_slice(data.as_slice())?)
    }
}

pub trait ToBech32 {
    type Err;
    fn to_bech32(&self) -> Result<String, Self::Err>;
}

impl ToBech32 for XOnlyPublicKey {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let data = self.serialize().to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_PUBLIC_KEY,
            data,
            Variant::Bech32,
        )?)
    }
}

impl ToBech32 for SecretKey {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let data = self.secret_bytes().to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_SECRET_KEY,
            data,
            Variant::Bech32,
        )?)
    }
}

// Note ID
#[cfg(feature = "base")]
impl ToBech32 for EventId {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let data = self.to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_NOTE_ID,
            data,
            Variant::Bech32,
        )?)
    }
}

#[cfg(feature = "base")]
impl FromBech32 for Profile {
    type Err = Error;
    fn from_bech32<S>(s: S) -> Result<Self, Self::Err>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&s.into()).map_err(|_| Error::Bech32ProfileParseError)?;

        if hrp != PREFIX_BECH32_PROFILE || checksum != Variant::Bech32 {
            return Err(Error::Bech32ProfileParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32ProfileParseError)?;

        let t = data.first().ok_or(Error::Bech32ProfileParseError)?;
        if *t != 0 {
            return Err(Error::Bech32ProfileParseError);
        }

        let l = data.get(1).ok_or(Error::Bech32ProfileParseError)?;
        if *l != 32 {
            return Err(Error::Bech32ProfileParseError);
        }

        let public_key = data.get(2..34).ok_or(Error::Bech32ProfileParseError)?;
        let public_key = XOnlyPublicKey::from_slice(public_key)?;

        let mut relays: Vec<String> = Vec::new();
        let mut relays_data: Vec<u8> = data
            .get(34..)
            .ok_or(Error::Bech32ProfileParseError)?
            .to_vec();

        while !relays_data.is_empty() {
            let t = relays_data.first().ok_or(Error::Bech32ProfileParseError)?;
            if *t != 1 {
                return Err(Error::Bech32ProfileParseError);
            }

            let l = relays_data.get(1).ok_or(Error::Bech32ProfileParseError)?;
            let l = *l as usize;

            let data = relays_data
                .get(2..l + 2)
                .ok_or(Error::Bech32ProfileParseError)?;

            relays.push(
                String::from_utf8(data.to_vec()).map_err(|_| Error::Bech32ProfileParseError)?,
            );
            relays_data.drain(..l + 2);
        }

        Ok(Self { public_key, relays })
    }
}

#[cfg(feature = "base")]
impl ToBech32 for Profile {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let mut bytes: Vec<u8> = vec![0, 32];
        bytes.extend(self.public_key.serialize());

        for relay in self.relays.iter() {
            bytes.extend([1, relay.len() as u8]);
            bytes.extend(relay.as_bytes());
        }

        let data = bytes.to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_PROFILE,
            data,
            Variant::Bech32,
        )?)
    }
}

#[cfg(feature = "base")]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Nip19Event {
    event_id: EventId,
    relays: Vec<String>,
}

#[cfg(feature = "base")]
impl Nip19Event {
    pub fn new<S>(event_id: EventId, relays: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            event_id,
            relays: relays.into_iter().map(|u| u.into()).collect(),
        }
    }
}

#[cfg(feature = "base")]
impl FromBech32 for Nip19Event {
    type Err = Error;
    fn from_bech32<S>(s: S) -> Result<Self, Self::Err>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&s.into()).map_err(|_| Error::Bech32EventParseError)?;

        if hrp != PREFIX_BECH32_EVENT || checksum != Variant::Bech32 {
            return Err(Error::Bech32EventParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32EventParseError)?;

        let t = data.first().ok_or(Error::Bech32EventParseError)?;
        if *t != 0 {
            return Err(Error::Bech32EventParseError);
        }

        let l = data.get(1).ok_or(Error::Bech32EventParseError)?;
        if *l != 32 {
            return Err(Error::Bech32EventParseError);
        }

        let event_id = data.get(2..34).ok_or(Error::Bech32EventParseError)?;
        let event_id = EventId::from_slice(event_id)?;

        let mut relays: Vec<String> = Vec::new();
        let mut relays_data: Vec<u8> = data.get(34..).ok_or(Error::Bech32EventParseError)?.to_vec();

        while !relays_data.is_empty() {
            let t = relays_data.first().ok_or(Error::Bech32EventParseError)?;
            if *t != 1 {
                return Err(Error::Bech32EventParseError);
            }

            let l = relays_data.get(1).ok_or(Error::Bech32EventParseError)?;
            let l = *l as usize;

            let data = relays_data
                .get(2..l + 2)
                .ok_or(Error::Bech32EventParseError)?;

            relays
                .push(String::from_utf8(data.to_vec()).map_err(|_| Error::Bech32EventParseError)?);
            relays_data.drain(..l + 2);
        }

        Ok(Self { event_id, relays })
    }
}

#[cfg(feature = "base")]
impl ToBech32 for Nip19Event {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let mut bytes: Vec<u8> = vec![0, 32];
        bytes.extend(self.event_id.inner().iter());

        for relay in self.relays.iter() {
            bytes.extend([1, relay.len() as u8]);
            bytes.extend(relay.as_bytes());
        }

        let data = bytes.to_base32();
        Ok(bech32::encode(PREFIX_BECH32_EVENT, data, Variant::Bech32)?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::Result;

    #[test]
    fn to_bech32_public_key() -> Result<()> {
        let public_key = XOnlyPublicKey::from_str(
            "aa4fc8665f5696e33db7e1a572e3b0f5b3d615837b0f362dcb1c8068b098c7b4",
        )?;
        assert_eq!(
            "npub14f8usejl26twx0dhuxjh9cas7keav9vr0v8nvtwtrjqx3vycc76qqh9nsy".to_string(),
            public_key.to_bech32()?
        );
        Ok(())
    }

    #[test]
    fn to_bech32_secret_key() -> Result<()> {
        let secret_key = SecretKey::from_str(
            "9571a568a42b9e05646a349c783159b906b498119390df9a5a02667155128028",
        )?;
        assert_eq!(
            "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99".to_string(),
            secret_key.to_bech32()?
        );
        Ok(())
    }

    #[cfg(feature = "base")]
    #[test]
    fn to_bech32_note() -> Result<()> {
        let event_id =
            EventId::from_hex("d94a3f4dd87b9a3b0bed183b32e916fa29c8020107845d1752d72697fe5309a5")?;
        assert_eq!(
            "note1m99r7nwc0wdrkzldrqan96gklg5usqspq7z9696j6unf0ljnpxjspqfw99".to_string(),
            event_id.to_bech32()?
        );
        Ok(())
    }

    #[cfg(feature = "base")]
    #[test]
    fn to_bech32_profile() -> Result<()> {
        let profile = Profile::new(
            XOnlyPublicKey::from_str(
                "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d",
            )?,
            vec![
                String::from("wss://r.x.com"),
                String::from("wss://djbas.sadkb.com"),
            ],
        );
        assert_eq!("nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p".to_string(), profile.to_bech32()?);
        Ok(())
    }

    #[cfg(feature = "base")]
    #[test]
    fn from_bech32_profile() -> Result<()> {
        let bech32_profile = "nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p";
        let profile = Profile::from_bech32(bech32_profile)?;
        assert_eq!(
            "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d".to_string(),
            profile.public_key.to_string()
        );
        assert_eq!(
            vec![
                "wss://r.x.com".to_string(),
                "wss://djbas.sadkb.com".to_string()
            ],
            profile.relays
        );
        Ok(())
    }
}
