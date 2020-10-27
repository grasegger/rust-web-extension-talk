use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["browser", "runtime"])]
    fn getURL(assetPath: &str) -> String;

    #[wasm_bindgen(js_namespace = ["console"])]
    fn log(message: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let logo_wrapper = document
        .query_selector("a[aria-label=\"Homepage\"]")
        .unwrap();

    // Before we create a new image element we need to get the url of the asset - which is where we need to bridge to JS by our own.
    let image_url = getURL(&"new_github_logo.png");

    // Because in the background we are using APIs that might be missing we need to unwrap this "constuctor".
    let image_element =
        HtmlImageElement::new().expect("Was not able to create new HtmlImageElement");
    image_element.set_src(&image_url);
    image_element.set_width(48);

    let image_style = image_element.style();
    image_style.set_css_text("vertical-align: middle");

    match logo_wrapper {
        Some(wrapper) => wrapper.set_inner_html(&image_element.outer_html()),
        None => log("The logo wrapper was not found."),
    }
}
