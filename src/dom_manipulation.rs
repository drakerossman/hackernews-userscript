use wasm_bindgen::prelude::*;
use web_sys::{
    Document,
    Element,
    Event,
    HtmlInputElement,
};
use regex::Regex;
use wasm_bindgen::JsCast;
use gloo::events::EventListener;

use crate::getters::{
    obtain_document,
    obtain_filter_summary_text_to_filter_target_text_from_local_storage,
    obtain_list_of_titleline_spans_as_rust_list_of_links_as_rust_list_of_hnusers_as_rust,
    obtain_athing_id_to_submission_title_from_local_storage,
};

use crate::setters::{
    write_athing_id_to_submission_title_as_stringified_json_to_local_storage,
};

use gloo_console::log as gloo_log;

#[wasm_bindgen]
pub fn replace_center_tag_with_div()
{
    let document = obtain_document();

    let center_element = document
        .get_elements_by_tag_name("center")
        .item(0)
        .expect("no center element found");

    let div_element = document
        .create_element("div")
        .expect("couldn't create div element");

    let inner_html_of_center = center_element.inner_html();

    div_element.set_inner_html(&inner_html_of_center);

    let parent = center_element
        .parent_node()
        .ok_or("kek")
        .unwrap();

    parent
        .remove_child(&center_element)
        .unwrap();
    parent
        .append_child(&div_element)
        .unwrap();
}

#[wasm_bindgen]
pub fn remove_all_children_of_filter_items_container()
{
    let document = obtain_document();

    let filter_items_container = document
        .get_element_by_id("filter-items-container")
        .unwrap();

    let filter_items_container: Element = filter_items_container
        .dyn_into()
        .unwrap();

    let mut child = filter_items_container.first_child();

    while let Some(child_element) = child
    {
        filter_items_container
            .remove_child(&child_element)
            .unwrap();

        child = filter_items_container.first_child();
    }
}

