use base64;
use wasm_bindgen::prelude::*;
use web_sys::*;
#[macro_use]
mod util;
use std::iter::Iterator;
use std::collections::HashMap;

use indexmap::IndexMap;
use serde_json::{json, Value, to_string};
use regex::Regex;
use wasm_bindgen::JsCast;
use gloo_console::log as gloo_log;
use gloo::events::EventListener;
use web_sys::{
    Document,
    Element,
    Event,
    HtmlButtonElement,
    KeyboardEvent,
    HtmlInputElement,
    HtmlImageElement,
};

mod getters;
use getters::{
    obtain_document,
    obtain_filter_summary_text_to_filter_target_text_from_local_storage,
    obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust,
};

mod setters;
use setters::{
    write_sidebar_items_as_stringified_json_to_local_storage,
    write_athing_id_to_submission_title_as_stringified_json_to_local_storage,
};

mod dom_manipulation;
use dom_manipulation::{
    hide_specific_item_of_a_specific_category,
    load_items_from_locastorage_populate_sidebar_and_apply_filters,
    parse_string_with_css_and_insert_into_header,
    parse_string_with_sidebar_and_insert_into_body,
    remove_all_children_of_filter_items_container,
    replace_center_tag_with_div,
    select_submission_elements_and_set_their_style,
    set_specific_radiobutton_for_category_as_checked,
};

mod pure;
use pure::stringify_filter_summary_text_to_filter_target_text_from_map;

