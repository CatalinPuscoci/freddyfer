use std::{collections::HashMap, path::Path, str::FromStr};

use itertools::Itertools;
use serenity::prelude::TypeMapKey;

pub struct SoundsPages {
    _pages: HashMap<usize, Vec<String>>,
}

impl SoundsPages {
    pub fn get_page(&self, page_index: usize) -> Option<&Vec<String>> {
        self._pages.get(&page_index)
    }
}

pub struct AppCache {
    pub sounds_pages: SoundsPages,
}

pub struct AppCacheKey {}

impl TypeMapKey for AppCacheKey {
    type Value = AppCache;
}

pub fn create_app_cache() -> AppCache {
    AppCache {
        sounds_pages: create_sounds_pages_cache(),
    }
}

fn create_sounds_pages_cache() -> SoundsPages {
    let dir = match Path::new("sounds").read_dir() {
        Ok(dir) => dir,
        Err(why) => {
            println!("Could not load sounds folder. Reason: {:?}", why);
            return SoundsPages {
                _pages: HashMap::new(),
            };
        }
    };

    let pages: HashMap<usize, Vec<String>> = dir
        .filter(|e| e.is_ok())
        .map(|e| String::from_str(e.ok().unwrap().file_name().to_str().unwrap()).unwrap())
        .collect_vec()
        .chunks(25)
        .into_iter()
        .enumerate()
        .map(|c| (c.0, c.1.to_vec()))
        .collect();

    SoundsPages { _pages: pages }
}