pub fn select_submission_elements_and_set_their_style(
    titleline_span_or_link_or_hnuser_or_comment: &Element,
    switch: &str,
    value_of_style_attribute: &str,
)
{
    let document = obtain_document();

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

    let value_of_style_attribute =
        if !enable_filters_checkbox_state_as_bool { "" }
        else { value_of_style_attribute };


    let submission_placeholder_tr = match switch
    {
        | "titleline_span" =>
        {
            let title_td = titleline_span_or_link_or_hnuser_or_comment
                .parent_node()
                .unwrap();

            let title_td_as_el: Element = title_td
                .dyn_into()
                .unwrap();

            let submission_placeholder_tr = title_td_as_el
                .parent_node()
                .unwrap();

            submission_placeholder_tr
        },
        | "hnuser" =>
        {
            let hn_user = titleline_span_or_link_or_hnuser_or_comment
                .parent_node()
                .unwrap();

                let hn_user_as_el: Element = hn_user
                .dyn_into()
                .unwrap();

            let subline_span = hn_user_as_el
                .parent_node()
                .unwrap();

            let subline_span_as_el: Element = subline_span
                .dyn_into()
                .unwrap();

            let subtext_td = subline_span_as_el
                .parent_node()
                .unwrap();

            let subtext_td_as_el: Element = subtext_td
                .dyn_into()
                .unwrap();

            let submission_placeholder_tr = subtext_td_as_el
                .previous_sibling()
                .unwrap();

            submission_placeholder_tr
        },
        | "link" =>
        {
            let link = titleline_span_or_link_or_hnuser_or_comment
                .parent_node()
                .unwrap();

            let link_as_el: Element = link
                .dyn_into()
                .unwrap();

            let titleline_span = link_as_el
                .parent_node()
                .unwrap();

            let titleline_span_as_el: Element = titleline_span
                .dyn_into()
                .unwrap();
            let title_td = link_as_el
                .parent_node()
                .unwrap();

            let title_td_as_el: Element = title_td
                .dyn_into()
                .unwrap();

            let submission_placeholder_tr = title_td_as_el
                .parent_node()
                .unwrap();

            submission_placeholder_tr
        },
        | "comment" =>
        {
            let commtext = titleline_span_or_link_or_hnuser_or_comment
                .parent_node()
                .unwrap();

            let commtext_as_el: Element = commtext
                .dyn_into()
                .unwrap();

            let comment = commtext_as_el
                .parent_node()
                .unwrap();

            let comment_as_el: Element = comment
                .dyn_into()
                .unwrap();

            let comment_td = comment_as_el
                .parent_node()
                .unwrap();

            let comment_td_as_el: Element = comment_td
                .dyn_into()
                .unwrap();

            let comment_tr = comment_as_el
                .parent_node()
                .unwrap();

            let comment_tr_as_el: Element = comment_tr
                .dyn_into()
                .unwrap();

            let comment_tbody = comment_as_el
                .parent_node()
                .unwrap();

            let comment_tbody_as_el: Element = comment_tbody
                .dyn_into()
                .unwrap();

            let comment_table = comment_as_el
                .parent_node()
                .unwrap();

            let comment_table_as_el: Element = comment_table
                .dyn_into()
                .unwrap();

            let comment_table_td = comment_as_el
                .parent_node()
                .unwrap();

            let comment_table_td_as_el: Element = comment_table_td
                .dyn_into()
                .unwrap();

            let comment_itself = comment_as_el
                .parent_node()
                .unwrap();

            let submission_placeholder_tr = comment_itself;

            submission_placeholder_tr
        },
        | &_ =>
        {
            panic!("switch must be either titleline_span or hnuser");
        },
    };

    let submission_placeholder_tr_as_el: Element = submission_placeholder_tr
        .dyn_into()
        .unwrap();

    submission_placeholder_tr_as_el
        .set_attribute("style", value_of_style_attribute)
        .unwrap();

    let subtext_tr = submission_placeholder_tr_as_el
        .next_element_sibling();

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


    if value_of_style_attribute == "display: none;" && substitute_placeholder_checkbox_state_as_bool
    {
        let document = obtain_document();

        let substitute_placeholder_of_hidden_entry = document
            .create_element("tr")
            .unwrap();

        substitute_placeholder_of_hidden_entry
            .set_attribute("class", "substitute-placeholder-of-hidden-entry")
            .unwrap();

        let inner_html_of_substitute_placeholder_of_hidden_entry = "<td></td><td></td><td><span>[hidden]</span></td>";

        substitute_placeholder_of_hidden_entry
            .set_inner_html(inner_html_of_substitute_placeholder_of_hidden_entry);

        submission_placeholder_tr_as_el.insert_adjacent_element("afterend", &substitute_placeholder_of_hidden_entry).unwrap();

    }

    match subtext_tr
    {
        | Some(subtext_tr) =>
        {
            subtext_tr
                .set_attribute("style", &value_of_style_attribute)
                .unwrap();
            let spacer_tr = subtext_tr
                .next_element_sibling();

            match spacer_tr
            {
                | Some(spacer_tr) =>
                {
                    if !substitute_placeholder_checkbox_state_as_bool
                    {
                        let value_of_style_attribute_for_spacer
                        = if value_of_style_attribute == "display: none;"
                        {
                            value_of_style_attribute
                        }
                        else
                        {
                            "height:5px"
                        };

                        spacer_tr
                            .set_attribute("style", value_of_style_attribute_for_spacer)
                            .unwrap();
                    }

                },
                | None => {},
            }
        },
        | None => {},
    }

    let document = obtain_document();

    let hide_children_comments_checkbox_checkbox = document
        .get_element_by_id("hide-children-comments-checkbox")
        .unwrap();

    let hide_children_comments_checkbox_checkbox_as_el: Element = hide_children_comments_checkbox_checkbox
        .dyn_into()
        .unwrap();

    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let hide_children_comments_checkbox_checkbox_state = local_storage
        .get_item("hide_children_comments")
        .unwrap()
        .unwrap_or_else(
            || {
                "false".to_owned()
            },
        );

    let hide_children_comments_checkbox_checkbox_state_as_bool = hide_children_comments_checkbox_checkbox_state
        .parse::<bool>()
        .unwrap();


    let value_of_style_attribute_for_children_comments
        = if hide_children_comments_checkbox_checkbox_state_as_bool && value_of_style_attribute == "display: none;"
        {
            "display: none;"
        }
        else
        {
            ""
        };

    if (
        true
    ) {
        let comment_tr = &submission_placeholder_tr_as_el
            .parent_node()
            .unwrap();

        let comment_tr_as_el: Element = comment_tr
            .clone()
            .dyn_into()
            .unwrap();

        let comment_tbody = comment_tr_as_el
            .parent_node()
            .unwrap();

        let comment_tbody_as_el: Element = comment_tbody
            .dyn_into()
            .unwrap();

        let comment_table = comment_tbody_as_el
            .parent_node()
            .unwrap();

        let comment_table_as_el: Element = comment_table
            .dyn_into()
            .unwrap();

        let comment_table_td =
            if switch == "hnuser" {
                let more_in_between = comment_table_as_el
                    .parent_node()
                    .unwrap();

                let more_in_between_as_el: Element = more_in_between
                    .dyn_into()
                    .unwrap();

                more_in_between_as_el
                    .parent_node()
                    .unwrap()
            }
            else {
                comment_table_as_el
                    .parent_node()
                    .unwrap()
            };

        let comment_table_td_as_el: Element = comment_table_td
            .dyn_into()
            .unwrap();


        {
            let mut next_sibling_comment_as_el = comment_table_td_as_el
                .next_element_sibling();

            while (
                next_sibling_comment_as_el.clone().is_some()
            ) {

                let next_sibling_comment_as_el_unwrapped = next_sibling_comment_as_el
                    .unwrap();

                let this_comment_as_el: Element = next_sibling_comment_as_el_unwrapped;

                this_comment_as_el
                    .set_attribute("style", &value_of_style_attribute_for_children_comments)
                    .unwrap();

                next_sibling_comment_as_el = this_comment_as_el
                    .next_element_sibling();
            }
        }

    }
}

