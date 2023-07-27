use crate::input2::{KeyboardButton, MouseControl, MouseControlFull};

use super::{CursorIcon, WindowLevel, WindowTheme};

pub fn convert_virtual_key_code(virtual_key_code: winit::event::VirtualKeyCode) -> KeyboardButton {
    match virtual_key_code {
        winit::event::VirtualKeyCode::Key1 => KeyboardButton::Key1,
        winit::event::VirtualKeyCode::Key2 => KeyboardButton::Key2,
        winit::event::VirtualKeyCode::Key3 => KeyboardButton::Key3,
        winit::event::VirtualKeyCode::Key4 => KeyboardButton::Key4,
        winit::event::VirtualKeyCode::Key5 => KeyboardButton::Key5,
        winit::event::VirtualKeyCode::Key6 => KeyboardButton::Key6,
        winit::event::VirtualKeyCode::Key7 => KeyboardButton::Key7,
        winit::event::VirtualKeyCode::Key8 => KeyboardButton::Key8,
        winit::event::VirtualKeyCode::Key9 => KeyboardButton::Key9,
        winit::event::VirtualKeyCode::Key0 => KeyboardButton::Key0,
        winit::event::VirtualKeyCode::A => KeyboardButton::A,
        winit::event::VirtualKeyCode::B => KeyboardButton::B,
        winit::event::VirtualKeyCode::C => KeyboardButton::C,
        winit::event::VirtualKeyCode::D => KeyboardButton::D,
        winit::event::VirtualKeyCode::E => KeyboardButton::E,
        winit::event::VirtualKeyCode::F => KeyboardButton::F,
        winit::event::VirtualKeyCode::G => KeyboardButton::G,
        winit::event::VirtualKeyCode::H => KeyboardButton::H,
        winit::event::VirtualKeyCode::I => KeyboardButton::I,
        winit::event::VirtualKeyCode::J => KeyboardButton::J,
        winit::event::VirtualKeyCode::K => KeyboardButton::K,
        winit::event::VirtualKeyCode::L => KeyboardButton::L,
        winit::event::VirtualKeyCode::M => KeyboardButton::M,
        winit::event::VirtualKeyCode::N => KeyboardButton::N,
        winit::event::VirtualKeyCode::O => KeyboardButton::O,
        winit::event::VirtualKeyCode::P => KeyboardButton::P,
        winit::event::VirtualKeyCode::Q => KeyboardButton::Q,
        winit::event::VirtualKeyCode::R => KeyboardButton::R,
        winit::event::VirtualKeyCode::S => KeyboardButton::S,
        winit::event::VirtualKeyCode::T => KeyboardButton::T,
        winit::event::VirtualKeyCode::U => KeyboardButton::U,
        winit::event::VirtualKeyCode::V => KeyboardButton::V,
        winit::event::VirtualKeyCode::W => KeyboardButton::W,
        winit::event::VirtualKeyCode::X => KeyboardButton::X,
        winit::event::VirtualKeyCode::Y => KeyboardButton::Y,
        winit::event::VirtualKeyCode::Z => KeyboardButton::Z,
        winit::event::VirtualKeyCode::Escape => KeyboardButton::Escape,
        winit::event::VirtualKeyCode::F1 => KeyboardButton::F1,
        winit::event::VirtualKeyCode::F2 => KeyboardButton::F2,
        winit::event::VirtualKeyCode::F3 => KeyboardButton::F3,
        winit::event::VirtualKeyCode::F4 => KeyboardButton::F4,
        winit::event::VirtualKeyCode::F5 => KeyboardButton::F5,
        winit::event::VirtualKeyCode::F6 => KeyboardButton::F6,
        winit::event::VirtualKeyCode::F7 => KeyboardButton::F7,
        winit::event::VirtualKeyCode::F8 => KeyboardButton::F8,
        winit::event::VirtualKeyCode::F9 => KeyboardButton::F9,
        winit::event::VirtualKeyCode::F10 => KeyboardButton::F10,
        winit::event::VirtualKeyCode::F11 => KeyboardButton::F11,
        winit::event::VirtualKeyCode::F12 => KeyboardButton::F12,
        winit::event::VirtualKeyCode::F13 => KeyboardButton::F13,
        winit::event::VirtualKeyCode::F14 => KeyboardButton::F14,
        winit::event::VirtualKeyCode::F15 => KeyboardButton::F15,
        winit::event::VirtualKeyCode::F16 => KeyboardButton::F16,
        winit::event::VirtualKeyCode::F17 => KeyboardButton::F17,
        winit::event::VirtualKeyCode::F18 => KeyboardButton::F18,
        winit::event::VirtualKeyCode::F19 => KeyboardButton::F19,
        winit::event::VirtualKeyCode::F20 => KeyboardButton::F20,
        winit::event::VirtualKeyCode::F21 => KeyboardButton::F21,
        winit::event::VirtualKeyCode::F22 => KeyboardButton::F22,
        winit::event::VirtualKeyCode::F23 => KeyboardButton::F23,
        winit::event::VirtualKeyCode::F24 => KeyboardButton::F24,
        winit::event::VirtualKeyCode::Snapshot => KeyboardButton::Snapshot,
        winit::event::VirtualKeyCode::Scroll => KeyboardButton::Scroll,
        winit::event::VirtualKeyCode::Pause => KeyboardButton::Pause,
        winit::event::VirtualKeyCode::Insert => KeyboardButton::Insert,
        winit::event::VirtualKeyCode::Home => KeyboardButton::Home,
        winit::event::VirtualKeyCode::Delete => KeyboardButton::Delete,
        winit::event::VirtualKeyCode::End => KeyboardButton::End,
        winit::event::VirtualKeyCode::PageDown => KeyboardButton::PageDown,
        winit::event::VirtualKeyCode::PageUp => KeyboardButton::PageUp,
        winit::event::VirtualKeyCode::Left => KeyboardButton::Left,
        winit::event::VirtualKeyCode::Up => KeyboardButton::Up,
        winit::event::VirtualKeyCode::Right => KeyboardButton::Right,
        winit::event::VirtualKeyCode::Down => KeyboardButton::Down,
        winit::event::VirtualKeyCode::Back => KeyboardButton::Back,
        winit::event::VirtualKeyCode::Return => KeyboardButton::Return,
        winit::event::VirtualKeyCode::Space => KeyboardButton::Space,
        winit::event::VirtualKeyCode::Compose => KeyboardButton::Compose,
        winit::event::VirtualKeyCode::Caret => KeyboardButton::Caret,
        winit::event::VirtualKeyCode::Numlock => KeyboardButton::Numlock,
        winit::event::VirtualKeyCode::Numpad0 => KeyboardButton::Numpad0,
        winit::event::VirtualKeyCode::Numpad1 => KeyboardButton::Numpad1,
        winit::event::VirtualKeyCode::Numpad2 => KeyboardButton::Numpad2,
        winit::event::VirtualKeyCode::Numpad3 => KeyboardButton::Numpad3,
        winit::event::VirtualKeyCode::Numpad4 => KeyboardButton::Numpad4,
        winit::event::VirtualKeyCode::Numpad5 => KeyboardButton::Numpad5,
        winit::event::VirtualKeyCode::Numpad6 => KeyboardButton::Numpad6,
        winit::event::VirtualKeyCode::Numpad7 => KeyboardButton::Numpad7,
        winit::event::VirtualKeyCode::Numpad8 => KeyboardButton::Numpad8,
        winit::event::VirtualKeyCode::Numpad9 => KeyboardButton::Numpad9,
        winit::event::VirtualKeyCode::AbntC1 => KeyboardButton::AbntC1,
        winit::event::VirtualKeyCode::AbntC2 => KeyboardButton::AbntC2,
        winit::event::VirtualKeyCode::NumpadAdd => KeyboardButton::NumpadAdd,
        winit::event::VirtualKeyCode::Apostrophe => KeyboardButton::Apostrophe,
        winit::event::VirtualKeyCode::Apps => KeyboardButton::Apps,
        winit::event::VirtualKeyCode::Asterisk => KeyboardButton::Asterisk,
        winit::event::VirtualKeyCode::Plus => KeyboardButton::Plus,
        winit::event::VirtualKeyCode::At => KeyboardButton::At,
        winit::event::VirtualKeyCode::Ax => KeyboardButton::Ax,
        winit::event::VirtualKeyCode::Backslash => KeyboardButton::Backslash,
        winit::event::VirtualKeyCode::Calculator => KeyboardButton::Calculator,
        winit::event::VirtualKeyCode::Capital => KeyboardButton::Capital,
        winit::event::VirtualKeyCode::Colon => KeyboardButton::Colon,
        winit::event::VirtualKeyCode::Comma => KeyboardButton::Comma,
        winit::event::VirtualKeyCode::Convert => KeyboardButton::Convert,
        winit::event::VirtualKeyCode::NumpadDecimal => KeyboardButton::NumpadDecimal,
        winit::event::VirtualKeyCode::NumpadDivide => KeyboardButton::NumpadDivide,
        winit::event::VirtualKeyCode::Equals => KeyboardButton::Equals,
        winit::event::VirtualKeyCode::Grave => KeyboardButton::Grave,
        winit::event::VirtualKeyCode::Kana => KeyboardButton::Kana,
        winit::event::VirtualKeyCode::Kanji => KeyboardButton::Kanji,
        winit::event::VirtualKeyCode::LAlt => KeyboardButton::AltLeft,
        winit::event::VirtualKeyCode::LBracket => KeyboardButton::BracketLeft,
        winit::event::VirtualKeyCode::LControl => KeyboardButton::ControlLeft,
        winit::event::VirtualKeyCode::LShift => KeyboardButton::ShiftLeft,
        winit::event::VirtualKeyCode::LWin => KeyboardButton::SuperLeft,
        winit::event::VirtualKeyCode::Mail => KeyboardButton::Mail,
        winit::event::VirtualKeyCode::MediaSelect => KeyboardButton::MediaSelect,
        winit::event::VirtualKeyCode::MediaStop => KeyboardButton::MediaStop,
        winit::event::VirtualKeyCode::Minus => KeyboardButton::Minus,
        winit::event::VirtualKeyCode::NumpadMultiply => KeyboardButton::NumpadMultiply,
        winit::event::VirtualKeyCode::Mute => KeyboardButton::Mute,
        winit::event::VirtualKeyCode::MyComputer => KeyboardButton::MyComputer,
        winit::event::VirtualKeyCode::NavigateForward => KeyboardButton::NavigateForward,
        winit::event::VirtualKeyCode::NavigateBackward => KeyboardButton::NavigateBackward,
        winit::event::VirtualKeyCode::NextTrack => KeyboardButton::NextTrack,
        winit::event::VirtualKeyCode::NoConvert => KeyboardButton::NoConvert,
        winit::event::VirtualKeyCode::NumpadComma => KeyboardButton::NumpadComma,
        winit::event::VirtualKeyCode::NumpadEnter => KeyboardButton::NumpadEnter,
        winit::event::VirtualKeyCode::NumpadEquals => KeyboardButton::NumpadEquals,
        winit::event::VirtualKeyCode::OEM102 => KeyboardButton::Oem102,
        winit::event::VirtualKeyCode::Period => KeyboardButton::Period,
        winit::event::VirtualKeyCode::PlayPause => KeyboardButton::PlayPause,
        winit::event::VirtualKeyCode::Power => KeyboardButton::Power,
        winit::event::VirtualKeyCode::PrevTrack => KeyboardButton::PrevTrack,
        winit::event::VirtualKeyCode::RAlt => KeyboardButton::AltRight,
        winit::event::VirtualKeyCode::RBracket => KeyboardButton::BracketRight,
        winit::event::VirtualKeyCode::RControl => KeyboardButton::ControlRight,
        winit::event::VirtualKeyCode::RShift => KeyboardButton::ShiftRight,
        winit::event::VirtualKeyCode::RWin => KeyboardButton::SuperRight,
        winit::event::VirtualKeyCode::Semicolon => KeyboardButton::Semicolon,
        winit::event::VirtualKeyCode::Slash => KeyboardButton::Slash,
        winit::event::VirtualKeyCode::Sleep => KeyboardButton::Sleep,
        winit::event::VirtualKeyCode::Stop => KeyboardButton::Stop,
        winit::event::VirtualKeyCode::NumpadSubtract => KeyboardButton::NumpadSubtract,
        winit::event::VirtualKeyCode::Sysrq => KeyboardButton::Sysrq,
        winit::event::VirtualKeyCode::Tab => KeyboardButton::Tab,
        winit::event::VirtualKeyCode::Underline => KeyboardButton::Underline,
        winit::event::VirtualKeyCode::Unlabeled => KeyboardButton::Unlabeled,
        winit::event::VirtualKeyCode::VolumeDown => KeyboardButton::VolumeDown,
        winit::event::VirtualKeyCode::VolumeUp => KeyboardButton::VolumeUp,
        winit::event::VirtualKeyCode::Wake => KeyboardButton::Wake,
        winit::event::VirtualKeyCode::WebBack => KeyboardButton::WebBack,
        winit::event::VirtualKeyCode::WebFavorites => KeyboardButton::WebFavorites,
        winit::event::VirtualKeyCode::WebForward => KeyboardButton::WebForward,
        winit::event::VirtualKeyCode::WebHome => KeyboardButton::WebHome,
        winit::event::VirtualKeyCode::WebRefresh => KeyboardButton::WebRefresh,
        winit::event::VirtualKeyCode::WebSearch => KeyboardButton::WebSearch,
        winit::event::VirtualKeyCode::WebStop => KeyboardButton::WebStop,
        winit::event::VirtualKeyCode::Yen => KeyboardButton::Yen,
        winit::event::VirtualKeyCode::Copy => KeyboardButton::Copy,
        winit::event::VirtualKeyCode::Paste => KeyboardButton::Paste,
        winit::event::VirtualKeyCode::Cut => KeyboardButton::Cut,
    }
}

