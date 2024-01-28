use indexmap::IndexMap;
use serde_json::json;

pub fn stringify_filter_summary_text_to_filter_target_text_from_map(
    filter_summary_text_to_filter_target_text: &IndexMap<String, String>
) -> String
{
    let filter_summary_texts = filter_summary_text_to_filter_target_text
        .keys()
        .collect::<Vec<_>>();

    let filter_target_texts = filter_summary_text_to_filter_target_text
        .values()
        .collect::<Vec<_>>();

    let filter_summary_text_to_filter_target_text_as_vec_of_tuples = filter_summary_texts
        .iter()
        .zip(filter_target_texts.iter())
        .collect::<Vec<_>>();

    let filter_summary_text_to_filter_target_text_as_json = json!(
        {
            "sidebar_items": filter_summary_text_to_filter_target_text_as_vec_of_tuples
        }
    );

    let filter_summary_text_to_filter_target_text_as_stringified_json =
        filter_summary_text_to_filter_target_text_as_json.to_string();

    filter_summary_text_to_filter_target_text_as_stringified_json
}
