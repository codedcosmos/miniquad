//! OS clipboard abstraction

use crate::Context;

/// Get current OS clipboard value
pub fn get(_ctx: &mut Context) -> Option<String> {
    //ctx.window_requests.clipboard.clone()
    unimplemented!()
}

/// Save value to OS clipboard
pub fn set(_ctx: &mut Context, _data: &str) {
    //ctx.window_requests.update_clipboard_requested = Some(data.to_owned());
    unimplemented!()
}