#[wasm_bindgen]
pub fn remove_elements_with_class(
    class_name: &str
) -> Result<(), JsValue>
{
    let document = obtain_document();

    let selector = format!(".{}", class_name);
    let elements = document.query_selector_all(&selector)?;

    for i in 0..elements.length()
    {
        if let Some(element) = elements.item(i) {
            let element: Element = element.dyn_into().unwrap();
            element.remove();
        }
    }

    Ok(())
}


pub fn hide_specific_item_of_a_specific_category(
    jaywalked_string_with_categories: &String,
    text_as_source_of_truth_against_which_to_check: &String,
    element_whose_text_is_to_be_checked: &Element,
    category: &str,
    mut count_of_filtered_entries: &mut (u32, u32, u32, u32),
)
{
    if jaywalked_string_with_categories.contains(category)
    {
        let text_as_source_of_truth_against_which_to_check_as_regex =
            Regex::new(&text_as_source_of_truth_against_which_to_check);

        let text_as_source_of_truth_against_which_to_check_as_regex =
            match text_as_source_of_truth_against_which_to_check_as_regex
            {
                | Ok(regex) => regex,
                | Err(_) =>
                {
                    Regex::new("invalid_regex").unwrap()
                },
            };

        let text_to_be_checked_if_its_element_should_be_filtered = match category
        {
            | "Title" =>
            {
                let text_to_be_checked_if_its_element_should_be_filtered =
                    element_whose_text_is_to_be_checked
                        .text_content()
                        .unwrap();

                text_to_be_checked_if_its_element_should_be_filtered
            },
            | "Link" =>
            {
                let text_to_be_checked_if_its_element_should_be_filtered =
                    element_whose_text_is_to_be_checked
                        .get_attribute("href")
                        .unwrap();

                text_to_be_checked_if_its_element_should_be_filtered
            },
            | "Username" =>
            {
                let text_to_be_checked_if_its_element_should_be_filtered =
                    element_whose_text_is_to_be_checked
                        .text_content()
                        .unwrap();

                text_to_be_checked_if_its_element_should_be_filtered
            },
            | "Comment" =>
            {
                let text_to_be_checked_if_its_element_should_be_filtered =
                    element_whose_text_is_to_be_checked
                        .text_content()
                        .unwrap();

                text_to_be_checked_if_its_element_should_be_filtered
            },
            | &_ =>
            {
                panic!("Not supported");
            },
        };

        let text_as_source_of_truth_against_which_to_check_as_js_value = JsValue::from_str(&text_as_source_of_truth_against_which_to_check);

        if text_as_source_of_truth_against_which_to_check_as_regex
            .is_match(&text_to_be_checked_if_its_element_should_be_filtered)
        || text_to_be_checked_if_its_element_should_be_filtered.contains(&text_as_source_of_truth_against_which_to_check.to_owned())
        {
            match category
            {
                | "Title" =>
                {
                    select_submission_elements_and_set_their_style(
                        element_whose_text_is_to_be_checked,
                        "titleline_span",
                        "display: none;",
                    );

                    count_of_filtered_entries.0 += 1;
                },
                | "Link" =>
                {
                    select_submission_elements_and_set_their_style(
                        element_whose_text_is_to_be_checked,
                        "link",
                        "display: none;",
                    );

                    count_of_filtered_entries.1 += 1;
                },
                | "Username" =>
                {
                    select_submission_elements_and_set_their_style(
                        element_whose_text_is_to_be_checked,
                        "hnuser",
                        "display: none;",
                    );

                    count_of_filtered_entries.2 += 1;
                },
                | "Comment" =>
                {
                    select_submission_elements_and_set_their_style(
                        element_whose_text_is_to_be_checked,
                        "comment",
                        "display: none;",
                    );

                    count_of_filtered_entries.3 += 1;
                },
                | &_ =>
                {
                    panic!("Not supported");
                },
            }
        }

    }
}

