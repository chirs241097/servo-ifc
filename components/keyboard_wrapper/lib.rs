#![feature(negative_impls)]

use keyboard_types::KeyboardEvent;
use secret_macros::side_effect_free_attr_full;
use secret_macros::InvisibleSideEffectFreeDerive;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::secret::InvisibleSideEffectFree;
use secret_structs::secret::*;
use secret_structs::ternary_lattice as sec_lat;
//use serde::ser::{Serializer, SerializeStruct};
use serde::{Deserialize /*Deserializer*/, Serialize};
//use std::marker::PhantomData;
use keyboard_types::{Code, Key, KeyState, Location, Modifiers};
use malloc_size_of_derive::MallocSizeOf;
//use malloc_size_of::MallocSizeOf;
//use malloc_size_of::MallocSizeOfOps;

//#[derive(Clone, Default, Serialize, Deserialize)]
//pub struct SecKeyboardEvent {
//    pub ke: KeyboardEvent
//}

unsafe impl<L1, L2> InvisibleSideEffectFree for SecKeyboardEvent<L1, L2> {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecKeyboardEvent<L1, L2> {
    /// Whether the key is pressed or released.
    pub state: StaticDynamicAll<KeyStateWrapper, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// Logical key value.
    pub key: StaticDynamicAll<KeyWrapper, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// Physical key position.
    pub code: StaticDynamicAll<CodeWrapper, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// Location for keys with multiple instances on common keyboards.
    pub location:
        StaticDynamicAll<LocationWrapper, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// Flags for pressed modifier keys.
    pub modifiers:
        StaticDynamicAll<ModifiersWrapper, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// True if the key is currently auto-repeated.
    pub repeat: StaticDynamicAll<bool, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
    /// Events with this flag should be ignored in a text editor
    /// and instead composition events should be used.
    pub is_composing: StaticDynamicAll<bool, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>,
}

impl<L1, L2> SecKeyboardEvent<L1, L2> {
    pub fn wrap(ke: KeyboardEvent, sl: DynamicSecretLabel, il: DynamicIntegrityLabel) -> Self {
        SecKeyboardEvent {
            state: StaticDynamicAll::<KeyStateWrapper,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(KeyStateWrapper{k: ke.state}, sl.clone(), il.clone()),
            key: StaticDynamicAll::<KeyWrapper,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(KeyWrapper{k: ke.key}, sl.clone(), il.clone()),
            code: StaticDynamicAll::<CodeWrapper,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(CodeWrapper{c: ke.code}, sl.clone(), il.clone()),
            location: StaticDynamicAll::<LocationWrapper,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(LocationWrapper{l: ke.location}, sl.clone(), il.clone()),
            modifiers: StaticDynamicAll::<ModifiersWrapper,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(ModifiersWrapper{m: ke.modifiers}, sl.clone(), il.clone()),
            repeat: StaticDynamicAll::<bool,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(ke.repeat, sl.clone(), il.clone()),
            is_composing: StaticDynamicAll::<bool,L1,L2,DynamicSecretLabel,DynamicIntegrityLabel>
            ::new_info_flow_struct(ke.is_composing, sl.clone(), il.clone())
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct KeyStateWrapper {
    pub k: KeyState,
}

pub type ServoSecure<T> = StaticDynamicAll<
    T,
    sec_lat::Label_A,
    int_lat::Label_All,
    DynamicSecretLabel,
    DynamicIntegrityLabel,
>;

unsafe impl InvisibleSideEffectFree for KeyStateWrapper {}

#[derive(Clone, Default, Serialize, Deserialize, MallocSizeOf)]
pub struct KeyWrapper {
    pub k: Key,
}
/*#[side_effect_free_attr_full]
pub fn is_enter(k: &KeyWrapper) -> bool {
    unsafe {k.k == Key::Enter}
}

#[side_effect_free_attr_full]
pub fn is_space(c: &CodeWrapper) -> bool {
    unsafe {c.c == Code::Space}
}

#[side_effect_free_attr_full]
pub fn is_up(k: &KeyStateWrapper) -> bool {
    unsafe {k.k == KeyState::Up}
}

#[side_effect_free_attr_full]
pub fn is_down(k: &KeyStateWrapper) -> bool {
    unsafe {k.k == KeyState::Down}
}

#[side_effect_free_attr_full]
pub fn code_to_string(c: &CodeWrapper) -> String {
    unsafe {c.c.to_string()}
}

#[side_effect_free_attr_full]
pub fn key_state_to_string(k: &KeyStateWrapper) -> String {
    unsafe {k.k.to_string()}
}

#[side_effect_free_attr_full]
pub fn legacy_charcode(k: &KeyWrapper) -> u32 {
    unsafe { k.k.legacy_charcode() }
}*/

#[side_effect_free_attr_full]
pub fn to_string(k: &KeyWrapper) -> String {
    match k.k {
        Key::Character(ref s) => std::string::String::from(s),
        Key::Unidentified => std::string::String::from("Unidentified"),
        Key::Alt => std::string::String::from("Alt"),
        Key::AltGraph => std::string::String::from("AltGraph"),
        Key::CapsLock => std::string::String::from("CapsLock"),
        Key::Control => std::string::String::from("Control"),
        Key::Fn => std::string::String::from("Fn"),
        Key::FnLock => std::string::String::from("FnLock"),
        Key::Meta => std::string::String::from("Meta"),
        Key::NumLock => std::string::String::from("NumLock"),
        Key::ScrollLock => std::string::String::from("ScrollLock"),
        Key::Shift => std::string::String::from("Shift"),
        Key::Symbol => std::string::String::from("Symbol"),
        Key::SymbolLock => std::string::String::from("SymbolLock"),
        Key::Hyper => std::string::String::from("Hyper"),
        Key::Super => std::string::String::from("Super"),
        Key::Enter => std::string::String::from("Enter"),
        Key::Tab => std::string::String::from("Tab"),
        Key::ArrowDown => std::string::String::from("ArrowDown"),
        Key::ArrowLeft => std::string::String::from("ArrowLeft"),
        Key::ArrowRight => std::string::String::from("ArrowRight"),
        Key::ArrowUp => std::string::String::from("ArrowUp"),
        Key::End => std::string::String::from("End"),
        Key::Home => std::string::String::from("Home"),
        Key::PageDown => std::string::String::from("PageDown"),
        Key::PageUp => std::string::String::from("PageUp"),
        Key::Backspace => std::string::String::from("Backspace"),
        Key::Clear => std::string::String::from("Clear"),
        Key::Copy => std::string::String::from("Copy"),
        Key::CrSel => std::string::String::from("CrSel"),
        Key::Cut => std::string::String::from("Cut"),
        Key::Delete => std::string::String::from("Delete"),
        Key::EraseEof => std::string::String::from("EraseEof"),
        Key::ExSel => std::string::String::from("ExSel"),
        Key::Insert => std::string::String::from("Insert"),
        Key::Paste => std::string::String::from("Paste"),
        Key::Redo => std::string::String::from("Redo"),
        Key::Undo => std::string::String::from("Undo"),
        Key::Accept => std::string::String::from("Accept"),
        Key::Again => std::string::String::from("Again"),
        Key::Attn => std::string::String::from("Attn"),
        Key::Cancel => std::string::String::from("Cancel"),
        Key::ContextMenu => std::string::String::from("ContextMenu"),
        Key::Escape => std::string::String::from("Escape"),
        Key::Execute => std::string::String::from("Execute"),
        Key::Find => std::string::String::from("Find"),
        Key::Help => std::string::String::from("Help"),
        Key::Pause => std::string::String::from("Pause"),
        Key::Play => std::string::String::from("Play"),
        Key::Props => std::string::String::from("Props"),
        Key::Select => std::string::String::from("Select"),
        Key::ZoomIn => std::string::String::from("ZoomIn"),
        Key::ZoomOut => std::string::String::from("ZoomOut"),
        Key::BrightnessDown => std::string::String::from("BrightnessDown"),
        Key::BrightnessUp => std::string::String::from("BrightnessUp"),
        Key::Eject => std::string::String::from("Eject"),
        Key::LogOff => std::string::String::from("LogOff"),
        Key::Power => std::string::String::from("Power"),
        Key::PowerOff => std::string::String::from("PowerOff"),
        Key::PrintScreen => std::string::String::from("PrintScreen"),
        Key::Hibernate => std::string::String::from("Hibernate"),
        Key::Standby => std::string::String::from("Standby"),
        Key::WakeUp => std::string::String::from("WakeUp"),
        Key::AllCandidates => std::string::String::from("AllCandidates"),
        Key::Alphanumeric => std::string::String::from("Alphanumeric"),
        Key::CodeInput => std::string::String::from("CodeInput"),
        Key::Compose => std::string::String::from("Compose"),
        Key::Convert => std::string::String::from("Convert"),
        Key::Dead => std::string::String::from("Dead"),
        Key::FinalMode => std::string::String::from("FinalMode"),
        Key::GroupFirst => std::string::String::from("GroupFirst"),
        Key::GroupLast => std::string::String::from("GroupLast"),
        Key::GroupNext => std::string::String::from("GroupNext"),
        Key::GroupPrevious => std::string::String::from("GroupPrevious"),
        Key::ModeChange => std::string::String::from("ModeChange"),
        Key::NextCandidate => std::string::String::from("NextCandidate"),
        Key::NonConvert => std::string::String::from("NonConvert"),
        Key::PreviousCandidate => std::string::String::from("PreviousCandidate"),
        Key::Process => std::string::String::from("Process"),
        Key::SingleCandidate => std::string::String::from("SingleCandidate"),
        Key::HangulMode => std::string::String::from("HangulMode"),
        Key::HanjaMode => std::string::String::from("HanjaMode"),
        Key::JunjaMode => std::string::String::from("JunjaMode"),
        Key::Eisu => std::string::String::from("Eisu"),
        Key::Hankaku => std::string::String::from("Hankaku"),
        Key::Hiragana => std::string::String::from("Hiragana"),
        Key::HiraganaKatakana => std::string::String::from("HiraganaKatakana"),
        Key::KanaMode => std::string::String::from("KanaMode"),
        Key::KanjiMode => std::string::String::from("KanjiMode"),
        Key::Katakana => std::string::String::from("Katakana"),
        Key::Romaji => std::string::String::from("Romaji"),
        Key::Zenkaku => std::string::String::from("Zenkaku"),
        Key::ZenkakuHankaku => std::string::String::from("ZenkakuHankaku"),
        Key::F1 => std::string::String::from("F1"),
        Key::F2 => std::string::String::from("F2"),
        Key::F3 => std::string::String::from("F3"),
        Key::F4 => std::string::String::from("F4"),
        Key::F5 => std::string::String::from("F5"),
        Key::F6 => std::string::String::from("F6"),
        Key::F7 => std::string::String::from("F7"),
        Key::F8 => std::string::String::from("F8"),
        Key::F9 => std::string::String::from("F9"),
        Key::F10 => std::string::String::from("F10"),
        Key::F11 => std::string::String::from("F11"),
        Key::F12 => std::string::String::from("F12"),
        Key::Soft1 => std::string::String::from("Soft1"),
        Key::Soft2 => std::string::String::from("Soft2"),
        Key::Soft3 => std::string::String::from("Soft3"),
        Key::Soft4 => std::string::String::from("Soft4"),
        Key::ChannelDown => std::string::String::from("ChannelDown"),
        Key::ChannelUp => std::string::String::from("ChannelUp"),
        Key::Close => std::string::String::from("Close"),
        Key::MailForward => std::string::String::from("MailForward"),
        Key::MailReply => std::string::String::from("MailReply"),
        Key::MailSend => std::string::String::from("MailSend"),
        Key::MediaClose => std::string::String::from("MediaClose"),
        Key::MediaFastForward => std::string::String::from("MediaFastForward"),
        Key::MediaPause => std::string::String::from("MediaPause"),
        Key::MediaPlay => std::string::String::from("MediaPlay"),
        Key::MediaPlayPause => std::string::String::from("MediaPlayPause"),
        Key::MediaRecord => std::string::String::from("MediaRecord"),
        Key::MediaRewind => std::string::String::from("MediaRewind"),
        Key::MediaStop => std::string::String::from("MediaStop"),
        Key::MediaTrackNext => std::string::String::from("MediaTrackNext"),
        Key::MediaTrackPrevious => std::string::String::from("MediaTrackPrevious"),
        Key::New => std::string::String::from("New"),
        Key::Open => std::string::String::from("Open"),
        Key::Print => std::string::String::from("Print"),
        Key::Save => std::string::String::from("Save"),
        Key::SpellCheck => std::string::String::from("SpellCheck"),
        Key::Key11 => std::string::String::from("Key11"),
        Key::Key12 => std::string::String::from("Key12"),
        Key::AudioBalanceLeft => std::string::String::from("AudioBalanceLeft"),
        Key::AudioBalanceRight => std::string::String::from("AudioBalanceRight"),
        Key::AudioBassBoostDown => std::string::String::from("AudioBassBoostDown"),
        Key::AudioBassBoostToggle => std::string::String::from("AudioBassBoostToggle"),
        Key::AudioBassBoostUp => std::string::String::from("AudioBassBoostUp"),
        Key::AudioFaderFront => std::string::String::from("AudioFaderFront"),
        Key::AudioFaderRear => std::string::String::from("AudioFaderRear"),
        Key::AudioSurroundModeNext => std::string::String::from("AudioSurroundModeNext"),
        Key::AudioTrebleDown => std::string::String::from("AudioTrebleDown"),
        Key::AudioTrebleUp => std::string::String::from("AudioTrebleUp"),
        Key::AudioVolumeDown => std::string::String::from("AudioVolumeDown"),
        Key::AudioVolumeUp => std::string::String::from("AudioVolumeUp"),
        Key::AudioVolumeMute => std::string::String::from("AudioVolumeMute"),
        Key::MicrophoneToggle => std::string::String::from("MicrophoneToggle"),
        Key::MicrophoneVolumeDown => std::string::String::from("MicrophoneVolumeDown"),
        Key::MicrophoneVolumeUp => std::string::String::from("MicrophoneVolumeUp"),
        Key::MicrophoneVolumeMute => std::string::String::from("MicrophoneVolumeMute"),
        Key::SpeechCorrectionList => std::string::String::from("SpeechCorrectionList"),
        Key::SpeechInputToggle => std::string::String::from("SpeechInputToggle"),
        Key::LaunchApplication1 => std::string::String::from("LaunchApplication1"),
        Key::LaunchApplication2 => std::string::String::from("LaunchApplication2"),
        Key::LaunchCalendar => std::string::String::from("LaunchCalendar"),
        Key::LaunchContacts => std::string::String::from("LaunchContacts"),
        Key::LaunchMail => std::string::String::from("LaunchMail"),
        Key::LaunchMediaPlayer => std::string::String::from("LaunchMediaPlayer"),
        Key::LaunchMusicPlayer => std::string::String::from("LaunchMusicPlayer"),
        Key::LaunchPhone => std::string::String::from("LaunchPhone"),
        Key::LaunchScreenSaver => std::string::String::from("LaunchScreenSaver"),
        Key::LaunchSpreadsheet => std::string::String::from("LaunchSpreadsheet"),
        Key::LaunchWebBrowser => std::string::String::from("LaunchWebBrowser"),
        Key::LaunchWebCam => std::string::String::from("LaunchWebCam"),
        Key::LaunchWordProcessor => std::string::String::from("LaunchWordProcessor"),
        Key::BrowserBack => std::string::String::from("BrowserBack"),
        Key::BrowserFavorites => std::string::String::from("BrowserFavorites"),
        Key::BrowserForward => std::string::String::from("BrowserForward"),
        Key::BrowserHome => std::string::String::from("BrowserHome"),
        Key::BrowserRefresh => std::string::String::from("BrowserRefresh"),
        Key::BrowserSearch => std::string::String::from("BrowserSearch"),
        Key::BrowserStop => std::string::String::from("BrowserStop"),
        Key::AppSwitch => std::string::String::from("AppSwitch"),
        Key::Call => std::string::String::from("Call"),
        Key::Camera => std::string::String::from("Camera"),
        Key::CameraFocus => std::string::String::from("CameraFocus"),
        Key::EndCall => std::string::String::from("EndCall"),
        Key::GoBack => std::string::String::from("GoBack"),
        Key::GoHome => std::string::String::from("GoHome"),
        Key::HeadsetHook => std::string::String::from("HeadsetHook"),
        Key::LastNumberRedial => std::string::String::from("LastNumberRedial"),
        Key::Notification => std::string::String::from("Notification"),
        Key::MannerMode => std::string::String::from("MannerMode"),
        Key::VoiceDial => std::string::String::from("VoiceDial"),
        Key::TV => std::string::String::from("TV"),
        Key::TV3DMode => std::string::String::from("TV3DMode"),
        Key::TVAntennaCable => std::string::String::from("TVAntennaCable"),
        Key::TVAudioDescription => std::string::String::from("TVAudioDescription"),
        Key::TVAudioDescriptionMixDown => std::string::String::from("TVAudioDescriptionMixDown"),
        Key::TVAudioDescriptionMixUp => std::string::String::from("TVAudioDescriptionMixUp"),
        Key::TVContentsMenu => std::string::String::from("TVContentsMenu"),
        Key::TVDataService => std::string::String::from("TVDataService"),
        Key::TVInput => std::string::String::from("TVInput"),
        Key::TVInputComponent1 => std::string::String::from("TVInputComponent1"),
        Key::TVInputComponent2 => std::string::String::from("TVInputComponent2"),
        Key::TVInputComposite1 => std::string::String::from("TVInputComposite1"),
        Key::TVInputComposite2 => std::string::String::from("TVInputComposite2"),
        Key::TVInputHDMI1 => std::string::String::from("TVInputHDMI1"),
        Key::TVInputHDMI2 => std::string::String::from("TVInputHDMI2"),
        Key::TVInputHDMI3 => std::string::String::from("TVInputHDMI3"),
        Key::TVInputHDMI4 => std::string::String::from("TVInputHDMI4"),
        Key::TVInputVGA1 => std::string::String::from("TVInputVGA1"),
        Key::TVMediaContext => std::string::String::from("TVMediaContext"),
        Key::TVNetwork => std::string::String::from("TVNetwork"),
        Key::TVNumberEntry => std::string::String::from("TVNumberEntry"),
        Key::TVPower => std::string::String::from("TVPower"),
        Key::TVRadioService => std::string::String::from("TVRadioService"),
        Key::TVSatellite => std::string::String::from("TVSatellite"),
        Key::TVSatelliteBS => std::string::String::from("TVSatelliteBS"),
        Key::TVSatelliteCS => std::string::String::from("TVSatelliteCS"),
        Key::TVSatelliteToggle => std::string::String::from("TVSatelliteToggle"),
        Key::TVTerrestrialAnalog => std::string::String::from("TVTerrestrialAnalog"),
        Key::TVTerrestrialDigital => std::string::String::from("TVTerrestrialDigital"),
        Key::TVTimer => std::string::String::from("TVTimer"),
        Key::AVRInput => std::string::String::from("AVRInput"),
        Key::AVRPower => std::string::String::from("AVRPower"),
        Key::ColorF0Red => std::string::String::from("ColorF0Red"),
        Key::ColorF1Green => std::string::String::from("ColorF1Green"),
        Key::ColorF2Yellow => std::string::String::from("ColorF2Yellow"),
        Key::ColorF3Blue => std::string::String::from("ColorF3Blue"),
        Key::ColorF4Grey => std::string::String::from("ColorF4Grey"),
        Key::ColorF5Brown => std::string::String::from("ColorF5Brown"),
        Key::ClosedCaptionToggle => std::string::String::from("ClosedCaptionToggle"),
        Key::Dimmer => std::string::String::from("Dimmer"),
        Key::DisplaySwap => std::string::String::from("DisplaySwap"),
        Key::DVR => std::string::String::from("DVR"),
        Key::Exit => std::string::String::from("Exit"),
        Key::FavoriteClear0 => std::string::String::from("FavoriteClear0"),
        Key::FavoriteClear1 => std::string::String::from("FavoriteClear1"),
        Key::FavoriteClear2 => std::string::String::from("FavoriteClear2"),
        Key::FavoriteClear3 => std::string::String::from("FavoriteClear3"),
        Key::FavoriteRecall0 => std::string::String::from("FavoriteRecall0"),
        Key::FavoriteRecall1 => std::string::String::from("FavoriteRecall1"),
        Key::FavoriteRecall2 => std::string::String::from("FavoriteRecall2"),
        Key::FavoriteRecall3 => std::string::String::from("FavoriteRecall3"),
        Key::FavoriteStore0 => std::string::String::from("FavoriteStore0"),
        Key::FavoriteStore1 => std::string::String::from("FavoriteStore1"),
        Key::FavoriteStore2 => std::string::String::from("FavoriteStore2"),
        Key::FavoriteStore3 => std::string::String::from("FavoriteStore3"),
        Key::Guide => std::string::String::from("Guide"),
        Key::GuideNextDay => std::string::String::from("GuideNextDay"),
        Key::GuidePreviousDay => std::string::String::from("GuidePreviousDay"),
        Key::Info => std::string::String::from("Info"),
        Key::InstantReplay => std::string::String::from("InstantReplay"),
        Key::Link => std::string::String::from("Link"),
        Key::ListProgram => std::string::String::from("ListProgram"),
        Key::LiveContent => std::string::String::from("LiveContent"),
        Key::Lock => std::string::String::from("Lock"),
        Key::MediaApps => std::string::String::from("MediaApps"),
        Key::MediaAudioTrack => std::string::String::from("MediaAudioTrack"),
        Key::MediaLast => std::string::String::from("MediaLast"),
        Key::MediaSkipBackward => std::string::String::from("MediaSkipBackward"),
        Key::MediaSkipForward => std::string::String::from("MediaSkipForward"),
        Key::MediaStepBackward => std::string::String::from("MediaStepBackward"),
        Key::MediaStepForward => std::string::String::from("MediaStepForward"),
        Key::MediaTopMenu => std::string::String::from("MediaTopMenu"),
        Key::NavigateIn => std::string::String::from("NavigateIn"),
        Key::NavigateNext => std::string::String::from("NavigateNext"),
        Key::NavigateOut => std::string::String::from("NavigateOut"),
        Key::NavigatePrevious => std::string::String::from("NavigatePrevious"),
        Key::NextFavoriteChannel => std::string::String::from("NextFavoriteChannel"),
        Key::NextUserProfile => std::string::String::from("NextUserProfile"),
        Key::OnDemand => std::string::String::from("OnDemand"),
        Key::Pairing => std::string::String::from("Pairing"),
        Key::PinPDown => std::string::String::from("PinPDown"),
        Key::PinPMove => std::string::String::from("PinPMove"),
        Key::PinPToggle => std::string::String::from("PinPToggle"),
        Key::PinPUp => std::string::String::from("PinPUp"),
        Key::PlaySpeedDown => std::string::String::from("PlaySpeedDown"),
        Key::PlaySpeedReset => std::string::String::from("PlaySpeedReset"),
        Key::PlaySpeedUp => std::string::String::from("PlaySpeedUp"),
        Key::RandomToggle => std::string::String::from("RandomToggle"),
        Key::RcLowBattery => std::string::String::from("RcLowBattery"),
        Key::RecordSpeedNext => std::string::String::from("RecordSpeedNext"),
        Key::RfBypass => std::string::String::from("RfBypass"),
        Key::ScanChannelsToggle => std::string::String::from("ScanChannelsToggle"),
        Key::ScreenModeNext => std::string::String::from("ScreenModeNext"),
        Key::Settings => std::string::String::from("Settings"),
        Key::SplitScreenToggle => std::string::String::from("SplitScreenToggle"),
        Key::STBInput => std::string::String::from("STBInput"),
        Key::STBPower => std::string::String::from("STBPower"),
        Key::Subtitle => std::string::String::from("Subtitle"),
        Key::Teletext => std::string::String::from("Teletext"),
        Key::VideoModeNext => std::string::String::from("VideoModeNext"),
        Key::Wink => std::string::String::from("Wink"),
        Key::ZoomToggle => std::string::String::from("ZoomToggle"),
        Key::F13 => std::string::String::from("F13"),
        Key::F14 => std::string::String::from("F14"),
        Key::F15 => std::string::String::from("F15"),
        Key::F16 => std::string::String::from("F16"),
        Key::F17 => std::string::String::from("F17"),
        Key::F18 => std::string::String::from("F18"),
        Key::F19 => std::string::String::from("F19"),
        Key::F20 => std::string::String::from("F20"),
        Key::F21 => std::string::String::from("F21"),
        Key::F22 => std::string::String::from("F22"),
        Key::F23 => std::string::String::from("F23"),
        Key::F24 => std::string::String::from("F24"),
        _ => std::string::String::from("Unrecognized Key"),
    }
}

unsafe impl InvisibleSideEffectFree for KeyWrapper {}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LocationWrapper {
    pub l: Location,
}

unsafe impl InvisibleSideEffectFree for LocationWrapper {}

#[derive(Clone, Default, Serialize, Deserialize, MallocSizeOf)]
pub struct ModifiersWrapper {
    pub m: Modifiers,
}

unsafe impl InvisibleSideEffectFree for ModifiersWrapper {}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CodeWrapper {
    pub c: Code,
}

unsafe impl InvisibleSideEffectFree for CodeWrapper {}

/*#[derive(Clone, Default, InvisibleSideEffectFreeDerive, MallocSizeOf)]
pub struct PreDOMString {
    pub s: String,
}

impl From<PreDOMString> for String {
    fn from(contents: PreDOMString) -> String {
        contents.s
    }
}

/*pub struct CellWrapper<T> {
    pub c: std::cell::Cell<T>
}*/

impl<T: MallocSizeOf + Copy + SecretValueSafe, L1: sec_lat::Label, L2: sec_lat::Label> MallocSizeOf for CellWrapper<InfoFlowStruct<T, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>> {
    fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
        self.c.borrow().unwrap_unsafe_dynamic_all::<L1, L2>().size_of(ops)
    }
}*/

#[derive(Clone, Default)]
pub struct SecurePart<T> {
    pub type_: T,         //this
    pub key: KeyWrapper,             //this
    pub code: T,          //this
    pub location: u32,               //this
    pub repeat: bool,                //this
    pub is_composing: bool,          //this
    pub modifiers: ModifiersWrapper, //this
    pub char_code: u32,              //this
    pub key_code: u32,               //this
}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for SecurePart<T> {}
