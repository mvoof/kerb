//! Remote control — the SDK channel for sending commands *into* iRacing.
//!
//! Everything else in this crate reads telemetry out of shared memory. This
//! module is the one exception. As the SDK header puts it:
//!
//! > Remote control the sim by sending these windows messages. Camera and
//! > replay commands only work when you are out of your car, pit commands only
//! > work when in your car.
//!
//! iRacing registers a window message named `IRSDK_BROADCASTMSG` and listens for
//! it on every top-level window, so no connection or handle is needed — the
//! calls here work whether or not the sim is running, and silently do nothing
//! when it is not.
//!
//! Mirrors `irsdk_utils.cpp` / `irsdk_defines.h` from the official iRacing SDK
//! 1.20, which ships a working sample of this channel (`irsdk_msgtest`).
//!
//! ```no_run
//! use kerb::iracing::{PitCommand, ReplaySearch, send_pit_command, replay_search};
//!
//! // In the car: add 26 liters and change the left front at 159 kPa.
//! send_pit_command(PitCommand::Fuel, 26);
//! send_pit_command(PitCommand::Lf, 159);
//!
//! // Out of the car: jump the replay to the previous incident.
//! replay_search(ReplaySearch::PrevIncident);
//! ```

use std::sync::atomic::{AtomicU32, Ordering};

use windows_sys::Win32::UI::WindowsAndMessaging::{
    HWND_BROADCAST, RegisterWindowMessageW, SendNotifyMessageW,
};

/// `IRSDK_BROADCASTMSGNAME`, NUL-terminated for the wide API.
const BROADCAST_MSG_NAME: &[u16] = &[
    b'I' as u16,
    b'R' as u16,
    b'S' as u16,
    b'D' as u16,
    b'K' as u16,
    b'_' as u16,
    b'B' as u16,
    b'R' as u16,
    b'O' as u16,
    b'A' as u16,
    b'D' as u16,
    b'C' as u16,
    b'A' as u16,
    b'S' as u16,
    b'T' as u16,
    b'M' as u16,
    b'S' as u16,
    b'G' as u16,
    0,
];

/// `irsdk_BroadcastMsg` — which subsystem the message is aimed at.
///
/// Prefer the wrapper functions in this module; this is public so callers can
/// reach commands added to the sim after this crate was released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BroadcastMsg {
    /// car position, group, camera
    CamSwitchPos = 0,
    /// driver #, group, camera
    CamSwitchNum = 1,
    /// [`CameraState`] bits
    CamSetState = 2,
    /// speed, slowMotion
    ReplaySetPlaySpeed = 3,
    /// [`ReplayPosition`], frame number
    ReplaySetPlayPosition = 4,
    /// [`ReplaySearch`]
    ReplaySearch = 5,
    /// [`ReplayState`]
    ReplaySetState = 6,
    /// [`ReloadTextures`], carIdx
    ReloadTextures = 7,
    /// [`ChatCommand`], sub-command
    ChatCommand = 8,
    /// [`PitCommand`], parameter
    PitCommand = 9,
    /// [`TelemetryCommand`]
    TelemCommand = 10,
    /// [`FfbCommand`], value as a 32-bit float
    FfbCommand = 11,
    /// session number, session time in ms
    ReplaySearchSessionTime = 12,
    /// [`VideoCapture`]
    VideoCapture = 13,
}

/// `irsdk_PitCommandMode` — one checkbox in the pit service black box.
///
/// Only acted on **while the driver is in the car**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PitCommand {
    /// Unchecks every box in the pit service order.
    Clear = 0,
    /// Order a windshield clean, which spends one tear off.
    Windshield = 1,
    /// Add fuel; the parameter is liters, or 0 to keep the current amount.
    Fuel = 2,
    /// Change the left front; the parameter is kPa, or 0 to keep the current pressure.
    Lf = 3,
    /// Right front.
    Rf = 4,
    /// Left rear.
    Lr = 5,
    /// Right rear.
    Rr = 6,
    /// Clear the four tire checkboxes.
    ClearTires = 7,
    /// Order a fast repair, if the session grants any.
    FastRepair = 8,
    /// Uncheck "clean the windshield".
    ClearWindshield = 9,
    /// Uncheck "fast repair".
    ClearFastRepair = 10,
    /// Uncheck "add fuel".
    ClearFuel = 11,
    /// Change tire compound; the parameter is the compound index.
    TireCompound = 12,
}

