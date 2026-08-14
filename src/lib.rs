//! Provider-neutral live HaxBall state protocol.
//!
//! The protocol contains only state needed to reproduce the visible room. It
//! deliberately has no fields for passwords, auth strings, connection data,
//! IP addresses, provider credentials, or program-private state.

use prost::Message;
use thiserror::Error;

pub const PROTOCOL_VERSION: u32 = 1;
pub const MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_PLAYERS: usize = 128;
pub const MAX_DISCS: usize = 4_096;
pub const MAX_AUDIO_CUES: usize = 64;
pub const MAX_STADIUM_JSON_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone, PartialEq, Message)]
pub struct Envelope {
    #[prost(uint32, tag = "1")]
    pub protocol_version: u32,
    #[prost(uint64, tag = "2")]
    pub epoch: u64,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    #[prost(uint64, tag = "4")]
    pub source_tick: u64,
    #[prost(uint64, tag = "5")]
    pub source_time_micros: u64,
    #[prost(enumeration = "AudioCueKind", repeated, tag = "6")]
    pub audio_cues: Vec<i32>,
    #[prost(oneof = "envelope::Payload", tags = "10, 11")]
    pub payload: Option<envelope::Payload>,
}

pub mod envelope {
    use prost::Oneof;

    use super::{Checkpoint, DynamicFrame};

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Payload {
        #[prost(message, tag = "10")]
        Checkpoint(Checkpoint),
        #[prost(message, tag = "11")]
        Dynamic(DynamicFrame),
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Checkpoint {
    #[prost(string, tag = "1")]
    pub room_name: String,
    #[prost(uint32, tag = "2")]
    pub max_players: u32,
    #[prost(string, tag = "3")]
    pub stadium_json: String,
    #[prost(uint32, tag = "4")]
    pub score_limit: u32,
    #[prost(uint32, tag = "5")]
    pub time_limit_seconds: u32,
    #[prost(bool, tag = "6")]
    pub teams_locked: bool,
    #[prost(message, optional, tag = "7")]
    pub red_team_colors: Option<TeamColors>,
    #[prost(message, optional, tag = "8")]
    pub blue_team_colors: Option<TeamColors>,
    #[prost(message, optional, tag = "9")]
    pub dynamic: Option<DynamicState>,
    /// Canonical zero-frame HBR2 bootstrap for arbitrary custom stadiums.
    #[prost(bytes = "vec", tag = "10")]
    pub initial_replay: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct DynamicFrame {
    #[prost(uint64, tag = "1")]
    pub checkpoint_sequence: u64,
    #[prost(message, optional, tag = "2")]
    pub state: Option<DynamicState>,
}

#[derive(Clone, PartialEq, Message)]
pub struct DynamicState {
    #[prost(message, optional, tag = "1")]
    pub game: Option<GameState>,
    #[prost(message, repeated, tag = "2")]
    pub players: Vec<PlayerState>,
    #[prost(message, repeated, tag = "3")]
    pub discs: Vec<DiscState>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GameState {
    #[prost(enumeration = "GameStatus", tag = "1")]
    pub status: i32,
    #[prost(uint32, tag = "2")]
    pub red_score: u32,
    #[prost(uint32, tag = "3")]
    pub blue_score: u32,
    #[prost(double, tag = "4")]
    pub elapsed_seconds: f64,
    #[prost(uint32, tag = "5")]
    pub paused_ticks: u32,
    #[prost(enumeration = "Team", tag = "6")]
    pub kickoff_team: i32,
    #[prost(uint64, tag = "7")]
    pub active_ticks: u64,
    #[prost(int32, tag = "8")]
    pub countdown_ticks: i32,
    #[prost(int32, tag = "9")]
    pub phase_code: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct PlayerState {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(enumeration = "Team", tag = "3")]
    pub team: i32,
    #[prost(bool, tag = "4")]
    pub admin: bool,
    #[prost(int32, tag = "5")]
    pub player_number: i32,
    #[prost(bool, tag = "6")]
    pub activity: bool,
    #[prost(message, optional, tag = "7")]
    pub position: Option<Vec2>,
    #[prost(uint32, optional, tag = "8")]
    pub disc_index: Option<u32>,
    #[prost(string, optional, tag = "9")]
    pub avatar: Option<String>,
    #[prost(uint32, tag = "10")]
    pub input: u32,
}

#[derive(Clone, PartialEq, Message)]
pub struct DiscState {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(message, optional, tag = "2")]
    pub position: Option<Vec2>,
    #[prost(message, optional, tag = "3")]
    pub speed: Option<Vec2>,
    #[prost(message, optional, tag = "4")]
    pub gravity: Option<Vec2>,
    #[prost(double, tag = "5")]
    pub radius: f64,
    #[prost(double, tag = "6")]
    pub bounce: f64,
    #[prost(double, tag = "7")]
    pub inverse_mass: f64,
    #[prost(double, tag = "8")]
    pub damping: f64,
    #[prost(int32, tag = "9")]
    pub color: i32,
    #[prost(int32, tag = "10")]
    pub collision_mask: i32,
    #[prost(int32, tag = "11")]
    pub collision_group: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct Vec2 {
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
}

#[derive(Clone, PartialEq, Message)]
pub struct TeamColors {
    #[prost(int32, tag = "1")]
    pub angle: i32,
    #[prost(uint32, tag = "2")]
    pub text_color: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub colors: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
#[repr(i32)]
pub enum Team {
    Spectators = 0,
    Red = 1,
    Blue = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
#[repr(i32)]
pub enum GameStatus {
    Stopped = 0,
    Running = 1,
    Paused = 2,
    Resuming = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
#[repr(i32)]
pub enum AudioCueKind {
    PlayerJoin = 0,
    PlayerLeave = 1,
    BallKick = 2,
    Goal = 3,
    Chat = 4,
    Notification = 5,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("live frame exceeds the {MAX_FRAME_BYTES}-byte limit")]
    FrameTooLarge,
    #[error("live frame could not be decoded: {0}")]
    Decode(String),
    #[error("unsupported live protocol version {0}")]
    UnsupportedVersion(u32),
    #[error("live frame has no payload")]
    MissingPayload,
    #[error("checkpoint has no dynamic state")]
    MissingCheckpointState,
    #[error("dynamic frame has no state")]
    MissingDynamicState,
    #[error("live frame contains too many players")]
    TooManyPlayers,
    #[error("live frame contains too many discs")]
    TooManyDiscs,
    #[error("live frame contains too many audio cues")]
    TooManyAudioCues,
    #[error("live frame contains an unknown audio cue")]
    UnknownAudioCue,
    #[error("checkpoint stadium exceeds the size limit")]
    StadiumTooLarge,
    #[error("checkpoint bootstrap exceeds the size limit")]
    BootstrapTooLarge,
    #[error("live frame sequence must be positive")]
    InvalidSequence,
}

pub fn encode_frame(envelope: &Envelope) -> Result<Vec<u8>, ProtocolError> {
    validate(envelope)?;
    let bytes = envelope.encode_to_vec();
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    Ok(bytes)
}

pub fn decode_frame(bytes: &[u8]) -> Result<Envelope, ProtocolError> {
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let envelope =
        Envelope::decode(bytes).map_err(|error| ProtocolError::Decode(error.to_string()))?;
    validate(&envelope)?;
    Ok(envelope)
}

pub fn validate(envelope: &Envelope) -> Result<(), ProtocolError> {
    if envelope.protocol_version != PROTOCOL_VERSION {
        return Err(ProtocolError::UnsupportedVersion(envelope.protocol_version));
    }
    if envelope.sequence == 0 {
        return Err(ProtocolError::InvalidSequence);
    }
    if envelope.audio_cues.len() > MAX_AUDIO_CUES {
        return Err(ProtocolError::TooManyAudioCues);
    }
    if envelope
        .audio_cues
        .iter()
        .any(|cue| AudioCueKind::try_from(*cue).is_err())
    {
        return Err(ProtocolError::UnknownAudioCue);
    }

    match envelope
        .payload
        .as_ref()
        .ok_or(ProtocolError::MissingPayload)?
    {
        envelope::Payload::Checkpoint(checkpoint) => {
            if checkpoint.stadium_json.len() > MAX_STADIUM_JSON_BYTES {
                return Err(ProtocolError::StadiumTooLarge);
            }
            if checkpoint.initial_replay.len() > MAX_STADIUM_JSON_BYTES {
                return Err(ProtocolError::BootstrapTooLarge);
            }
            validate_state(
                checkpoint
                    .dynamic
                    .as_ref()
                    .ok_or(ProtocolError::MissingCheckpointState)?,
            )
        }
        envelope::Payload::Dynamic(frame) => validate_state(
            frame
                .state
                .as_ref()
                .ok_or(ProtocolError::MissingDynamicState)?,
        ),
    }
}

fn validate_state(state: &DynamicState) -> Result<(), ProtocolError> {
    if state.players.len() > MAX_PLAYERS {
        return Err(ProtocolError::TooManyPlayers);
    }
    if state.discs.len() > MAX_DISCS {
        return Err(ProtocolError::TooManyDiscs);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn finite_vec2() -> impl Strategy<Value = Vec2> {
        (-1_000_000.0_f64..1_000_000.0, -1_000_000.0_f64..1_000_000.0)
            .prop_map(|(x, y)| Vec2 { x, y })
    }

    fn player_state() -> impl Strategy<Value = PlayerState> {
        (
            any::<u32>(),
            "[ -~]{0,32}",
            0_i32..=2,
            any::<bool>(),
            any::<i32>(),
            any::<bool>(),
            prop::option::of(finite_vec2()),
            prop::option::of(0_u32..MAX_DISCS as u32),
            prop::option::of("[ -~]{0,8}"),
            any::<u32>(),
        )
            .prop_map(
                |(
                    id,
                    name,
                    team,
                    admin,
                    player_number,
                    activity,
                    position,
                    disc_index,
                    avatar,
                    input,
                )| {
                    PlayerState {
                        id,
                        name,
                        team,
                        admin,
                        player_number,
                        activity,
                        position,
                        disc_index,
                        avatar,
                        input,
                    }
                },
            )
    }

    fn disc_state() -> impl Strategy<Value = DiscState> {
        (
            any::<u32>(),
            prop::option::of(finite_vec2()),
            prop::option::of(finite_vec2()),
            prop::option::of(finite_vec2()),
            -10_000.0_f64..10_000.0,
            -10_000.0_f64..10_000.0,
            -10_000.0_f64..10_000.0,
            -10_000.0_f64..10_000.0,
            any::<i32>(),
            any::<i32>(),
            any::<i32>(),
        )
            .prop_map(
                |(
                    index,
                    position,
                    speed,
                    gravity,
                    radius,
                    bounce,
                    inverse_mass,
                    damping,
                    color,
                    collision_mask,
                    collision_group,
                )| DiscState {
                    index,
                    position,
                    speed,
                    gravity,
                    radius,
                    bounce,
                    inverse_mass,
                    damping,
                    color,
                    collision_mask,
                    collision_group,
                },
            )
    }

    fn dynamic_state() -> impl Strategy<Value = DynamicState> {
        (
            prop::collection::vec(player_state(), 0..16),
            prop::collection::vec(disc_state(), 0..32),
        )
            .prop_map(|(players, discs)| DynamicState {
                game: None,
                players,
                discs,
            })
    }

    fn fixture() -> Envelope {
        Envelope {
            protocol_version: PROTOCOL_VERSION,
            epoch: 7,
            sequence: 1,
            source_tick: 42,
            source_time_micros: 700_000,
            audio_cues: vec![AudioCueKind::BallKick as i32],
            payload: Some(envelope::Payload::Checkpoint(Checkpoint {
                room_name: "BFL".into(),
                max_players: 16,
                stadium_json: "{}".into(),
                score_limit: 3,
                time_limit_seconds: 180,
                teams_locked: false,
                red_team_colors: None,
                blue_team_colors: None,
                dynamic: Some(DynamicState {
                    game: None,
                    players: vec![PlayerState {
                        id: 1,
                        name: "gabinho".into(),
                        team: Team::Red as i32,
                        admin: false,
                        player_number: 1,
                        activity: true,
                        position: Some(Vec2 { x: 10.0, y: 20.0 }),
                        disc_index: Some(4),
                        avatar: None,
                        input: 8,
                    }],
                    discs: vec![],
                }),
                initial_replay: Vec::new(),
            })),
        }
    }

    #[test]
    fn round_trips_checkpoint() {
        let value = fixture();
        let decoded = decode_frame(&encode_frame(&value).unwrap()).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn rejects_unknown_versions() {
        let mut value = fixture();
        value.protocol_version = PROTOCOL_VERSION + 1;
        assert_eq!(
            encode_frame(&value),
            Err(ProtocolError::UnsupportedVersion(PROTOCOL_VERSION + 1))
        );
    }

    #[test]
    fn rejects_private_scale_payloads() {
        let mut value = fixture();
        let Some(envelope::Payload::Checkpoint(checkpoint)) = value.payload.as_mut() else {
            unreachable!();
        };
        checkpoint.dynamic.as_mut().unwrap().players = (0..=MAX_PLAYERS)
            .map(|_| PlayerState {
                id: 0,
                name: String::new(),
                team: Team::Spectators as i32,
                admin: false,
                player_number: 0,
                activity: false,
                position: None,
                disc_index: None,
                avatar: None,
                input: 0,
            })
            .collect();
        assert_eq!(encode_frame(&value), Err(ProtocolError::TooManyPlayers));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        #[test]
        fn generated_checkpoints_round_trip(
            epoch in any::<u64>(),
            sequence in 1_u64..=u64::MAX,
            source_tick in any::<u64>(),
            source_time_micros in any::<u64>(),
            room_name in "[ -~]{0,64}",
            state in dynamic_state(),
            bootstrap in prop::collection::vec(any::<u8>(), 0..512),
        ) {
            let value = Envelope {
                protocol_version: PROTOCOL_VERSION,
                epoch,
                sequence,
                source_tick,
                source_time_micros,
                audio_cues: Vec::new(),
                payload: Some(envelope::Payload::Checkpoint(Checkpoint {
                    room_name,
                    max_players: MAX_PLAYERS as u32,
                    stadium_json: "{}".into(),
                    score_limit: 3,
                    time_limit_seconds: 180,
                    teams_locked: false,
                    red_team_colors: None,
                    blue_team_colors: None,
                    dynamic: Some(state),
                    initial_replay: bootstrap,
                })),
            };

            let encoded = encode_frame(&value).unwrap();
            prop_assert_eq!(decode_frame(&encoded).unwrap(), value);
        }

        #[test]
        fn arbitrary_bytes_fail_deterministically(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
            prop_assert_eq!(decode_frame(&bytes), decode_frame(&bytes));
        }
    }
}
