use std::cell::RefCell;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW,
    WH_KEYBOARD_LL,
    HHOOK, MSG,
};

use crate::core::Disposition;


thread_local! {
    static HANDLER: RefCell<Option<Box<dyn LowLevelHookHandler>>> = RefCell::new(None);
}


unsafe extern "system" fn low_level_keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let disposition = HANDLER.with(|refcell| {
        refcell.try_borrow()
            .ok()
            .and_then(|boxed| {
                boxed.as_ref().map(|handler| {
                    handler.handle(code, wparam, lparam)
                })
            })
        })
        .unwrap_or(Disposition::Passthrough);

    match disposition {
        Disposition::Passthrough => unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) },
        Disposition::Swallow => LRESULT(1),
    }
}


pub trait LowLevelHookHandler {
    fn handle(&self, code: i32, wparam: WPARAM, lparam: LPARAM) -> Disposition;
}


pub struct WindowsHook {
    hook: Option<HHOOK>,
}

impl WindowsHook {

    pub fn new() -> Self {
        WindowsHook {
            hook: None,
        }
    }


    pub fn install(&mut self, handler: Box<dyn LowLevelHookHandler>) -> Result<(), String> {
        let hook = unsafe {
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_hook), None, 0)
        }.map_err(|e| format!("SetWindowsHookExW failed: {e}"))?;

        HANDLER.with(|refcell| { *refcell.borrow_mut() = Some(handler) });

        self.hook = Some(hook);

        Ok(())
    }


    pub fn activate(&self) -> Result<(), String> {
        let mut msg = MSG::default();

        let _ = unsafe { GetMessageW(&mut msg, None, 0, 0) }.as_bool();

        Ok(())
    }

}
