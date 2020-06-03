use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::Result;

/// Paginated response
#[derive(Serialize, Deserialize, Debug)]
pub struct Page<T> {
    /// The collection
    pub collection: Vec<T>,
    /// The url to the next page of results
    pub next_href: Option<String>
}

impl <T> Page<T> {
    pub fn next_query(&self) -> Result<Option<HashMap<String, String>>> {
        if self.next_href.is_none() {
            return Ok(None);
        }
        let url = Url::parse(self.next_href.as_ref().unwrap())?;
        let next_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

        match next_query.is_empty() {
            true => Ok(None),
            false => Ok(Some(next_query))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
}