pub fn convert_cursor_icon(cursor_icon: CursorIcon) -> winit::window::CursorIcon {
    match cursor_icon {
        CursorIcon::Default => winit::window::CursorIcon::Default,
        CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
        CursorIcon::Hand => winit::window::CursorIcon::Hand,
        CursorIcon::Arrow => winit::window::CursorIcon::Arrow,
        CursorIcon::Move => winit::window::CursorIcon::Move,
        CursorIcon::Text => winit::window::CursorIcon::Text,
        CursorIcon::Wait => winit::window::CursorIcon::Wait,
        CursorIcon::Help => winit::window::CursorIcon::Help,
        CursorIcon::Progress => winit::window::CursorIcon::Progress,
        CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
        CursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
        CursorIcon::Cell => winit::window::CursorIcon::Cell,
        CursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
        CursorIcon::Alias => winit::window::CursorIcon::Alias,
        CursorIcon::Copy => winit::window::CursorIcon::Copy,
        CursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
        CursorIcon::Grab => winit::window::CursorIcon::Grab,
        CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
        CursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
        CursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
        CursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
        CursorIcon::EResize => winit::window::CursorIcon::EResize,
        CursorIcon::NResize => winit::window::CursorIcon::NResize,
        CursorIcon::NeResize => winit::window::CursorIcon::NeResize,
        CursorIcon::NwResize => winit::window::CursorIcon::NwResize,
        CursorIcon::SResize => winit::window::CursorIcon::SResize,
        CursorIcon::SeResize => winit::window::CursorIcon::SeResize,
        CursorIcon::SwResize => winit::window::CursorIcon::SwResize,
        CursorIcon::WResize => winit::window::CursorIcon::WResize,
        CursorIcon::EwResize => winit::window::CursorIcon::EwResize,
        CursorIcon::NsResize => winit::window::CursorIcon::NsResize,
        CursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
        CursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
        CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
        CursorIcon::RowResize => winit::window::CursorIcon::RowResize,
    }
}

