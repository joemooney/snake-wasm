// use std::collections::HashSet;

mod snake;
mod random;

use std::{rc::Rc, cell::RefCell};

use js_sys::Function;
pub use snake::SnakeGame;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
// the JsCast is needed for the dyn_ref below
use web_sys::{window, console, HtmlElement, HtmlDivElement, KeyboardEvent};

use crate::snake::Direction;

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> =  Rc::new(RefCell::new(SnakeGame::new(15, 15)));

    static HANDLE_TICK: Closure<dyn FnMut()> =  Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent) -> ()> = Closure::wrap(Box::new({
        |evt: KeyboardEvent| {
            GAME.with(|game| {
                console::log_1(&"keyboard...".into());
                let direction = match &evt.key()[..] {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowDown" => Some(Direction::Down),
                    "ArrowRight" => Some(Direction::Right),
                    "ArrowLeft" => Some(Direction::Left),
                    _ => None,
                };
                // console_log0("going");
                if let Some(direction) = direction {
                    console::log_1(&"change direction...".into());
                    game.borrow_mut().change_direction(direction)
                }
            });
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
}

#[wasm_bindgen(start)]
pub fn main() {
    // expects a JsValue, &str can go into()
    console::log_1(&"Starting...".into());

    // closure can only accept rust static
    // cannot move game into closure we need access to it later

    // unwrap_throw will throw exception visible in browser

    HANDLE_TICK.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 
                200,  // milliseconds
            )
            .unwrap_throw();
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback("keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(), 
            )
            .unwrap_throw();
    });

    render();

    // GAME.with(|game| game.borrow_mut().tick());
}

pub fn render() {
    let document = window()
        .unwrap_throw()
        .document()
        .unwrap_throw();

    let root_container = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    root_container.set_inner_html("");

    let width = GAME.with(|game| game.borrow().width);
    let height = GAME.with(|game| game.borrow().height);

    root_container.style().set_property("display", "inline-grid").unwrap_throw();
    root_container.style().set_property("grid-template",
        &format!("repeat({}, auto) / repeat({}, auto)", 
            height, width))
        .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            let field_element = document.create_element("div").unwrap_throw().dyn_into::<HtmlDivElement>().unwrap_throw();

            field_element.set_class_name("field");

            field_element.set_inner_text({
                if pos == GAME.with(|game| game.borrow().food) {
                    "🍎"
                } else if GAME.with(|game| game.borrow().snake.get(0) == Some(&pos)) {
                    "🐍"
                } else if GAME.with(|game| game.borrow().snake.contains(&pos)) {
                    "🟩"
                } else {
                    "⬜"
                }
            });
            root_container.append_child(&field_element).unwrap_throw();
        }
    }
}

// let (x, y) = head;
// let (x, y) = (*x as isize, *y as isize);
// if x + dx < 0 || y + dy < 0 || x + dx == self.width as isize || y + dy == self.height as isize {
//     None
// } else {
//     let (x, y) = ((x + dx) as usize, (y + dy) as usize);
//     let new_head = (x, y);
//     new_head
// }

