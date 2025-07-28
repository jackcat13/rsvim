use crate::js::JsRuntime;
use crate::js::msg::{CreateCommandFeedback, JsRuntimeToEventLoopMessage};
use compact_str::CompactString;

/// Javascript `Rsvim.createCommand` API.
pub fn create_command(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  _: v8::ReturnValue,
) {
  let name = args.get(0);
  let command = args.get(1);
  let context = scope.get_current_context();
  let global = context.global(scope);
  let tc_scope = &mut v8::TryCatch::new(scope);

  // Create function in JS runtime if does not already exist
  if !global.get(tc_scope, name).unwrap().is_undefined() {
    let msg = format!(
      "Command '{}' already exists and can't be created. It happened registering following function : {}",
      name.to_rust_string_lossy(tc_scope),
      command.to_rust_string_lossy(tc_scope)
    );
    let msg = v8::String::new(tc_scope, &msg).unwrap();
    let exception = v8::Exception::type_error(tc_scope, msg);
    tc_scope.throw_exception(exception);
  } else {
    let command = command.to_object(tc_scope).unwrap();
    let callback = v8::Local::<v8::Function>::try_from(command).unwrap();
    global.set(tc_scope, name, callback.into());
  }

  // Report if function creation threw an exception.
  let feedback = if tc_scope.has_caught() {
    let exception = tc_scope.exception().unwrap();
    let message = CompactString::new(exception.to_rust_string_lossy(tc_scope));
    CreateCommandFeedback::Error(message)
  } else {
    CreateCommandFeedback::Created
  };

  let state_rc = JsRuntime::state(tc_scope);
  let state = state_rc.borrow_mut();
  let jsrt_to_mstr = state.jsrt_to_mstr.clone();
  let current_handle = tokio::runtime::Handle::current();
  current_handle.spawn_blocking(move || {
    jsrt_to_mstr
      .blocking_send(JsRuntimeToEventLoopMessage::CreateCommandFeedbackReq(
        feedback,
      ))
      .unwrap();
  });
}
