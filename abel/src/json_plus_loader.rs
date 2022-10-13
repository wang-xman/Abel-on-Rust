use crate::json_loader::JsonLoader;

pub fn make_json_plus_loader() -> JsonLoader {
    JsonLoader::new_plus()
}

#[cfg(test)]
#[path = "./unittest/json_plus_loader/tests.rs"]
mod tests;