#[wasm_bindgen]
pub fn register_sidebar_item_event_listener() -> Result<(), JsValue>
{
    let document = obtain_document();

    let body = document
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap();

    let result =
        obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust();
    let (
        list_of_titleline_spans_as_rust,
        list_of_links_as_rust,
        list_of_comments_as_rust,
        list_of_hnusers_as_rust,
    ) =
        match result
        {
            | Ok(value) => value,
            | Err(err) => return Err(err),
        };

    let sidebar_container = &document
        .get_element_by_id("filter-items-container")
        .unwrap();

    let sidebar_container: Element = sidebar_container
        .clone()
        .dyn_into()?;

    let document_ref = document;

    let list_of_titleline_spans_as_rust_ref = list_of_titleline_spans_as_rust;

    let listener = EventListener::new(&sidebar_container, "click", move |event: &Event| {
        let ic_remove_or_filter_selector_wrapper = event
            .target()
            .unwrap();

        if let Ok(ic_remove_or_filter_selector_wrapper) =
            ic_remove_or_filter_selector_wrapper.dyn_into::<web_sys::Element>()
        {
            match ic_remove_or_filter_selector_wrapper
                .class_name()
                .as_str()
            {
                | "ic-remove" =>
                {
                    let filter_item_wrapper = ic_remove_or_filter_selector_wrapper
                        .parent_node()
                        .unwrap();

                    let filter_item_keyword_wrapper = filter_item_wrapper
                        .first_child()
                        .unwrap();

                    let filter_item_keyword_wrapper_as_el = filter_item_keyword_wrapper
                        .dyn_into::<Element>()
                        .unwrap();

                    let filter_item_keyword = filter_item_keyword_wrapper_as_el
                        .query_selector(".filter-item-keyword")
                        .unwrap();

                    let filter_item_keyword_as_el: Element = filter_item_keyword
                        .unwrap()
                        .dyn_into()
                        .unwrap();

                    let text_of_filter_item_keyword = filter_item_keyword_as_el
                        .text_content()
                        .unwrap();

                    let text_of_filter_item_keyword = text_of_filter_item_keyword
                        .trim()
                        .to_owned();

                    let mut filter_summary_text_to_filter_target =
                        obtain_filter_summary_text_to_filter_target_text_from_local_storage();

                    filter_summary_text_to_filter_target.remove(&text_of_filter_item_keyword);

                    let filter_summary_text_to_filter_target_as_stringified_json =
                        stringify_filter_summary_text_to_filter_target_text_from_map(
                            &filter_summary_text_to_filter_target,
                        );

                    write_sidebar_items_as_stringified_json_to_local_storage(
                        &filter_summary_text_to_filter_target_as_stringified_json,
                    );
                },

                | "filter-selector-wrapper" =>
                {
                    let filter_item_wrapper_as_node = ic_remove_or_filter_selector_wrapper
                        .parent_node()
                        .unwrap();

                    let filter_item_keyword_wrapper = filter_item_wrapper_as_node
                        .first_child()
                        .unwrap();

                    let filter_item_keyword_wrapper_as_el = filter_item_keyword_wrapper
                        .dyn_into::<Element>()
                        .unwrap();

                    let filter_item_keyword = filter_item_keyword_wrapper_as_el
                        .query_selector(".filter-item-keyword")
                        .unwrap();

                    let filter_item_keyword_as_el: Element = filter_item_keyword
                        .unwrap()
                        .dyn_into()
                        .unwrap();

                    let text_of_filter_item_keyword = filter_item_keyword_as_el
                        .text_content()
                        .unwrap();

                    let text_of_filter_item_keyword = text_of_filter_item_keyword
                        .trim()
                        .to_owned();

                    let filter_item_wrapper: Element = filter_item_wrapper_as_node
                        .clone()
                        .dyn_into()
                        .unwrap();

                    let checkbox_container_div_children = filter_item_wrapper
                        .query_selector_all("input[type='checkbox']")
                        .unwrap();

                    let checkbox_container_div_children_length =
                        checkbox_container_div_children.length();

                    let checkbox_container_div_children_as_vec_of_els: Vec<Element> = (1
                        ..checkbox_container_div_children_length)
                        .map(|index| {
                            checkbox_container_div_children
                                .item(index)
                                .unwrap()
                        })
                        .map(|item| {
                            item.dyn_into::<Element>()
                                .unwrap()
                        })
                        .collect();

                    let checkbox_container_div_children_as_vec_of_els_length =
                        checkbox_container_div_children_as_vec_of_els.len();

                    let checkbox_id_to_checkbox_toggle_state = (0
                        ..checkbox_container_div_children_as_vec_of_els_length)
                        .map(|index| {
                            let checkbox_id = checkbox_container_div_children_as_vec_of_els
                                .get(index)
                                .unwrap()
                                .get_attribute("id")
                                .unwrap();

                            let checkbox_toggle_state =
                                checkbox_container_div_children_as_vec_of_els
                                    .get(index)
                                    .unwrap()
                                    .clone()
                                    .dyn_into::<HtmlInputElement>()
                                    .unwrap()
                                    .checked();

                            (checkbox_id, checkbox_toggle_state)
                        })
                        .collect::<HashMap<_, _>>();

                    let checkbox_id_to_checkbox_toggle_state_as_stringified_json =
                        serde_json::to_string(&checkbox_id_to_checkbox_toggle_state).unwrap();

                    let some_checkboxes_are_true = checkbox_id_to_checkbox_toggle_state
                        .values()
                        .any(|checkbox_toggle_state| *checkbox_toggle_state);

                    let vector_of_all_checkbox_ids_that_are_true =
                        checkbox_id_to_checkbox_toggle_state
                            .iter()
                            .filter(|(_checkbox_id, checkbox_toggle_state)| **checkbox_toggle_state)
                            .map(|(checkbox_id, _checkbox_toggle_state)| checkbox_id)
                            .collect::<Vec<&String>>();

                    let vector_of_all_checkbox_ids_that_are_true =
                        vector_of_all_checkbox_ids_that_are_true
                            .iter()
                            .map(|s| {
                                let mut s = s.clone();
                                let mut s = s
                                    .split("-")
                                    .collect::<Vec<_>>();

                                let mut s = s
                                    .iter_mut()
                                    .map(|s| {
                                        let mut s = s
                                            .chars()
                                            .collect::<Vec<_>>();
                                        s[0] = s[0].to_ascii_uppercase();
                                        s.iter()
                                            .collect::<String>()
                                    })
                                    .collect::<Vec<_>>();

                                let kek = s
                                    .get(1)
                                    .unwrap()
                                    .to_string();
                                kek

                            })
                            .collect::<Vec<_>>();

                    let vector_of_all_checkbox_ids_that_are_true_as_joined_string =
                        vector_of_all_checkbox_ids_that_are_true
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(",");

                    let mut filter_summary_text_to_filter_target_text_ =
                        obtain_filter_summary_text_to_filter_target_text_from_local_storage();

                    filter_summary_text_to_filter_target_text_.insert(
                        text_of_filter_item_keyword,
                        vector_of_all_checkbox_ids_that_are_true_as_joined_string,
                    );

                    let filter_summary_text_to_filter_target_text_as_stringified_json =
                        stringify_filter_summary_text_to_filter_target_text_from_map(
                            &filter_summary_text_to_filter_target_text_,
                        );

                    write_sidebar_items_as_stringified_json_to_local_storage(
                        &filter_summary_text_to_filter_target_text_as_stringified_json,
                    );

                    event.stop_propagation();
                },

                | _ =>
                {
                    let mut parent = ic_remove_or_filter_selector_wrapper.parent_element();

                    let mut found = false;

                    let mut parent_clone = parent.clone();

                    while let Some(element) = parent
                    {
                        if let Some(class_string) = element.get_attribute("class")
                        {
                            let class_list: Vec<&str> = class_string
                                .split_whitespace()
                                .collect();

                            if class_list.contains(&"filter-selector-wrapper")
                            {
                                found = true;

                                event.stop_propagation();

                                break;
                            }
                        }

                        parent = element.parent_element();

                        parent_clone = parent.clone()
                    }

                    if found
                    {
                        let filter_item_wrapper_as_node = parent_clone
                            .expect("no parent found")
                            .parent_node()
                            .unwrap();

                        let filter_item_wrapper: Element = filter_item_wrapper_as_node
                            .clone()
                            .dyn_into()
                            .unwrap();

                        let filter_item_keyword_wrapper = filter_item_wrapper
                            .first_child()
                            .unwrap();

                        let filter_item_keyword_wrapper_as_el = filter_item_keyword_wrapper
                            .dyn_into::<Element>()
                            .unwrap();

                        let filter_item_keyword = filter_item_keyword_wrapper_as_el
                            .query_selector(".filter-item-keyword")
                            .unwrap();

                        let filter_item_keyword_as_el: Element = filter_item_keyword
                            .unwrap()
                            .dyn_into()
                            .unwrap();

                        let text_of_filter_item_keyword = filter_item_keyword_as_el
                            .text_content()
                            .unwrap();

                        let text_of_filter_item_keyword = text_of_filter_item_keyword
                            .trim()
                            .to_owned();

                        let checkbox_container_div_children = filter_item_wrapper
                            .query_selector_all("input[type='checkbox']")
                            .unwrap();

                        let checkbox_container_div_children_length =
                            checkbox_container_div_children.length();

                        let checkbox_container_div_children_as_vec_of_els: Vec<Element> = (1
                            ..checkbox_container_div_children_length)
                            .map(|index| {
                                checkbox_container_div_children
                                    .item(index)
                                    .unwrap()
                            })
                            .map(|item| {
                                item.dyn_into::<Element>()
                                    .unwrap()
                            })
                            .collect();

                        let checkbox_container_div_children_as_vec_of_els_length =
                            checkbox_container_div_children_as_vec_of_els.len();

                        let checkbox_id_to_checkbox_toggle_state = (0
                            ..checkbox_container_div_children_as_vec_of_els_length)
                            .map(|index| {
                                let checkbox_id = checkbox_container_div_children_as_vec_of_els
                                    .get(index)
                                    .unwrap()
                                    .get_attribute("id")
                                    .unwrap();

                                let checkbox_toggle_state =
                                    checkbox_container_div_children_as_vec_of_els
                                        .get(index)
                                        .unwrap()
                                        .clone()
                                        .dyn_into::<HtmlInputElement>()
                                        .unwrap()
                                        .checked();

                                (checkbox_id, checkbox_toggle_state)
                            })
                            .collect::<HashMap<_, _>>();

                        let checkbox_id_to_checkbox_toggle_state_as_stringified_json =
                            serde_json::to_string(&checkbox_id_to_checkbox_toggle_state).unwrap();

                        let some_checkboxes_are_true = checkbox_id_to_checkbox_toggle_state
                            .values()
                            .any(|checkbox_toggle_state| *checkbox_toggle_state);

                        let vector_of_all_checkbox_ids_that_are_true =
                            checkbox_id_to_checkbox_toggle_state
                                .iter()
                                .filter(|(_checkbox_id, checkbox_toggle_state)| {
                                    **checkbox_toggle_state
                                })
                                .map(|(checkbox_id, _checkbox_toggle_state)| checkbox_id)
                                .collect::<Vec<&String>>();

                        let vector_of_all_checkbox_ids_that_are_true =
                            vector_of_all_checkbox_ids_that_are_true
                                .iter()
                                .map(|s| {
                                    let mut s = s.clone();
                                    let mut s = s
                                        .split("-")
                                        .collect::<Vec<_>>();

                                    let mut s = s
                                        .iter_mut()
                                        .map(|s| {
                                            let mut s = s
                                                .chars()
                                                .collect::<Vec<_>>();
                                            s[0] = s[0].to_ascii_uppercase();
                                            s.iter()
                                                .collect::<String>()
                                        })
                                        .collect::<Vec<_>>();

                                    let kek = s
                                        .get(1)
                                        .unwrap()
                                        .to_string();
                                    kek
                                })
                                .collect::<Vec<_>>();

                        let vector_of_all_checkbox_ids_that_are_true_as_joined_string =
                            vector_of_all_checkbox_ids_that_are_true
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                                .join(",");

                        let mut filter_summary_text_to_filter_target_text_ =
                            obtain_filter_summary_text_to_filter_target_text_from_local_storage();

                        filter_summary_text_to_filter_target_text_.insert(
                            text_of_filter_item_keyword,
                            vector_of_all_checkbox_ids_that_are_true_as_joined_string,
                        );

                        let filter_summary_text_to_filter_target_text_as_stringified_json =
                            stringify_filter_summary_text_to_filter_target_text_from_map(
                                &filter_summary_text_to_filter_target_text_,
                            );

                        write_sidebar_items_as_stringified_json_to_local_storage(
                            &filter_summary_text_to_filter_target_text_as_stringified_json,
                        );
                    }
                },
            }
        }

        match obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust()
        {
            | Ok(
                (
                    list_of_titleline_spans_as_rust_,
                    list_of_links_as_rust_,
                    list_of_comments_as_rust_,
                    list_of_hnusers_as_rust_,
                )
            ) =>
            {
                for titleline_span in &list_of_titleline_spans_as_rust_
                {
                    select_submission_elements_and_set_their_style(
                        &titleline_span,
                        "titleline_span",
                        "",
                    );
                }

                for link in &list_of_links_as_rust_
                {
                    select_submission_elements_and_set_their_style(&link, "link", "");
                }

                for comment in &list_of_comments_as_rust_
                {
                    select_submission_elements_and_set_their_style(&comment, "comment", "");
                }

                for username in &list_of_hnusers_as_rust_
                {
                    select_submission_elements_and_set_their_style(&username, "hnuser", "");
                }
            },
            | Err(err) =>
            {
                console::error_1(&format!("An error occurred: {:?}", err).into());
            },
        }

        let window = web_sys::window().expect("should have a Window");

        let closure = Closure::once(move || {
            remove_all_children_of_filter_items_container();
            load_items_from_locastorage_populate_sidebar_and_apply_filters();
        });

        window
            .request_animation_frame(
                closure
                    .as_ref()
                    .unchecked_ref(),
            )
            .expect("should register `requestAnimationFrame` OK");

        closure.forget();
    });

    listener.forget();

    Ok(())
}

