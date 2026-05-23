use std::{collections::HashSet, sync::OnceLock};

const DEFAULT_DICT_STRING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/default-dict.txt"
));
static DEFAULT_DICT: OnceLock<HashSet<String>> = OnceLock::new();

// Loaded only when called to avoid redundant load time
pub fn load_default_dict() -> HashSet<String> {
    return DEFAULT_DICT
        .get_or_init(|| {
            let mut dict: HashSet<String> = HashSet::new();
            for word in DEFAULT_DICT_STRING.trim().split("\n") {
                dict.insert(String::from(word.trim()));
            }
            return dict;
        })
        .clone();
}
