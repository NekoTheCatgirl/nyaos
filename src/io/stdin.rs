use core::sync::atomic::{AtomicBool, Ordering};

use alloc::string::String;
use conquer_once::spin::OnceCell;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::task::keyboard::ScancodeStream;

static READ_KEYBOARD: AtomicBool = AtomicBool::new(false);
static STDIO_STREAM: OnceCell<Mutex<String>> = OnceCell::uninit();

/// Reads from the character buffer from [STDIO_STREAM] and returns a String object when a '\n' is reached
pub fn read_line() -> String {
    let mut scancode_stream = ScancodeStream::new();
    let mut input = String::new();

    READ_KEYBOARD.store(true, Ordering::Relaxed);

    input
}

/// This function ensures that [read_line] works
pub async fn stdin_handler() {
    STDIO_STREAM
        .try_init_once(|| Mutex::new(String::new()))
        .expect("Unable to init STDIO Stream");
    
    loop {
        
    }
}
