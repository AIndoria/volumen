use std::path::Path;

use anyhow::Result;
use rand::prelude::*;

// List of English adjective words generated based on the data/adjectives.txt file
pub const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

// List of English noun words generated based on the data/nouns.txt file
pub const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

pub fn dir_is_writable(path: &Path) -> Result<bool> {
    let meta = path.metadata()?;
    Ok(!meta.permissions().readonly())
}

pub fn gen_name() -> String {
    let mut rng = thread_rng();

    let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
    let adj2 = ADJECTIVES.choose(&mut rng).unwrap();
    let noun = NOUNS.choose(&mut rng).unwrap();

    format!("{}{}{}", adj1, adj2, noun)
}

#[cfg(feature = "web")]
pub fn gen_base_url(domain: &str, https: bool) -> String {
    let mut url = if https {
        String::from("https://")
    } else {
        String::from("http://")
    };

    url.push_str(domain);
    if !domain.ends_with('/') {
        url.push('/');
    }

    url
}