#[wasm_bindgen]
pub fn parse_string_with_sidebar_and_insert_into_body() -> Result<(), JsValue>
{
    let html_string = include_str!("sidebar.html");

    let document = obtain_document();

    let sidebar = document.create_element("div")?;
    sidebar.set_attribute("id", "sidebar")?;
    sidebar.set_class_name("visible");
    sidebar.set_inner_html(html_string);

    let body = document
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap();

    let body: Element = body.dyn_into()?;

    body.set_attribute("style", "display: grid; grid-template-columns: 3fr 1fr;")
        .unwrap();

    body.append_child(&sidebar)?;

    let hnmain = document
        .get_elements_by_tag_name("table")
        .get_with_index(0)
        .unwrap();

    hnmain.set_attribute("width", "100%")?;
    hnmain.set_attribute("margin", "0 auto")?;

    Ok(())
}

#[wasm_bindgen]
pub fn parse_string_with_css_and_insert_into_header() -> Result<(), JsValue>
{
    let html_string = include_str!("style_for_sidebar.css");

    let document = obtain_document();

    let style_element = document
        .create_element("style")
        .expect("couldn't create style element");

    style_element.set_inner_html(html_string);

    let head = document
        .get_elements_by_tag_name("head")
        .get_with_index(0)
        .unwrap();

    head.append_child(&style_element)?;

    Ok(())
}

