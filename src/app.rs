use crate::{
  detail_view,
  home,
  types::{
    AppState,
    Page,
  },
};
use smithy::{
  smd,
  types::Component,
};

pub fn render(app_state: AppState) -> impl smithy::types::Component {
  let AppState {
    mut current_page,
    mut todo_lists,
  } = app_state;
  smd!(
    on_hash_change={|_| {
      &mut current_page.handle_hash_change();
    }};
    {
      match current_page {
        Page::Home(ref mut home_info) => {
          home::render_home_page(&mut todo_lists, home_info)
        },
        Page::TodoListDetail((ref id, ref mut input_dom_ref, ref mut input_text, ref mut showing)) => {
          detail_view::render_detail_view_page(&mut todo_lists, *id, input_dom_ref, input_text, showing)
        },
      }
    }
  )
}
