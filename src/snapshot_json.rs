//! Optional bounded domain transport. Validation does not authenticate execution.

use alloc::{string::String, vec::Vec};
use core::{cell::Cell, fmt, fmt::Write};
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};

use super::{CampaignCounts, CampaignSnapshot};
use crate::{ContractIdentity, RequirementId, RevisionId};

const MAX_BYTES: usize = 65536;
const MAX_IDENTITY_BYTES: usize = 4096;
const SCHEMA: &str = "runtime.campaign-snapshot/v1";
const COUNTER_SEMANTICS: &str = "saturating-u64-v1";

/// Structured input/limit refusal; allocator exhaustion can still abort the process.
// Implements: FR-004
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotError {
    /// Wire, decoded identity or encoded output exceeds its declared byte limit.
    ResourceLimit,
    /// JSON shape, member population, numeric spelling or UTF-8 is invalid.
    Malformed,
    /// The schema or counter-semantics version is not supported.
    UnsupportedVersion,
    /// Failed exceeds accepted, contrary to the runtime accounting invariant.
    InvalidCounts,
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ResourceLimit => "campaign snapshot resource limit exceeded",
            Self::Malformed => "malformed campaign snapshot",
            Self::UnsupportedVersion => "unsupported campaign snapshot version",
            Self::InvalidCounts => "campaign snapshot failed exceeds accepted",
        })
    }
}

/// Owned, structurally validated but unauthenticated imported accounting.
///
/// There is no conversion back to a mutable report:
/// ~~~compile_fail
/// use quire_contract_runtime::{CampaignReport, DecodedCampaignSnapshot};
/// fn resume(value: DecodedCampaignSnapshot) -> CampaignReport<'static> { value.into() }
/// ~~~
/// Imported identity cannot be replaced through public fields:
/// ~~~compile_fail
/// use quire_contract_runtime::DecodedCampaignSnapshot;
/// fn rewrite(value: &mut DecodedCampaignSnapshot) { value.requirement = "foreign".into(); }
/// ~~~
// Implements: FR-004
#[derive(Debug, Eq, PartialEq)]
pub struct DecodedCampaignSnapshot {
    requirement: String,
    revision: String,
    counts: CampaignCounts,
}

impl DecodedCampaignSnapshot {
    /// Borrows exact identities and copies complete counters; makes no execution claim.
    // Implements: FR-004
    #[must_use]
    pub fn snapshot(&self) -> CampaignSnapshot<'_> {
        CampaignSnapshot {
            identity: ContractIdentity::new(
                RequirementId::new(&self.requirement),
                RevisionId::new(&self.revision),
            ),
            counts: self.counts,
        }
    }
}

/// Emits deterministic complete JSON with bounded output and no partial public bytes.
///
/// Each identity is at most 4096 UTF-8 bytes and output is at most 65536 bytes.
/// Allocation failure may abort; this is not a universal recoverable-OOM guarantee.
// Implements: FR-004
pub fn encode_campaign_snapshot(snapshot: &CampaignSnapshot<'_>) -> Result<Vec<u8>, SnapshotError> {
    let identity = snapshot.identity();
    let requirement = identity.requirement.as_str();
    let revision = identity.revision.as_str();
    if requirement.len() > MAX_IDENTITY_BYTES || revision.len() > MAX_IDENTITY_BYTES {
        return Err(SnapshotError::ResourceLimit);
    }
    let bound = requirement
        .len()
        .checked_add(revision.len())
        .and_then(|length| length.checked_mul(6))
        .and_then(|length| length.checked_add(512))
        .filter(|length| *length <= MAX_BYTES)
        .ok_or(SnapshotError::ResourceLimit)?;
    let mut output = BoundedOutput(Vec::with_capacity(bound));
    let counts = snapshot.counts();
    let result = (|| {
        write!(output, "{{\"schemaVersion\":\"{SCHEMA}\",\"requirement\":")?;
        write_json_string(&mut output, requirement)?;
        output.write_str(",\"revision\":")?;
        write_json_string(&mut output, revision)?;
        write!(output,
            ",\"counterSemantics\":\"{COUNTER_SEMANTICS}\",\"counts\":{{\"accepted\":{},\"rejected\":{},\"failed\":{},\"discarded\":{}}}}}",
            counts.accepted(), counts.rejected(), counts.failed(), counts.discarded())
    })();
    result.map_err(|_| SnapshotError::ResourceLimit)?;
    Ok(output.0)
}

struct BoundedOutput(Vec<u8>);

impl fmt::Write for BoundedOutput {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        if self
            .0
            .len()
            .checked_add(value.len())
            .filter(|length| *length <= MAX_BYTES)
            .is_none()
        {
            return Err(fmt::Error);
        }
        self.0.extend_from_slice(value.as_bytes());
        Ok(())
    }
}

fn write_json_string(output: &mut BoundedOutput, value: &str) -> fmt::Result {
    output.write_char('"')?;
    for character in value.chars() {
        match character {
            '"' => output.write_str("\\\"")?,
            '\\' => output.write_str("\\\\")?,
            '\u{0000}'..='\u{001f}' => write!(output, "\\u{:04x}", u32::from(character))?,
            _ => output.write_char(character)?,
        }
    }
    output.write_char('"')
}

