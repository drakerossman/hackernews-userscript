use wasm_bindgen::prelude::*;
use web_sys::{
    Document,
    Element,
    Event,
    *,
};
use indexmap::IndexMap;
use wasm_bindgen::JsCast;

use gloo_console::log as gloo_log;

#[wasm_bindgen]
pub fn obtain_document() -> Document
{
    let window = web_sys::window().expect("no global `window` exists");

    window
        .document()
        .expect("should have a document on window")
}

pub fn obtain_filter_summary_text_to_filter_target_text_from_local_storage(
) -> IndexMap<String, String>
{
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let hackernew_userscript_data_as_string_option = local_storage
        .get_item("hackernews_userscript")
        .unwrap()
        .unwrap_or_else(
            || {
                let stringified_json_with_single_key = "{\"sidebar_items\": []}";

                local_storage
                    .set_item("hackernews_userscript", &stringified_json_with_single_key)
                    .unwrap();

                stringified_json_with_single_key.to_owned()
            },
        );

    let hackernew_userscript_data_as_string = hackernew_userscript_data_as_string_option;

    let hackernew_userscript_data_as_json: serde_json::Value =
        serde_json::from_str(&hackernew_userscript_data_as_string).unwrap();

    let sidebar_items_as_json = hackernew_userscript_data_as_json
        .as_object()
        .unwrap()
        .get("sidebar_items")
        .unwrap();

    let sidebar_items_as_vec_of_tuples_with_two_strings: Vec<(String, String)> =
        serde_json::from_value(sidebar_items_as_json.clone()).unwrap();

    let sidebar_items_as_indexmap: IndexMap<String, String> =
        sidebar_items_as_vec_of_tuples_with_two_strings
            .into_iter()
            .collect();

    sidebar_items_as_indexmap
}

pub fn obtain_settings_from_local_storage(
) -> serde_json::Value
{
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let settings_as_string_option = local_storage
        .get_item("settings")
        .unwrap()
        .unwrap_or_else(
            || {
                let stringified_json_with_single_key = "{}";

                local_storage
                    .set_item("settings", &stringified_json_with_single_key)
                    .unwrap();

                stringified_json_with_single_key.to_owned()
            },
        );

    let settings_as_string = settings_as_string_option;

    let settings_as_json: serde_json::Value =
        serde_json::from_str(&settings_as_string).unwrap();

    settings_as_json
}

pub fn obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust(
) -> Result<(Vec<Element>, Vec<Element>, Vec<Element>, Vec<Element>), JsValue>
{
    let document = obtain_document();

    let body = document
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .ok_or_else(|| JsValue::from_str("No body element found"))?;

    let list_of_titleline_spans_as_js = body.query_selector_all(".athing .titleline")?;

    let list_of_titleline_spans_as_rust: Vec<Element> = (0..list_of_titleline_spans_as_js.length())
        .map(|index| {
            list_of_titleline_spans_as_js
                .item(index)
                .ok_or_else(|| JsValue::from_str("Missing element at index"))
                .and_then(|item| {
                    item.dyn_into::<Element>()
                        .map_err(|_| JsValue::from_str("Failed to cast to Element"))
                })
        })
        .collect::<Result<_, _>>()?;

    let list_of_links_as_rust = list_of_titleline_spans_as_rust
        .iter()
        .map(|titleline_span| {
            titleline_span
                .query_selector("a")
                .map_err(|_| JsValue::from_str("Failed to query_selector for 'a'"))?
                .ok_or_else(|| JsValue::from_str("No 'a' element found"))
                .and_then(|link| {
                    link.dyn_into::<Element>()
                        .map_err(|_| JsValue::from_str("Failed to cast to Element"))
                })
        })
        .collect::<Result<_, _>>()?;

    let list_of_hnusers_as_js = body.query_selector_all(".hnuser")?;

    let list_of_hnusers_as_rust: Vec<Element> = (0..list_of_hnusers_as_js.length())
        .map(|index| {
            list_of_hnusers_as_js
                .item(index)
                .ok_or_else(|| JsValue::from_str("Missing hnuser at index"))
                .and_then(|item| {
                    item.dyn_into::<Element>()
                        .map_err(|_| JsValue::from_str("Failed to cast to Element"))
                })
        })
        .collect::<Result<_, _>>()?;

    let list_of_comments_as_js = body.query_selector_all(".commtext")?;

    let list_of_comments_as_rust: Vec<Element> = (0..list_of_comments_as_js.length())
        .map(|index| {
            list_of_comments_as_js
                .item(index)
                .ok_or_else(|| JsValue::from_str("Missing comment at index"))
                .and_then(|item| {
                    item.dyn_into::<Element>()
                        .map_err(|_| JsValue::from_str("Failed to cast to Element"))
                })
        })
        .collect::<Result<_, _>>()?;

    Ok(
        (
            list_of_titleline_spans_as_rust,
            list_of_links_as_rust,
            list_of_comments_as_rust,
            list_of_hnusers_as_rust,
        )
    )
}

fn value_to_indexmap(value: &serde_json::Value) -> Option<IndexMap<String, String>> {
    let mut map = IndexMap::new();

    if let serde_json::Value::Object(obj) = value {
        for (key, val) in obj {
            let key_str = key.to_string();
            let val_str = val.as_str().unwrap_or("").to_string();
            map.insert(key_str, val_str);
        }
        Some(map)
    } else {
        None
    }
}


pub fn obtain_athing_id_to_submission_title_from_local_storage(
) -> IndexMap<String, String>
{
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let athing_id_to_submission_title_as_string_option = local_storage
        .get_item("softhidden_things")
        .unwrap()
        .unwrap_or_else(
            || {
                let stringified_json_with_single_key = "{}";

                local_storage
                    .set_item("softhidden_things", &stringified_json_with_single_key)
                    .unwrap();

                stringified_json_with_single_key.to_owned()
            },
        );

    let athing_id_to_submission_title_as_string = athing_id_to_submission_title_as_string_option;

    let athing_id_to_submission_title_as_json: serde_json::Value =
        serde_json::from_str(&athing_id_to_submission_title_as_string).unwrap();

    let athing_id_to_submission_title_indexmap: IndexMap<String, String> =
        value_to_indexmap(&athing_id_to_submission_title_as_json).unwrap();

    athing_id_to_submission_title_indexmap
}

