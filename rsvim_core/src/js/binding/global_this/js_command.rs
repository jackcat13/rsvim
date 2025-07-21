use crate::js::{JsFuture, JsRuntime};
use compact_str::CompactString;

/// Create a future to execute any JS command from APIs
pub struct JsCommandFuture {
  command: CompactString,
}

impl JsCommandFuture {
    pub fn new(command: CompactString) -> Self {
        JsCommandFuture {
            command
        }
    }
}

impl JsFuture for JsCommandFuture {
  fn run(&mut self, scope: &mut v8::HandleScope) {
    todo!("Js command execution");
  }
}