pub fn load_items_from_locastorage_populate_sidebar_and_apply_filters() -> Result<(), JsValue>
{


    let document = obtain_document();

    remove_elements_with_class("substitute-placeholder-of-hidden-entry").unwrap();

    let filter_summary_text_to_filter_target_text =
        obtain_filter_summary_text_to_filter_target_text_from_local_storage();

    let body = document
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap();

    let filter_summary_texts = filter_summary_text_to_filter_target_text
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    let mut iteration = 0;

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

    let document = obtain_document();

    let athing_id_to_submission_title =
        obtain_athing_id_to_submission_title_from_local_storage();

    let body = document
        .get_elements_by_tag_name("body")
        .get_with_index(0)
        .unwrap();

    let list_of_titlelines_to_softhide = athing_id_to_submission_title
        .values()
        .cloned()
        .collect::<Vec<String>>();

    let list_of_athing_ids_to_softhide = athing_id_to_submission_title
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    let softhidden_entries_container = document
        .get_element_by_id("softhidden-entries-container")
        .unwrap();

    let mut index_of_athing = 0;

    for text_of_sidebar_item in list_of_titlelines_to_softhide
    {
        let mut count_of_filtered_entries: &mut (u32, u32, u32, u32) = &mut (0, 0, 0, 0);

        for titleline_span in &list_of_titleline_spans_as_rust
        {
            hide_specific_item_of_a_specific_category(
                &"Title".to_owned(),
                &text_of_sidebar_item,
                &titleline_span,
                "Title",
                count_of_filtered_entries,
            );
        }

        let document = obtain_document();

        let softhidden_entry_in_sidebar_div = document
            .create_element("div")
            .unwrap();

        let softhidden_entry_in_sidebar_div: Element = softhidden_entry_in_sidebar_div
            .dyn_into()
            .unwrap();

        softhidden_entry_in_sidebar_div
            .set_attribute("class", "softhidden-entry")
            .unwrap();

        softhidden_entry_in_sidebar_div
            .set_attribute("style", "position: relative; clear: both; float: left; box-sizing: border-box; width: 100%; border-radius: 4px; background-color: #f6f6ef; padding: 3px; margin-bottom: 6px; font-size:0.6rem"
        );

        let athing_id = list_of_athing_ids_to_softhide[index_of_athing].clone();

        softhidden_entry_in_sidebar_div.set_inner_html(
            &format!(
                r###"
                    <div class="exclude-softhidden" id="softhidden-{athing_id}" width="16px" height="16px" class="cursor: pointer; position: absolute; right: 10px; top: 12px;" style="color:#D2C6A4;">⨯</div>
                    <div>{text_of_sidebar_item}</div>
                "###
            )
        );

        softhidden_entry_in_sidebar_div.set_attribute("id", &format!("container-{athing_id}"));

        &softhidden_entries_container
            .append_child(&softhidden_entry_in_sidebar_div)
            .unwrap();

        let softhidden_entries_container_clone = softhidden_entries_container.clone();

        let exclude_softhidden_button = &document
            .get_element_by_id(&format!("softhidden-{athing_id}"))
            .unwrap();

        let exclude_softhidden_button: Element = exclude_softhidden_button
            .clone()
            .dyn_into()
            .unwrap();

        let listener = EventListener::new(&exclude_softhidden_button, "click", move |event: &Event| {

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

            if let Some(obj) = athing_id_to_submission_title.as_object_mut() {
                obj.remove(&athing_id.to_string()); // This line modifies `value` in place
            }

            let amended_athing_id_to_submission_title_as_string = serde_json::to_string(&athing_id_to_submission_title);


            match amended_athing_id_to_submission_title_as_string {
                Ok(stringified_json) => {
                    write_athing_id_to_submission_title_as_stringified_json_to_local_storage(&stringified_json);
                }
                Err(e) => { }
            }

            if let Some(submission_to_unhide_tr) =
                &document.get_element_by_id(&athing_id)
            {
                submission_to_unhide_tr.set_attribute("style", "");

                if let Some(subtext_tr) = submission_to_unhide_tr.next_element_sibling()
                {
                    subtext_tr.set_attribute("style", "");


                    if let Some(spacer_tr) = subtext_tr.next_element_sibling()
                    {
                        spacer_tr.set_attribute("style", "");
                    }
                }
            }

            let document_clone = obtain_document();

            let softhidden_entries_container_here = document_clone
                .get_element_by_id("softhidden-entries-container")
                .unwrap();



            let softhidden_entry_in_sidebar_div = document_clone
                .get_element_by_id(&format!("container-{athing_id}"))
                .unwrap();


            softhidden_entries_container_here.remove_child(&softhidden_entry_in_sidebar_div);

        });

        listener.forget();


        index_of_athing += 1;

    }

    for text_of_sidebar_item in filter_summary_texts
    {
        iteration += 1;

        let document = obtain_document();

        let filter_container_div = document
            .create_element("div")
            .unwrap();

        let filter_container_div_as_el: Element = filter_container_div
            .dyn_into()
            .unwrap();

        filter_container_div_as_el
            .set_attribute(
                "class",
                &format!("filter-item-wrapper"),
            )
            .unwrap();

        let filter_summary_div = document
            .create_element("div")
            .unwrap();

        let filter_summary_div: Element = filter_summary_div
            .dyn_into()
            .unwrap();

        filter_summary_div
            .set_attribute("class", "filter-item-keyword-wrapper")
            .unwrap();

        filter_summary_div.set_inner_html(
            &format!(
                r###"
                    <input type="checkbox" id="cb-keywords-{iteration}" class="filter-item-keyword-cb"/>
                    <label class="filter-item-keyword" for="cb-keywords-{iteration}">
                        {text_of_sidebar_item}
                    <svg width="8px" height="8px" viewBox="0 0 8 8" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g><rect id="base" fill-opacity="0" fill="#FFFFFF" x="0" y="0" width="8" height="8"></rect><polygon id="Triangle" fill="#AAAAAA" transform="translate(4.000000, 4.500000) scale(1, -1) translate(-4.000000, -4.500000) " points="4 1.5 8 7.5 0 7.5"></polygon></g></g></svg>
                    </label>
                "###
            )
        );

        filter_container_div_as_el
            .append_child(&filter_summary_div)
            .unwrap();

        let radiobuttons_holder = format!(
            r###"
                    <div width="16px" height="16px" class="ic-remove" style="color:#D2C6A4;">⨯</div>
                    <div class="filter-selector-wrapper fi-title">
                    <input type="checkbox" class="cb-filter-item" id="cb-title-keywords-{iteration}"/>
                    <label class="filter-selector" for="cb-title-keywords-{iteration}">

                    <svg class="filter-selector-cb-active" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -243.000000)"><g id="filter-item-cb-active" transform="translate(587.000000, 243.000000)"><rect id="Rectangle" fill="#FFFFFF" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#FF6600" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>
                    <svg class="filter-selector-cb-inactive" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -207.000000)"><g id="filter-item-cb-inactive" transform="translate(587.000000, 207.000000)"><rect id="Rectangle" fill="#F6F6F0" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#D2C6A4" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>

                    <span>Title</span>
                    </label>
                </div>
                <div class="filter-selector-wrapper fi-link">
                    <input type="checkbox" class="cb-filter-item" id="cb-link-keywords-{iteration}"/>
                    <label class="filter-selector" for="cb-link-keywords-{iteration}">

                    <svg class="filter-selector-cb-active" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -243.000000)"><g id="filter-item-cb-active" transform="translate(587.000000, 243.000000)"><rect id="Rectangle" fill="#FFFFFF" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#FF6600" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>
                    <svg class="filter-selector-cb-inactive" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -207.000000)"><g id="filter-item-cb-inactive" transform="translate(587.000000, 207.000000)"><rect id="Rectangle" fill="#F6F6F0" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#D2C6A4" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>

                    <span>Link</span>
                    </label>
                </div>
                <div class="filter-selector-wrapper fi-username">
                    <input type="checkbox" class="cb-filter-item" id="cb-username-keywords-{iteration}"/>
                    <label class="filter-selector" for="cb-username-keywords-{iteration}">

                    <svg class="filter-selector-cb-active" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -243.000000)"><g id="filter-item-cb-active" transform="translate(587.000000, 243.000000)"><rect id="Rectangle" fill="#FFFFFF" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#FF6600" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>
                    <svg class="filter-selector-cb-inactive" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -207.000000)"><g id="filter-item-cb-inactive" transform="translate(587.000000, 207.000000)"><rect id="Rectangle" fill="#F6F6F0" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#D2C6A4" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>

                    <span>Username</span>
                    </label>
                </div>
                <div class="filter-selector-wrapper fi-comment">
                    <input type="checkbox" class="cb-filter-item" id="cb-comment-keywords-{iteration}"/>
                    <label class="filter-selector" for="cb-comment-keywords-{iteration}">
                    <svg class="filter-selector-cb-active" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -243.000000)"><g id="filter-item-cb-active" transform="translate(587.000000, 243.000000)"><rect id="Rectangle" fill="#FFFFFF" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#FF6600" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>
                    <svg class="filter-selector-cb-inactive" width="16px" height="16px" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"><g id="HD" transform="translate(-587.000000, -207.000000)"><g id="filter-item-cb-inactive" transform="translate(587.000000, 207.000000)"><rect id="Rectangle" fill="#F6F6F0" x="0" y="0" width="16" height="16" rx="4"></rect><polyline id="Path" stroke="#D2C6A4" stroke-width="2" stroke-linecap="round" points="4.11930522 7.64977898 7.08018065 10.5876657 12.1193052 5.58766571"></polyline></g></g></g></svg>                        <span>Comment</span>
                    </label>
                </div>
            "###);

        let inner_html_of_fitler_container_div_as_el = filter_container_div_as_el.inner_html();

        filter_container_div_as_el.set_inner_html(&format!(
            "{inner_html_of_fitler_container_div_as_el} {radiobuttons_holder}"
        ));

        let sidebar_items_container = document
            .get_element_by_id("filter-items-container")
            .unwrap();

        sidebar_items_container
            .append_child(&filter_container_div_as_el)
            .unwrap();

        let filter_target_text = filter_summary_text_to_filter_target_text
            .get(&text_of_sidebar_item)
            .unwrap();

        // ----------------------------
        // ----------------------------
        // ----------------------------

        let mut count_of_filtered_entries: &mut (u32, u32, u32, u32) = &mut (0, 0, 0, 0);

        for titleline_span in &list_of_titleline_spans_as_rust
        {
            hide_specific_item_of_a_specific_category(
                filter_target_text,
                &text_of_sidebar_item,
                &titleline_span,
                "Title",
                count_of_filtered_entries,
            );
        }

        // ----------------------------
        // ----------------------------
        // ----------------------------

        for link in &list_of_links_as_rust
        {
            hide_specific_item_of_a_specific_category(
                filter_target_text,
                &text_of_sidebar_item,
                &link,
                "Link",
                count_of_filtered_entries,
            );
        }

        // ----------------------------
        // ----------------------------
        // ----------------------------

        for username in &list_of_hnusers_as_rust
        {
            hide_specific_item_of_a_specific_category(
                filter_target_text,
                &text_of_sidebar_item,
                &username,
                "Username",
                count_of_filtered_entries,
            );
        }

        // ----------------------------
        // ----------------------------
        // ----------------------------

        for comment in &list_of_comments_as_rust
        {
            hide_specific_item_of_a_specific_category(
                filter_target_text,
                &text_of_sidebar_item,
                &comment,
                "Comment",
                count_of_filtered_entries,
            );
        }

        let (
            count_of_filtered_entries_0,
            count_of_filtered_entries_1,
            count_of_filtered_entries_2,
            count_of_filtered_entries_3,
        ) = count_of_filtered_entries;

        let radiobuttons_holder_2 = format!(
            r#"
                <div class="hidden-count" style="padding-top:3px;">{count_of_filtered_entries_0}/{count_of_filtered_entries_1}/{count_of_filtered_entries_2}/{count_of_filtered_entries_3}</div>
            "#);

        let filter_items_container = document
            .get_element_by_id("filter-items-container")
            .unwrap();

        let filter_container_div_as_el_with_proper_radiobutton_set = filter_items_container
            .last_child()
            .unwrap();

        let filter_container_div_as_el_with_proper_radiobutton_set_as_el: Element = filter_container_div_as_el_with_proper_radiobutton_set
            .dyn_into()
            .unwrap();

        let inner_html_of_fitler_container_div_as_el = filter_container_div_as_el_with_proper_radiobutton_set_as_el.inner_html();

        filter_container_div_as_el_with_proper_radiobutton_set_as_el.set_inner_html(&format!(
            "{inner_html_of_fitler_container_div_as_el} {radiobuttons_holder_2}"
        ));


        if filter_target_text.contains("Title")
        {
            set_specific_radiobutton_for_category_as_checked(
                &filter_container_div_as_el_with_proper_radiobutton_set_as_el,
                "title",
                iteration,
            );
        }
        if filter_target_text.contains("Link")
        {
            set_specific_radiobutton_for_category_as_checked(
                &filter_container_div_as_el_with_proper_radiobutton_set_as_el,
                "link",
                iteration,
            );
        }

        if filter_target_text.contains("Username")
        {
            set_specific_radiobutton_for_category_as_checked(
                &filter_container_div_as_el_with_proper_radiobutton_set_as_el,
                "username",
                iteration,
            );
        }

        if filter_target_text.contains("Comment")
        {
            set_specific_radiobutton_for_category_as_checked(
                &filter_container_div_as_el_with_proper_radiobutton_set_as_el,
                "comment",
                iteration,
            );
        }
    }

    Ok(())
}

pub fn set_specific_radiobutton_for_category_as_checked(
    filter_container_div_as_el: &Element,
    category_str: &str,
    iteration: usize,
)
{
    let radiobutton_as_rust = filter_container_div_as_el
        .query_selector(&format!("#cb-{category_str}-keywords-{iteration}"))
        .unwrap();

    let radiobutton_as_el: Element = radiobutton_as_rust
        .unwrap()
        .dyn_into()
        .unwrap();

    let radiobutton_as_radiobutton: HtmlInputElement = radiobutton_as_el
        .dyn_into()
        .unwrap();

    radiobutton_as_radiobutton.set_checked(true);
}
