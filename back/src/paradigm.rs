//! Functionality related to paradigm.
//!
//! In here is:
//! - `struct ParadigmSize`: which size of paradigm, min, std or full
//! - `struct AcceptedPos`: only particuar poses can be asked for when
//!   generating paradigm
//!
use serde::{Deserialize, Serialize};

use analysis_string_parser::Pos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ParadigmSize {
    #[serde(rename(deserialize = "minimal", serialize = "min"))]
    Minimal,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "full")]
    Full,
}

impl std::fmt::Display for ParadigmSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

impl std::str::FromStr for AcceptedPos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use serde::de::IntoDeserializer;
        // Invoke serde's deserialization machinery to deserialize it from the string
        Ok(
            Self::deserialize::<serde::de::value::StrDeserializer<'_, serde::de::value::Error>>(
                s.into_deserializer(),
            )
            .unwrap_or(AcceptedPos::Any),
        )
    }
}

impl std::fmt::Display for AcceptedPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AcceptedPos {
    #[serde(alias = "any")]
    Any,
    #[serde(alias = "a")]
    A,
    #[serde(alias = "n")]
    N,
    #[serde(alias = "v")]
    V,
    #[serde(alias = "adv")]
    Adv,
    #[serde(alias = "num")]
    Num,
    #[serde(alias = "pron")]
    Pron,
}

impl AcceptedPos {
    /// Turn this [`AcceptedPos`] into the corresponding standard [`Pos`], wrapped in
    /// [`Some`] if it is an actual pos, or [`None`] if it is [`AcceptedPos::Any`].
    pub fn to_standard_pos(&self) -> Option<Pos> {
        use analysis_string_parser::Pos::*;
        match self {
            AcceptedPos::Any => None,
            AcceptedPos::A => Some(A),
            AcceptedPos::N => Some(N),
            AcceptedPos::V => Some(V),
            AcceptedPos::Adv => Some(Adv),
            AcceptedPos::Num => Some(Num),
            AcceptedPos::Pron => Some(Pron),
        }
    }
}