pub fn convert_window_level(window_level: WindowLevel) -> winit::window::WindowLevel {
    match window_level {
        WindowLevel::AlwaysOnBottom => winit::window::WindowLevel::AlwaysOnBottom,
        WindowLevel::Normal => winit::window::WindowLevel::Normal,
        WindowLevel::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
    }
}

pub fn convert_winit_theme(theme: winit::window::Theme) -> WindowTheme {
    match theme {
        winit::window::Theme::Light => WindowTheme::Light,
        winit::window::Theme::Dark => WindowTheme::Dark,
    }
}

pub fn convert_window_theme(theme: WindowTheme) -> winit::window::Theme {
    match theme {
        WindowTheme::Light => winit::window::Theme::Light,
        WindowTheme::Dark => winit::window::Theme::Dark,
    }
}

pub fn convert_element_state(element_state: winit::event::ElementState) -> f32 {
    match element_state {
        winit::event::ElementState::Pressed => 1.0,
        winit::event::ElementState::Released => 0.0,
    }
}

pub fn convert_mouse_button(mouse_button: winit::event::MouseButton) -> MouseControlFull {
    match mouse_button {
        winit::event::MouseButton::Left => MouseControlFull::MouseControl(MouseControl::Left),
        winit::event::MouseButton::Right => MouseControlFull::MouseControl(MouseControl::Right),
        winit::event::MouseButton::Middle => MouseControlFull::MouseControl(MouseControl::Middle),
        winit::event::MouseButton::Other(val) => match val {
            4 => MouseControlFull::MouseControl(MouseControl::X1), // TODO: test these ids are correct on all platforms
            5 => MouseControlFull::MouseControl(MouseControl::X2),
            _ => MouseControlFull::Other(val),
        },
    }
}
