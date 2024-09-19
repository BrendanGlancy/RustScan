use once_cell::sync::Lazy;
use std::collections::BTreeMap;

fn generated_data() -> BTreeMap<Vec<u16>, Vec<u8>> {
    let mut map = BTreeMap::new();
    map
}

static PARSED_DATA: Lazy<BTreeMap<Vec<u16>, Vec<u8>>> = Lazy::new(generated_data);
pub fn get_parsed_data() -> &'static BTreeMap<Vec<u16>, Vec<u8>> {
    &PARSED_DATA
}
