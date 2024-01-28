use wasm_bindgen::prelude::*;
use web_sys::{
    window,
    Document,
    Element,
    Event,
};
use wasm_bindgen::JsCast;

fn set_item_in_local_storage(
    key: &str,
    value: &str
)
{
    let local_storage = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    local_storage
        .set_item(key, value)
        .unwrap();
}

pub fn write_sidebar_items_as_stringified_json_to_local_storage(
    sidebar_items_as_stringified_json: &str
)
{
    set_item_in_local_storage(
        "hackernews_userscript",
        sidebar_items_as_stringified_json
    );
}

pub fn write_settings_as_stringified_json_to_local_storage(
    settings_as_stringified_json: &str
)
{
    set_item_in_local_storage(
        "settings",
        settings_as_stringified_json
    );
}

pub fn write_athing_id_to_submission_title_as_stringified_json_to_local_storage(
    athing_id_to_submission_title_as_stringified_json: &str
)
{
    set_item_in_local_storage(
        "softhidden_things",
        athing_id_to_submission_title_as_stringified_json
    );
}
