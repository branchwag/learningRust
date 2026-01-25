use web_sys::{Document, window};

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello world from vanilla rust. ;)");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