/// Validates complete bounded JSON, preserving opaque identity and all counters.
///
/// No filesystem access, verdict reconstruction, mutable report import or execution
/// authentication occurs. Input is capped before parsing; escaped-string parser scratch
/// is bounded by input volume. Allocation failure may still abort the process.
// Implements: FR-004
pub fn decode_campaign_snapshot(input: &[u8]) -> Result<DecodedCampaignSnapshot, SnapshotError> {
    if input.len() > MAX_BYTES {
        return Err(SnapshotError::ResourceLimit);
    }
    let refusal = Cell::new(None);
    let mut decoder = serde_json::Deserializer::from_slice(input);
    let snapshot = SnapshotSeed(&refusal)
        .deserialize(&mut decoder)
        .map_err(|_| refusal.get().unwrap_or(SnapshotError::Malformed))?;
    decoder.end().map_err(|_| SnapshotError::Malformed)?;
    if snapshot.counts.failed > snapshot.counts.accepted {
        return Err(SnapshotError::InvalidCounts);
    }
    Ok(snapshot)
}

// A single fixed vocabulary avoids allocating keys or accepting ignored nested values.
#[derive(Clone, Copy)]
enum Field {
    Schema,
    Requirement,
    Revision,
    Semantics,
    Counts,
    Accepted,
    Rejected,
    Failed,
    Discarded,
}

impl Field {
    fn bit(self) -> u16 {
        match self {
            Self::Schema => 1,
            Self::Requirement => 2,
            Self::Revision => 4,
            Self::Semantics => 8,
            Self::Counts => 16,
            Self::Accepted => 32,
            Self::Rejected => 64,
            Self::Failed => 128,
            Self::Discarded => 256,
        }
    }
}

impl<'de> serde::Deserialize<'de> for Field {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        decoder.deserialize_str(FieldVisitor)
    }
}

struct FieldVisitor;
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a declared campaign snapshot member")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Field, E> {
        match value {
            "schemaVersion" => Ok(Field::Schema),
            "requirement" => Ok(Field::Requirement),
            "revision" => Ok(Field::Revision),
            "counterSemantics" => Ok(Field::Semantics),
            "counts" => Ok(Field::Counts),
            "accepted" => Ok(Field::Accepted),
            "rejected" => Ok(Field::Rejected),
            "failed" => Ok(Field::Failed),
            "discarded" => Ok(Field::Discarded),
            _ => Err(E::custom("unknown snapshot member")),
        }
    }
}

struct TextSeed<'a> {
    refusal: &'a Cell<Option<SnapshotError>>,
    expected: Option<&'static str>,
}

impl<'de> DeserializeSeed<'de> for TextSeed<'_> {
    type Value = String;
    fn deserialize<D: serde::Deserializer<'de>>(self, decoder: D) -> Result<String, D::Error> {
        decoder.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for TextSeed<'_> {
    type Value = String;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded snapshot string")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<String, E> {
        if let Some(expected) = self.expected {
            if value != expected {
                self.refusal.set(Some(SnapshotError::UnsupportedVersion));
                return Err(E::custom("unsupported snapshot version"));
            }
            return Ok(String::new());
        }
        if value.len() > MAX_IDENTITY_BYTES {
            self.refusal.set(Some(SnapshotError::ResourceLimit));
            return Err(E::custom("snapshot identity limit"));
        }
        Ok(String::from(value))
    }
}

struct SnapshotSeed<'a>(&'a Cell<Option<SnapshotError>>);
impl<'de> DeserializeSeed<'de> for SnapshotSeed<'_> {
    type Value = DecodedCampaignSnapshot;
    fn deserialize<D: serde::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for SnapshotSeed<'_> {
    type Value = DecodedCampaignSnapshot;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a complete campaign snapshot object")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen = 0u16;
        let mut requirement = String::new();
        let mut revision = String::new();
        let mut counts = CampaignCounts::new();
        while let Some(field) = map.next_key::<Field>()? {
            if seen & field.bit() != 0 {
                return Err(de::Error::custom("duplicate snapshot member"));
            }
            seen |= field.bit();
            match field {
                Field::Schema => {
                    map.next_value_seed(TextSeed {
                        refusal: self.0,
                        expected: Some(SCHEMA),
                    })?;
                }
                Field::Semantics => {
                    map.next_value_seed(TextSeed {
                        refusal: self.0,
                        expected: Some(COUNTER_SEMANTICS),
                    })?;
                }
                Field::Requirement => {
                    requirement = map.next_value_seed(TextSeed {
                        refusal: self.0,
                        expected: None,
                    })?
                }
                Field::Revision => {
                    revision = map.next_value_seed(TextSeed {
                        refusal: self.0,
                        expected: None,
                    })?
                }
                Field::Counts => counts = map.next_value_seed(CountsSeed)?,
                _ => return Err(de::Error::custom("unexpected root member")),
            }
        }
        if seen != 31 {
            return Err(de::Error::custom("missing snapshot member"));
        }
        Ok(DecodedCampaignSnapshot {
            requirement,
            revision,
            counts,
        })
    }
}

struct CountsSeed;
impl<'de> DeserializeSeed<'de> for CountsSeed {
    type Value = CampaignCounts;
    fn deserialize<D: serde::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for CountsSeed {
    type Value = CampaignCounts;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("exactly four unsigned campaign counters")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen = 0u16;
        let mut counts = CampaignCounts::new();
        while let Some(field) = map.next_key::<Field>()? {
            if seen & field.bit() != 0 {
                return Err(de::Error::custom("duplicate counter"));
            }
            seen |= field.bit();
            match field {
                Field::Accepted => counts.accepted = map.next_value::<u64>()?,
                Field::Rejected => counts.rejected = map.next_value::<u64>()?,
                Field::Failed => counts.failed = map.next_value::<u64>()?,
                Field::Discarded => counts.discarded = map.next_value::<u64>()?,
                _ => return Err(de::Error::custom("unexpected counts member")),
            }
        }
        if seen != 480 {
            return Err(de::Error::custom("missing counter"));
        }
        Ok(counts)
    }
}
