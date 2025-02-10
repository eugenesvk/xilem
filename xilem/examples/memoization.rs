#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
use xilem::view::{button, flex, label, prose, Axis};
use xilem::{AnyWidgetView, EventLoop, WidgetView, Xilem};

struct AppState {count: i32,}
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
  // works
  // flex((
  //   label("Label→"),
  //   label("←LabelEnd"),
  // )).direction(Axis::Horizontal)

  // fails
  flex((
    prose("Prose→"),
    prose("←ProseEnd"),
  )).direction(Axis::Horizontal)
}
fn main() {
  let data = AppState {count: 0};
  let app = Xilem::new(data, app_logic);
  app.run_windowed(EventLoop::with_user_event(), "Memoization".into()).unwrap();
}
