#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
use xilem::view::{button, flex, label, prose, inline_prose, Axis, FlexExt,};
use xilem::{AnyWidgetView, EventLoop, WidgetView, Xilem};

struct AppState {count: i32,}
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
  // works
  // flex((
  //   label("Label→"),
  //   label("←LabelEnd"),
  // )).direction(Axis::Horizontal)

  // fails, only Prose→ is visible
  // flex((
  //   prose("Prose→"),
  //   prose("←ProseEnd"),
  // )).direction(Axis::Horizontal)

  // fails, both are visible, but window can't be resized and has height=monitor's height on Windows
  flex((
    prose("Prose→"   ).flex(1.0),
    prose("←ProseEnd").flex(1.0),
  )).direction(Axis::Horizontal)

  // fails, both are visible (and without a gap unlike ↑ 1.0), but window can't be resized and has height=monitor's height on Windows
  // flex((
  //   inline_prose("Prose→"),
  //   inline_prose("←ProseEnd"),
  // )).direction(Axis::Horizontal)

}
fn main() {
  let data = AppState {count: 0};
  let app = Xilem::new(data, app_logic);
  app.run_windowed(EventLoop::with_user_event(), "Memoization".into()).unwrap();
}
