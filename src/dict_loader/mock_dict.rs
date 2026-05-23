use std::collections::HashSet;

pub fn load_mock_dict(word_len: usize) -> HashSet<String> {
    let mut dict: HashSet<String> = HashSet::new();
    for c in 'a'..='z' {
        dict.insert(c.to_string().repeat(word_len));
    }

    return dict;
}