use std::collections::HashSet;

#[wasm_bindgen]
pub fn register_filter_out_submissions_event_listener() -> Result<(), JsValue>
{
    let document = obtain_document();

    let filter_button = &document
        .get_element_by_id("filter-button")
        .unwrap();

    let filter_button: Element = filter_button
        .clone()
        .dyn_into()
        .unwrap();

    let result =
        obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust();

    let (
        list_of_titleline_spans_as_rust,
        list_of_links_as_rust,
        list_of_comments_as_rust,
        list_of_hnusers_as_rust,
    ) =
        match result
        {
            | Ok(value) => value,
            | Err(err) => return Err(err),
        };

    let document_ref = document;

    let listener = EventListener::new(&filter_button, "click", move |_event: &Event| {
        let document = document_ref.clone();

        let input = document
            .get_element_by_id("sidebar-filter-input")
            .unwrap();

        let input: Element = input
            .dyn_into()
            .unwrap();

        let input_value = input
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let input_value_as_js = JsValue::from(&input_value);

        let object_str: String = input_value_as_js
            .as_string()
            .unwrap();

        let object_str_as_regex = Regex::new(&object_str);

        let object_str_as_regex = match object_str_as_regex
        {
            | Ok(regex) => regex,
            | Err(_) =>
            {
                Regex::new("invalid_regex").unwrap()
            },
        };

        let sidebar_items_container = &document
            .get_element_by_id("filter-items-container")
            .unwrap();

        let sidebar_items_container: Element = sidebar_items_container
            .clone()
            .dyn_into()
            .unwrap();

        let checkbox_container_div = document
            .get_element_by_id("checkbox-container")
            .unwrap();

        let checkbox_container_div: Element = checkbox_container_div
            .dyn_into()
            .unwrap();

        let checkbox_container_div_children = checkbox_container_div
            .query_selector_all("input[type='checkbox']")
            .unwrap();

        let checkbox_container_div_children_length = checkbox_container_div_children.length();

        let checkbox_container_div_children_as_vec_of_els: Vec<Element> = (0
            ..checkbox_container_div_children_length)
            .map(|index| {
                checkbox_container_div_children
                    .item(index)
                    .unwrap()
            })
            .map(|item| {
                item.dyn_into::<Element>()
                    .unwrap()
            })
            .collect();

        let checkbox_container_div_children_as_vec_of_els_as_string =
            checkbox_container_div_children_as_vec_of_els
                .iter()
                .map(|el| el.outer_html())
                .collect::<Vec<_>>()
                .join("\n");

        let checkbox_container_div_children_as_vec_of_els_length =
            checkbox_container_div_children_as_vec_of_els.len();

        let checkbox_id_to_checkbox_toggle_state = (0
            ..checkbox_container_div_children_as_vec_of_els_length)
            .map(|index| {
                let checkbox_id = checkbox_container_div_children_as_vec_of_els
                    .get(index)
                    .unwrap()
                    .get_attribute("id")
                    .unwrap();

                let checkbox_toggle_state = checkbox_container_div_children_as_vec_of_els
                    .get(index)
                    .unwrap()
                    .clone()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .checked();

                (checkbox_id, checkbox_toggle_state)
            })
            .collect::<HashMap<_, _>>();

        let checkbox_id_to_checkbox_toggle_state_as_stringified_json =
            serde_json::to_string(&checkbox_id_to_checkbox_toggle_state).unwrap();

        let vector_of_all_checkbox_ids_that_are_true = checkbox_id_to_checkbox_toggle_state
            .iter()
            .filter(|(_checkbox_id, checkbox_toggle_state)| **checkbox_toggle_state)
            .map(|(checkbox_id, _checkbox_toggle_state)| checkbox_id)
            .collect::<Vec<&String>>();

        let vector_of_all_checkbox_ids_that_are_true = vector_of_all_checkbox_ids_that_are_true
            .iter()
            .map(|s| {
                let mut s = s.clone();
                let mut s = s
                    .split("-")
                    .collect::<Vec<_>>();

                let mut s = s
                    .iter_mut()
                    .map(|s| {
                        let mut s = s
                            .chars()
                            .collect::<Vec<_>>();
                        s[0] = s[0].to_ascii_uppercase();
                        s.iter()
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>();

                let kek = s
                    .get(0)
                    .unwrap()
                    .to_string();

                kek
            })
            .collect::<Vec<_>>();

        let vector_of_all_checkbox_ids_that_are_true_as_joined_string =
            vector_of_all_checkbox_ids_that_are_true
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",");

        let mut object_str_to_checkbox_id = IndexMap::new();

        object_str_to_checkbox_id.insert(&object_str, vector_of_all_checkbox_ids_that_are_true.clone());

        let filter_summary_texts = object_str_to_checkbox_id
            .keys()
            .collect::<Vec<_>>();

        let filter_target_texts = object_str_to_checkbox_id
            .values()
            .collect::<Vec<_>>();

        let object_str_to_checkbox_id_as_vec_of_tuples = filter_summary_texts
            .iter()
            .zip(filter_target_texts.iter())
            .collect::<Vec<_>>();

        let object_str_to_checkbox_id_as_json = json!(
            {
                "sidebar_items": object_str_to_checkbox_id_as_vec_of_tuples
            }
        );

        let object_str_to_checkbox_id_as_stringified_json =
            object_str_to_checkbox_id_as_json.to_string();

        let some_checkboxes_are_true = checkbox_id_to_checkbox_toggle_state
            .values()
            .any(|checkbox_toggle_state| *checkbox_toggle_state);

        if some_checkboxes_are_true
        {
            let mut filter_summary_text_to_filter_target_text_from_local_storage =
                obtain_filter_summary_text_to_filter_target_text_from_local_storage();

            let object_str_key_is_already_present_in_filter_summary_text_to_filter_target_text_from_local_storage =
                filter_summary_text_to_filter_target_text_from_local_storage
                    .contains_key(&object_str);

            if object_str_key_is_already_present_in_filter_summary_text_to_filter_target_text_from_local_storage
            {
                let string_of_targets =
                    filter_summary_text_to_filter_target_text_from_local_storage
                        .get(&object_str)
                        .unwrap()
                        .clone();

                let vector_of_targets = string_of_targets
                    .split(",")
                    .collect::<Vec<_>>();

                let vector_of_all_checkbox_ids_that_are_true_: Vec<&str> = vector_of_all_checkbox_ids_that_are_true
                    .iter()
                    .map(|s| s.as_str())
                    .collect();

                let concatenated_vec: Vec<&str> = vector_of_targets
                    .iter()
                    .chain(vector_of_all_checkbox_ids_that_are_true_.iter())
                    .cloned()
                    .collect();

                let hashset_of_targets = concatenated_vec
                    .iter()
                    .collect::<HashSet<_>>();

                let joined_string_of_targets = hashset_of_targets
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                filter_summary_text_to_filter_target_text_from_local_storage.insert(
                    object_str,
                    joined_string_of_targets,
                );

            }
            else
            {

                filter_summary_text_to_filter_target_text_from_local_storage.insert(
                    object_str,
                    vector_of_all_checkbox_ids_that_are_true_as_joined_string,
                );
            };

            let filter_summary_text_to_filter_target_text_from_local_storage_as_stringified_json =
                stringify_filter_summary_text_to_filter_target_text_from_map(
                    &filter_summary_text_to_filter_target_text_from_local_storage,
                );

            write_sidebar_items_as_stringified_json_to_local_storage(
                &filter_summary_text_to_filter_target_text_from_local_storage_as_stringified_json,
            );
        }

        remove_all_children_of_filter_items_container();

        let input = document
            .get_element_by_id("sidebar-filter-input")
            .unwrap();

        let input: Element = input
            .dyn_into()
            .unwrap();

        let input_element = input.dyn_into::<HtmlInputElement>().unwrap();

        if some_checkboxes_are_true {
            input_element.set_value("");
        };

        _ = load_items_from_locastorage_populate_sidebar_and_apply_filters();
    });

    listener.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn restore_active_tab_from_previous_session_and_register_filter_tab_switching_event_listener() -> Result<(), JsValue>
{
    let document = obtain_document();

    let list_of_tab_buttons = document
        .query_selector_all(".tab")
        .unwrap();

    let list_of_tab_buttons_as_rust: Vec<HtmlElement> = (0..list_of_tab_buttons.length())
        .map(|index| {
            list_of_tab_buttons
                .item(index)
                .ok_or_else(|| JsValue::from_str("Missing tab at index"))
                .and_then(|item| {
                    item.dyn_into::<HtmlElement>()
                        .map_err(|_| JsValue::from_str("Failed to cast to HtmlElement"))
                })
        })
        .collect::<Result<_, _>>()
        .unwrap();

    let list_of_tab_contents = document
        .query_selector_all(".tab-content")
        .unwrap();

    let list_of_tab_contents_as_rust: Vec<HtmlElement> = (0..list_of_tab_contents.length())
        .map(|index| {
            list_of_tab_contents
                .item(index)
                .ok_or_else(|| JsValue::from_str("Missing tab content at index"))
                .and_then(|item| {
                    item.dyn_into::<HtmlElement>()
                        .map_err(|_| JsValue::from_str("Failed to cast to HtmlElement"))
                })
        })
        .collect::<Result<_, _>>()
        .unwrap();

    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let name_of_active_tab = local_storage
        .get_item("name_of_active_tab")
        .unwrap()
        .unwrap_or_else(
            || {
                "settings-tab".to_owned()
            },
        );

    let list_of_tab_contents_as_rust_clone_1 = list_of_tab_contents_as_rust.clone();

    for tab_content in &list_of_tab_contents_as_rust_clone_1 {

        let tab_content_as_el = tab_content.clone();

        let tab_content_class_name = tab_content_as_el.class_name();

        if tab_content_class_name.contains(&name_of_active_tab) {

            tab_content_as_el.set_attribute("style", "");

        }
        else {
            tab_content_as_el.set_attribute("style", "display: none");

        }
    }

    for tab_button in &list_of_tab_buttons_as_rust {

        let tab_button_as_el = tab_button.clone();

        let current_class_name = &tab_button_as_el.class_name();

        if  (
            current_class_name.contains(&name_of_active_tab)
        )
        {
            let class_name_with_active = format!("{} active", current_class_name);

            let class_name_with_active = class_name_with_active
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            tab_button_as_el.set_class_name(&class_name_with_active);
        }



        let list_of_tab_buttons_as_rust_clone = list_of_tab_buttons_as_rust.clone();
        let list_of_tab_contents_as_rust_clone = list_of_tab_contents_as_rust.clone();

        let listener = EventListener::new(&tab_button, "click", move |event: &Event| {

            for tab_button in &list_of_tab_buttons_as_rust_clone {

                let tab_button_as_el = tab_button.clone();

                let current_class_name = &tab_button_as_el.class_name();

                let class_name_without_active = current_class_name
                    .replace("active", "");

                tab_button_as_el.set_class_name(&class_name_without_active);
            }

            let active_tab_button = event
                .target()
                .unwrap();

            let active_tab_button_as_el = active_tab_button
                .dyn_into::<HtmlElement>()
                .unwrap();

            let current_active_class_name = &active_tab_button_as_el.class_name();

            let class_name_with_active = format!("{} active", current_active_class_name);

            let class_name_with_active = class_name_with_active
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            active_tab_button_as_el.set_class_name(&class_name_with_active);

            let first_class = &class_name_with_active
                .split_whitespace()
                .collect::<Vec<_>>()
                .get(0)
                .unwrap()
                .to_string();

            let local_storage = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap();

            local_storage
                .set_item("name_of_active_tab", &first_class)
                .unwrap();

            let name_of_first_class_of_active_tab_button = &class_name_with_active
                .split_whitespace()
                .collect::<Vec<_>>()
                .get(0)
                .unwrap().to_string();

    let name_of_first_class_of_active_tab_button_as_js = JsValue::from(name_of_first_class_of_active_tab_button.clone());

            for tab_content in &list_of_tab_contents_as_rust_clone {

                let tab_content_as_el = tab_content.clone();

                let tab_content_class_name = tab_content_as_el.class_name();

                if tab_content_class_name.contains(&name_of_first_class_of_active_tab_button.as_str()) {

                    let tab_content_that_matches_tab_button_name = tab_content_as_el.clone();

                    tab_content_as_el.set_attribute("style", "");
                }
                else {
                    tab_content_as_el.set_attribute("style", "display: none");

                }
            }
        });

        listener.forget();
    }

    Ok(())
}

#[wasm_bindgen]
pub fn load_settings_from_local_storage() -> Result<(), JsValue>
{

    let document = obtain_document();

    let substitute_placeholder_checkbox = document
        .get_element_by_id("substitute-placeholder-checkbox")
        .unwrap();

    let substitute_placeholder_checkbox_as_el: Element = substitute_placeholder_checkbox
        .dyn_into()
        .unwrap();

    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let substitute_placeholder_checkbox_state = local_storage
        .get_item("substitute_placeholder")
        .unwrap()
        .unwrap_or_else(
            || {
                "false".to_owned()
            },
        );

    let substitute_placeholder_checkbox_state_as_bool = substitute_placeholder_checkbox_state
        .parse::<bool>()
        .unwrap();

    substitute_placeholder_checkbox_as_el
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_checked(substitute_placeholder_checkbox_state_as_bool);

    let enable_filters_checkbox = document
        .get_element_by_id("enable-filters-checkbox")
        .unwrap();

    let enable_filters_checkbox_as_el: Element = enable_filters_checkbox
        .dyn_into()
        .unwrap();

    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let enable_filters_checkbox_state = local_storage
        .get_item("enable_filters")
        .unwrap()
        .unwrap_or_else(
            || {
                "true".to_owned()
            },
        );

    let enable_filters_checkbox_state_as_bool = enable_filters_checkbox_state
        .parse::<bool>()
        .unwrap();

    enable_filters_checkbox_as_el
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_checked(enable_filters_checkbox_state_as_bool);

    let hide_children_comments_checkbox = document
        .get_element_by_id("hide-children-comments-checkbox")
        .unwrap();

    let hide_children_comments_checkbox_as_el: Element = hide_children_comments_checkbox
        .dyn_into()
        .unwrap();

    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let hide_children_comments_checkbox_state = local_storage
        .get_item("hide_children_comments")
        .unwrap()
        .unwrap_or_else(
            || {
                "true".to_owned()
            },
        );

    let hide_children_comments_checkbox_state_as_bool = hide_children_comments_checkbox_state
        .parse::<bool>()
        .unwrap();

    hide_children_comments_checkbox_as_el
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_checked(hide_children_comments_checkbox_state_as_bool);

    Ok(())
}

#[wasm_bindgen]
pub fn register_settings_checkboxes_event_listener() -> Result<(), JsValue>
{
    let document = obtain_document();

    let substitute_placeholder_checkbox = document
        .get_element_by_id("substitute-placeholder-checkbox")
        .unwrap();

    let substitute_placeholder_checkbox_as_el: Element = substitute_placeholder_checkbox
        .dyn_into()
        .unwrap();

    let substitute_placeholder_checkbox_listener = EventListener::new(&substitute_placeholder_checkbox_as_el, "click", move |event: &Event| {

        let substitute_placeholder_checkbox_as_el = event
            .target()
            .unwrap();

        let substitute_placeholder_checkbox_as_el = substitute_placeholder_checkbox_as_el
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let substitute_placeholder_checkbox_state = substitute_placeholder_checkbox_as_el
            .checked();

        let substitute_placeholder_checkbox_state_as_string = substitute_placeholder_checkbox_state
            .to_string();

        let local_storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        local_storage
            .set_item(
                "substitute_placeholder",
                &substitute_placeholder_checkbox_state_as_string
            )
            .unwrap();

        remove_all_children_of_filter_items_container();

        load_items_from_locastorage_populate_sidebar_and_apply_filters();
    });

    substitute_placeholder_checkbox_listener.forget();

    let enable_filters_checkbox = document
        .get_element_by_id("enable-filters-checkbox")
        .unwrap();

    let enable_filters_checkbox_as_el: Element = enable_filters_checkbox
        .dyn_into()
        .unwrap();

    let enable_filters_checkbox_listener = EventListener::new(&enable_filters_checkbox_as_el, "click", move |event: &Event| {

        let enable_filters_checkbox_as_el = event
            .target()
            .unwrap();

        let enable_filters_checkbox_as_el = enable_filters_checkbox_as_el
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let enable_filters_checkbox_state = enable_filters_checkbox_as_el
            .checked();

        let enable_filters_checkbox_state_as_string = enable_filters_checkbox_state
            .to_string();

        let local_storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        local_storage
            .set_item(
                "enable_filters",
                &enable_filters_checkbox_state_as_string
            )
            .unwrap();

        remove_all_children_of_filter_items_container();

        load_items_from_locastorage_populate_sidebar_and_apply_filters();
    });

    enable_filters_checkbox_listener.forget();

    let hide_children_comments_checkbox = document
        .get_element_by_id("hide-children-comments-checkbox")
        .unwrap();

    let hide_children_comments_checkbox_as_el: Element = hide_children_comments_checkbox
        .dyn_into()
        .unwrap();

    let hide_children_comments_checkbox_listener = EventListener::new(&hide_children_comments_checkbox_as_el, "click", move |event: &Event| {

        let hide_children_comments_checkbox_as_el = event
            .target()
            .unwrap();

        let hide_children_comments_checkbox_as_el = hide_children_comments_checkbox_as_el
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let hide_children_comments_checkbox_state = hide_children_comments_checkbox_as_el
            .checked();

        let hide_children_comments_checkbox_state_as_string = hide_children_comments_checkbox_state
            .to_string();

        let local_storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        local_storage
            .set_item(
                "hide_children_comments",
                &hide_children_comments_checkbox_state_as_string
            )
            .unwrap();

        remove_all_children_of_filter_items_container();

        load_items_from_locastorage_populate_sidebar_and_apply_filters();
    });

    hide_children_comments_checkbox_listener.forget();



    Ok(())

}

#[wasm_bindgen]
pub fn add_donation_buttons_to_sidebar() -> Result<(), JsValue> {
    let document = obtain_document();

    let settings_tab_content = document
        .get_element_by_id("settings-checkbox-container")
        .unwrap();

    let image_bytes_patreon = include_bytes!("../support-on-patreon.png");

    let base64_string_patreon = base64::encode(image_bytes_patreon);

    let data_url_patreon = format!("data:image/png;base64,{}", base64_string_patreon);

    let container_with_images = document
        .create_element("div")
        .unwrap();

    let container_with_images_as_el: Element = container_with_images
        .dyn_into()
        .unwrap();

    container_with_images_as_el.set_inner_html(
        r#"
            Support my work:
            <div style="margin-top: 10px; margin-left: 5px; display: grid; grid-template-columns: auto auto; justify-content: start; gap: 10px;">
                <style>
                    .donate-img { height: 1.5rem; }
                </style>
                <a href="https://liberapay.com/drakerossman/donate">
                    <img id="liberapay" class="donate-img" alt="Donate using Liberapay"/>
                </a>
                <a href="https://patreon.com/DrakeRossman">
                    <img id="patreon" class="donate-img" alt="Donate using Patreon"/>
                </a>
            </div>
        "#
    );

    settings_tab_content
            .append_child(&container_with_images_as_el);

    let img_patreon = document
        .get_element_by_id("patreon")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .expect("should be able to cast as img");

    img_patreon.set_src(&data_url_patreon);

    let image_bytes_liberapay = include_bytes!("../support-on-liberapay.png");

    let base64_string_liberapay = base64::encode(image_bytes_liberapay);

    let data_url_liberapay = format!("data:image/png;base64,{}", base64_string_liberapay);

    let img_liberapay = document
        .get_element_by_id("liberapay")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .expect("should be able to cast as img");

    img_liberapay.set_src(&data_url_liberapay);

    Ok(())
}

fn find_child_with_class(parent: &Element, class_name: &str) -> Result<Option<Element>, JsValue> {
    let selector = format!(".{}", class_name);
    parent.query_selector(&selector).map_err(|e| e.into())
}


#[wasm_bindgen]
pub fn insert_softhide_buttons() -> Result<(), JsValue> {

    let href = window().ok_or("No global `window` object")?.location().href()?;

    if href.contains("item?id") { return Ok(()); }

    let document = obtain_document();

    let list_of_ranking_numbers = document.
        get_elements_by_class_name("rank");

    let length = list_of_ranking_numbers.length();

    for index in 0..length
    {
        if let Some(ranking_number) = list_of_ranking_numbers.item(index)
        {
            let ranking_number_container = ranking_number.parent_element().unwrap();

            let new_td = document.create_element("td")?;

            new_td.set_inner_html(" [–] ");

            new_td.set_attribute("style", "width: 10px; height: 10px; padding-left: 5px;");
            new_td.set_attribute("class", "softhide");

            if let Some(athing_tr) = ranking_number_container.parent_element()
            {
                athing_tr.insert_before(&new_td, ranking_number_container.next_sibling().as_ref())?;

                let listener = EventListener::new(&new_td, "click", move |event: &Event| {

                    let document = obtain_document();

                    let softhidden_entries_container = document
                        .get_element_by_id("softhidden-entries-container")
                        .unwrap();

                    let mut child = softhidden_entries_container.first_child();

                    while let Some(child_element) = child
                    {
                        softhidden_entries_container
                            .remove_child(&child_element)
                            .unwrap();

                        child = softhidden_entries_container.first_child();
                    }

                    let local_storage = web_sys::window()
                        .unwrap()
                        .local_storage()
                        .unwrap()
                        .unwrap();

                    let athing_id_to_submission_title_as_string = local_storage
                        .get_item("softhidden_things")
                        .unwrap()
                        .unwrap_or_else(
                            || {
                                "{}".to_owned()
                            },
                        );

                    let mut athing_id_to_submission_title: serde_json::Value =
                        serde_json::from_str(&athing_id_to_submission_title_as_string).unwrap();


                    let athing_id = athing_tr.id();




                    match find_child_with_class(&athing_tr, "titleline") {
                        Ok(Some(title_td)) => {

                            let a_href_with_title = title_td
                                .first_child()
                                .unwrap();

                            let submission_title = a_href_with_title.text_content();

                            if let Some(athing_id_to_submission_title) =
                                athing_id_to_submission_title.as_object_mut()
                                {
                                    athing_id_to_submission_title.insert(athing_id.to_string(), Value::String(submission_title.unwrap_or_else(|| "".to_string())));
                                }

                            let amended_athing_id_to_submission_title_as_string = to_string(&athing_id_to_submission_title);

                            match amended_athing_id_to_submission_title_as_string {
                                Ok(stringified_json) => {
                                    write_athing_id_to_submission_title_as_stringified_json_to_local_storage(&stringified_json);
                                }
                                Err(e) => { }
                            }

                        }
                        Ok(None) => { }
                        Err(e) => { }
                    }


                    remove_all_children_of_filter_items_container();

                    load_items_from_locastorage_populate_sidebar_and_apply_filters();
                });

                listener.forget();

            }
        }
    }

    let document = obtain_document();

    let list_of_subtexts = document.
        get_elements_by_class_name("subtext");

    let length = list_of_subtexts.length();

    for index in 0..length
    {
        if let Some(subtext) = list_of_subtexts.item(index)
        {
            let empty_td_to_preserve_layout = document.create_element("td")?;

            if let Some(parent) = subtext.parent_element()
            {
                let subtext_node = subtext.dyn_into::<web_sys::Node>()?;
                parent.insert_before(&empty_td_to_preserve_layout, Some(&subtext_node))?;
            }
        }
    }




    Ok(())
}


#[wasm_bindgen]
pub fn register_enter_press_with_focused_input_event_listener() -> Result<(), JsValue> {
    let document = obtain_document();

    let filter_button = document
        .get_element_by_id("filter-button")
        .expect("Should have #filter-button on the page")
        .dyn_into::<web_sys::HtmlButtonElement>()
        .map_err(|_| JsValue::from_str("Could not cast to HtmlButtonElement"))?;

    let input = document
        .get_element_by_id("sidebar-filter-input")
        .expect("Should have #sidebar-filter-input on the page")
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| JsValue::from_str("Could not cast to HtmlInputElement"))?;

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.key() == "Enter" && document.active_element() == Some(input.clone().into()) {
            filter_button.click();
        }
    }) as Box<dyn FnMut(_)>);

    let document = obtain_document();
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>
{
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    replace_center_tag_with_div();

    parse_string_with_sidebar_and_insert_into_body();

    parse_string_with_css_and_insert_into_header();

    load_items_from_locastorage_populate_sidebar_and_apply_filters();

    register_filter_out_submissions_event_listener();

    register_sidebar_item_event_listener();

    restore_active_tab_from_previous_session_and_register_filter_tab_switching_event_listener();

    load_settings_from_local_storage();

    register_settings_checkboxes_event_listener();

    register_enter_press_with_focused_input_event_listener();

    insert_softhide_buttons();

    add_donation_buttons_to_sidebar();

    Ok(())
}
