use std::fmt;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VIRTUAL_KEY,
    VK_CONTROL, VK_LCONTROL, VK_RCONTROL,
    VK_SHIFT, VK_LSHIFT, VK_RSHIFT,
    VK_MENU, VK_LMENU, VK_RMENU,
};
use windows::Win32::UI::WindowsAndMessaging::{
    KBDLLHOOKSTRUCT,
    WM_KEYDOWN, WM_SYSKEYDOWN,
};

use crate::Runtime;
use crate::core::key::Modifier;


pub(in crate::platform::windows) trait FromVk: Copy {
    type Output;

    fn to_modifier(self) -> Self::Output;
}

impl FromVk for u16 {
    type Output = Modifier;

    fn to_modifier(self) -> Self::Output {
        let vk = self;

        let is_ctrl_down  = vk == VK_CONTROL.0 || vk == VK_LCONTROL.0 || vk == VK_RCONTROL.0;
        let is_shift_down = vk == VK_SHIFT.0   || vk == VK_LSHIFT.0   || vk == VK_RSHIFT.0;
        let is_alt_down   = vk == VK_MENU.0    || vk == VK_LMENU.0    || vk == VK_RMENU.0;

        if is_ctrl_down {
            return Modifier::CTRL
        }

        if is_shift_down {
            return Modifier::SHIFT
        }

        if is_alt_down {
            return Modifier::ALT
        }

        Modifier::empty()
    }

}


pub(in crate::platform::windows) enum Kind {
    KeyDown,
    KeyUp,
    ModsDown,
    ModsUp,
}


pub(in crate::platform::windows) struct LowKeyEvent {
    pub vk: u16,
    pub mods: Modifier,
    pub activated_mods: Modifier,
    pub is_key_down: bool,
    pub is_injected: bool,
}


impl LowKeyEvent {

    pub fn new(wparam: WPARAM, lparam: LPARAM) -> Self {
        let kb = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };

        let vk = u16::try_from(kb.vkCode).expect("vkCode exceeds u16 range");
        let mods = vk.to_modifier();
        let activated_mods = Self::activated_mods();
        let is_key_down = match wparam.0 as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => true,
            _                          => false,
        };
        let is_injected = kb.dwExtraInfo == Runtime::SERIAL_ID;

        Self {
            vk,
            mods,
            activated_mods,
            is_key_down,
            is_injected,
        }
    }


    pub fn kind(&self) -> Kind {
        let mods_event = self.mods != Modifier::empty();

        match (mods_event, self.is_key_down) {
            (false, false) => Kind::KeyUp,
            (false, true)  => Kind::KeyDown,
            (true,  false) => Kind::ModsUp,
            _              => Kind::ModsDown,
        }
    }


    pub fn is_injected(&self) -> bool {
        self.is_injected
    }


    pub fn active_modifiers(&self) -> Modifier {
        self.mods | self.activated_mods
    }


    pub fn to_raw_chord(self) -> RawChord {
        RawChord {
            vk: self.vk,
            mods: self.active_modifiers(),
        }
    }


    fn activated_mods() -> Modifier {
        let mut mods = Modifier::empty();

        if Self::is_down(VK_CONTROL) { mods |= Modifier::CTRL;  }
        if Self::is_down(VK_SHIFT)   { mods |= Modifier::SHIFT; }
        if Self::is_down(VK_MENU)    { mods |= Modifier::ALT;   }

        mods
    }


    fn is_down(vk: VIRTUAL_KEY) -> bool {
        unsafe { GetAsyncKeyState(vk.0 as i32) as u16 & 0x8000 != 0 }
    }

}


impl fmt::Display for LowKeyEvent {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "low key event(mods={}({}), vk={} {}){}",
            self.mods,
            self.activated_mods,
            self.vk,
            match self.is_key_down {
                true  => "key down",
                false => "key up",
            },
            match self.is_injected {
                true  => "[injected]",
                false => "",
            }

        );

        Ok(())
    }

}


pub struct RawChord {
    pub vk: u16,
    pub mods: Modifier,
}
