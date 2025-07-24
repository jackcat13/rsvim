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

  if let Some(command) = command.to_object(scope) {
    if let Ok(callback) = v8::Local::<v8::Function>::try_from(command) {
      global.set(scope, name, callback.into());
    }
  }
}