/// `irsdk_ChatCommandMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChatCommand {
    /// Launch a chat macro; the parameter is a slot from 1 to 15.
    ///
    /// The macro text itself is configured in the sim's options and cannot be
    /// set from outside — this channel carries no free text.
    Macro = 0,
    /// Open a new chat window.
    BeginChat = 1,
    /// Reply to the last private chat.
    Reply = 2,
    /// Close the chat window.
    Cancel = 3,
}

/// `irsdk_TelemCommandMode`. Can be called any time, but the sim only records
/// while the driver is in the car.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TelemetryCommand {
    /// Stop recording to disk.
    Stop = 0,
    /// Begin recording to disk.
    Start = 1,
    /// Flush the current file to disk and start a new one.
    Restart = 2,
}

/// `irsdk_RpyStateMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReplayState {
    /// Discard everything currently held on the tape.
    EraseTape = 0,
}

/// `irsdk_ReloadTexturesMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReloadTextures {
    /// Reload the textures of every car.
    All = 0,
    /// Reload only the textures of one car; the parameter is its `carIdx`.
    CarIdx = 1,
}

/// `irsdk_RpySrchMode` — jump the replay tape to an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReplaySearch {
    ToStart = 0,
    ToEnd = 1,
    PrevSession = 2,
    NextSession = 3,
    PrevLap = 4,
    NextLap = 5,
    PrevFrame = 6,
    NextFrame = 7,
    PrevIncident = 8,
    NextIncident = 9,
}

/// `irsdk_RpyPosMode` — what a replay frame number is relative to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReplayPosition {
    Begin = 0,
    Current = 1,
    End = 2,
}

/// `irsdk_FFBCommandMode`. Can be called any time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FfbCommand {
    /// Maximum force used when mapping steering torque to direct input units, in Nm.
    MaxForce = 0,
}

/// `irsdk_VideoCaptureMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VideoCapture {
    /// Write a single still frame to disk.
    TriggerScreenShot = 0,
    StartVideoCapture = 1,
    EndVideoCapture = 2,
    ToggleVideoCapture = 3,
    /// Show the video timer in the upper left of the display.
    ShowVideoTimer = 4,
    HideVideoTimer = 5,
}

/// `irsdk_csMode` — the "focus at" selectors accepted by [`camera_switch_position`]
/// in place of a position, and by [`camera_switch_number`] in place of a car number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CameraFocus {
    Incident = -3,
    Leader = -2,
    Exiting = -1,
    /// Follow whichever car the position or number argument selects.
    Driver = 0,
}

/// `irsdk_CameraState` bits. Only the flags below the first two can be set
/// through [`camera_set_state`]; `IsSessionScreen` and `IsScenicActive` are
/// reported by the sim and ignored on the way in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraState(pub i32);

impl CameraState {
    pub const CAM_TOOL_ACTIVE: Self = Self(0x0004);
    pub const UI_HIDDEN: Self = Self(0x0008);
    pub const USE_AUTO_SHOT_SELECTION: Self = Self(0x0010);
    pub const USE_TEMPORARY_EDITS: Self = Self(0x0020);
    pub const USE_KEY_ACCELERATION: Self = Self(0x0040);
    pub const USE_KEY_10X_ACCELERATION: Self = Self(0x0080);
    pub const USE_MOUSE_AIM_MODE: Self = Self(0x0100);

