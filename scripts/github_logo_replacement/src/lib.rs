use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["browser", "runtime"])]
    fn getURL(assetPath: &str) -> String;
}

#[wasm_bindgen(start)]
pub fn main() {
    // So, this is there my code should go.
    // 1) Delete old SVG node, while remembering the parent to attach a picture
    // 2) Create a new image source and insert that into the old node

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut logo_wrapper = document
        .query_selector("a[aria-label=\"Homepage\"]")
        .unwrap()
        .unwrap();
    logo_wrapper.set_inner_html(""); // There is no content anymore :)

    // Before we create a new image element we need to get the url of the asset - which is where we need to bridge to JS by our own.
    let image_url = getURL(&"new_github_logo.png");

    let mut image_element = HtmlImageElement::new().unwrap();
    image_element.set_src(&image_url);
    image_element.set_width(48);
    let mut image_style = image_element.style();
    image_style.set_css_text("vertical-align: middle");

    logo_wrapper.set_inner_html(&image_element.outer_html());
}