    /// Combines two sets of flags.
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Caches the registered id. Zero doubles as "not resolved yet", which is sound
/// because Windows never hands out atom 0.
static MESSAGE_ID: AtomicU32 = AtomicU32::new(0);

/// The registered id of `IRSDK_BROADCASTMSG`, resolved on first use and cached
/// for the rest of the process.
///
/// `None` means Windows refused to register the message, which is the only way
/// sending can fail up front. A failure is not cached — the next call tries
/// again. A non-zero id does **not** mean iRacing is running.
pub fn broadcast_msg_id() -> Option<u32> {
    let cached = MESSAGE_ID.load(Ordering::Relaxed);
    if cached != 0 {
        return Some(cached);
    }

    // SAFETY: RegisterWindowMessageW only reads the NUL-terminated string.
    let id = unsafe { RegisterWindowMessageW(BROADCAST_MSG_NAME.as_ptr()) };

    // Racing threads all register the same name, and the call is idempotent, so
    // they can only ever store the same atom.
    if id != 0 {
        MESSAGE_ID.store(id, Ordering::Relaxed);
    }

    (id != 0).then_some(id)
}

/// `MAKELONG(msg, var1)`, as packed by `irsdk_broadcastMsg`.
fn make_wparam(msg: i32, var1: i32) -> usize {
    (((var1 as u32) << 16) | ((msg as u32) & 0xFFFF)) as usize
}

/// `MAKELONG(var2, var3)` — the two-16-bit-arguments form.
fn make_lparam(var2: i32, var3: i32) -> i32 {
    (((var3 as u32) << 16) | ((var2 as u32) & 0xFFFF)) as i32
}

/// Posts a raw broadcast message to every top-level window, with `var2` as a
/// full 32-bit value.
///
/// Fire-and-forget by design — `SendNotifyMessage` does not wait, so `true`
/// means the message was posted, not that iRacing acted on it. Returns `false`
/// only when the broadcast message could not be registered.
pub fn send_broadcast(msg: BroadcastMsg, var1: i32, var2: i32) -> bool {
    let Some(msg_id) = broadcast_msg_id() else {
        return false;
    };

    // SAFETY: SendNotifyMessageW takes plain integers and does not block.
    unsafe {
        SendNotifyMessageW(
            HWND_BROADCAST,
            msg_id,
            make_wparam(msg as i32, var1),
            var2 as isize,
        );
    }

    true
}

/// Posts a raw broadcast message where `var2` and `var3` are two signed 16-bit
/// arguments packed into the lparam.
pub fn send_broadcast3(msg: BroadcastMsg, var1: i32, var2: i32, var3: i32) -> bool {
    send_broadcast(msg, var1, make_lparam(var2, var3))
}

/// Posts a raw broadcast message carrying a 32-bit float in `var2`.
///
/// The SDK moves the fractional part into the integer part by scaling with
/// 2^16, so the sim receives a fixed-point value.
pub fn send_broadcast_float(msg: BroadcastMsg, var1: i32, var2: f32) -> bool {
    send_broadcast(msg, var1, (var2 * 65536.0) as i32)
}

// --- Pit service -----------------------------------------------------------

/// Sends one pit service command.
///
/// `param` is liters for [`PitCommand::Fuel`], kPa for the four corners, the
/// compound index for [`PitCommand::TireCompound`], and ignored otherwise. Pass
/// 0 to leave the existing value alone. Out-of-range values are clamped by the
/// sim, so callers need not validate them.
///
/// Only acted on while the driver is in the car.
pub fn send_pit_command(command: PitCommand, param: i32) -> bool {
    send_broadcast(BroadcastMsg::PitCommand, command as i32, param)
}

// --- Chat ------------------------------------------------------------------

/// Triggers one of the sim's 15 configured chat macros.
///
/// Slots outside 1..=15 are rejected without sending anything. The macro text
/// lives in the sim's options — it cannot be supplied here.
pub fn send_chat_macro(slot: i32) -> bool {
    if !(1..=15).contains(&slot) {
        return false;
    }

    send_broadcast(BroadcastMsg::ChatCommand, ChatCommand::Macro as i32, slot)
}

/// Opens, replies to, or closes the chat window.
///
/// Use [`send_chat_macro`] for [`ChatCommand::Macro`]; passing it here sends
/// slot 0, which the sim ignores.
pub fn send_chat_command(command: ChatCommand) -> bool {
    send_broadcast(BroadcastMsg::ChatCommand, command as i32, 0)
}

// --- Cameras ---------------------------------------------------------------

/// Switches the camera to a car by its running position.
///
/// Pass a [`CameraFocus`] value in place of `position` to follow the leader, the
/// latest incident, or the car exiting the pits. `group` and `camera` are
/// 1-based indices into the sim's camera groups; pass 0 to keep the current one.
///
/// Camera commands only work while out of the car.
pub fn camera_switch_position(position: i32, group: i32, camera: i32) -> bool {
    send_broadcast3(BroadcastMsg::CamSwitchPos, position, group, camera)
}

/// Switches the camera to a car by its car number.
///
/// Car numbers with leading zeros must be encoded with [`pad_car_num`].
pub fn camera_switch_number(car_number: i32, group: i32, camera: i32) -> bool {
    send_broadcast3(BroadcastMsg::CamSwitchNum, car_number, group, camera)
}

/// Sets the camera tool state flags.
pub fn camera_set_state(state: CameraState) -> bool {
    send_broadcast(BroadcastMsg::CamSetState, state.0, 0)
}

/// Encodes a car number that carries leading zeros, mirroring `irsdk_padCarNum`.
///
/// Car #001 is `pad_car_num(1, 2)` — `zero` is how many leading zeros the number
/// is displayed with.
pub fn pad_car_num(num: i32, zero: i32) -> i32 {
    let mut retval = num;
    let mut num_place = if num > 99 {
        3
    } else if num > 9 {
        2
    } else {
        1
    };

    if zero != 0 {
        num_place += zero;
        retval = num + 1000 * num_place;
    }

    retval
}

// --- Replay ----------------------------------------------------------------

/// Sets replay playback speed. `speed` is a multiplier, negative to rewind;
/// with `slow_motion` set it is instead a divisor (2 = half speed).
pub fn replay_set_play_speed(speed: i32, slow_motion: bool) -> bool {
    send_broadcast3(
        BroadcastMsg::ReplaySetPlaySpeed,
        speed,
        i32::from(slow_motion),
        0,
    )
}

/// Jumps to a frame of the replay tape, relative to `origin`.
pub fn replay_set_position(origin: ReplayPosition, frame: i32) -> bool {
    send_broadcast(BroadcastMsg::ReplaySetPlayPosition, origin as i32, frame)
}

/// Jumps to an event on the replay tape.
pub fn replay_search(mode: ReplaySearch) -> bool {
    send_broadcast(BroadcastMsg::ReplaySearch, mode as i32, 0)
}

/// Jumps to a point in time within a session, in milliseconds.
pub fn replay_search_session_time(session_num: i32, session_time_ms: i32) -> bool {
    send_broadcast(
        BroadcastMsg::ReplaySearchSessionTime,
        session_num,
        session_time_ms,
    )
}

/// Changes the replay tape state — currently only erasing it.
pub fn replay_set_state(state: ReplayState) -> bool {
    send_broadcast(BroadcastMsg::ReplaySetState, state as i32, 0)
}

// --- Misc ------------------------------------------------------------------

/// Starts, stops or rotates the sim's own telemetry recording.
pub fn send_telemetry_command(command: TelemetryCommand) -> bool {
    send_broadcast(BroadcastMsg::TelemCommand, command as i32, 0)
}

/// Sets a force feedback parameter. `value` is in Nm for [`FfbCommand::MaxForce`].
pub fn send_ffb_command(command: FfbCommand, value: f32) -> bool {
    send_broadcast_float(BroadcastMsg::FfbCommand, command as i32, value)
}

/// Reloads car textures. `car_idx` is only read for [`ReloadTextures::CarIdx`].
pub fn reload_textures(mode: ReloadTextures, car_idx: i32) -> bool {
    send_broadcast(BroadcastMsg::ReloadTextures, mode as i32, car_idx)
}

/// Triggers a screenshot or drives the sim's video capture.
pub fn send_video_capture(mode: VideoCapture) -> bool {
    send_broadcast(BroadcastMsg::VideoCapture, mode as i32, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadcast_msg_codes_match_the_sdk_enum() {
        assert_eq!(BroadcastMsg::CamSwitchPos as i32, 0);
        assert_eq!(BroadcastMsg::ChatCommand as i32, 8);
        assert_eq!(BroadcastMsg::PitCommand as i32, 9);
        assert_eq!(BroadcastMsg::TelemCommand as i32, 10);
        assert_eq!(BroadcastMsg::FfbCommand as i32, 11);
        assert_eq!(BroadcastMsg::ReplaySearchSessionTime as i32, 12);
        assert_eq!(BroadcastMsg::VideoCapture as i32, 13);
    }

    #[test]
    fn pit_command_codes_match_the_sdk_enum() {
        assert_eq!(PitCommand::Clear as i32, 0);
        assert_eq!(PitCommand::Fuel as i32, 2);
        assert_eq!(PitCommand::Lf as i32, 3);
        assert_eq!(PitCommand::Rr as i32, 6);
        assert_eq!(PitCommand::ClearFuel as i32, 11);
        assert_eq!(PitCommand::TireCompound as i32, 12);
    }

    #[test]
    fn sub_command_codes_match_the_sdk_enums() {
        assert_eq!(ChatCommand::Macro as i32, 0);
        assert_eq!(ChatCommand::Cancel as i32, 3);
        assert_eq!(TelemetryCommand::Restart as i32, 2);
        assert_eq!(ReplaySearch::NextIncident as i32, 9);
        assert_eq!(ReplayPosition::End as i32, 2);
        assert_eq!(ReloadTextures::CarIdx as i32, 1);
        assert_eq!(VideoCapture::HideVideoTimer as i32, 5);
        assert_eq!(FfbCommand::MaxForce as i32, 0);
        assert_eq!(ReplayState::EraseTape as i32, 0);
    }

    #[test]
    fn camera_focus_selectors_are_negative() {
        assert_eq!(CameraFocus::Incident as i32, -3);
        assert_eq!(CameraFocus::Leader as i32, -2);
        assert_eq!(CameraFocus::Exiting as i32, -1);
        assert_eq!(CameraFocus::Driver as i32, 0);
    }

    #[test]
    fn camera_state_flags_combine() {
        let combined = CameraState::UI_HIDDEN.with(CameraState::CAM_TOOL_ACTIVE);

        assert_eq!(combined.0, 0x000C);
    }

    #[test]
    fn wparam_packs_msg_low_and_subcommand_high() {
        // 9 is BroadcastMsg::PitCommand, 2 is PitCommand::Fuel.
        assert_eq!(make_wparam(9, 2), 0x0002_0009);
        assert_eq!(make_wparam(9, 12), 0x000C_0009);
    }

    #[test]
    fn lparam_packs_two_signed_words() {
        assert_eq!(make_lparam(1, 2), 0x0002_0001);
        assert_eq!(make_lparam(-1, 0), 0x0000_FFFF);
    }

    #[test]
    fn float_payload_is_scaled_by_2_pow_16() {
        assert_eq!((1.5_f32 * 65536.0) as i32, 98_304);
    }

    #[test]
    fn chat_macro_rejects_slots_outside_the_sim_range() {
        assert!(!send_chat_macro(0));
        assert!(!send_chat_macro(16));
        assert!(!send_chat_macro(-1));
    }

    #[test]
    fn pad_car_num_matches_the_sdk_helper() {
        assert_eq!(pad_car_num(1, 0), 1);
        assert_eq!(pad_car_num(1, 2), 3001);
        assert_eq!(pad_car_num(12, 1), 3012);
        assert_eq!(pad_car_num(123, 0), 123);
    }

    #[test]
    fn broadcast_name_matches_the_sdk_literal() {
        let decoded: String = BROADCAST_MSG_NAME[..BROADCAST_MSG_NAME.len() - 1]
            .iter()
            .map(|c| char::from(*c as u8))
            .collect();

        assert_eq!(decoded, "IRSDK_BROADCASTMSG");
    }
}
