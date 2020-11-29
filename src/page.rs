use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::{form_urlencoded, Url};

use crate::error::Result;

const DEFAULT_PAGE_SIZE: u32 = 15;

pub struct PageOptions {
    params: HashMap<&'static str, String>,
}

impl PageOptions {
    fn new() -> Self {
        PageOptions {
            params: Default::default()
        }
    }

    pub fn builder() -> PageOptionsBuilder {
        PageOptionsBuilder::new()
    }

    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
}

impl Default for PageOptions {
    fn default() -> Self {
        Self::builder().page_size(DEFAULT_PAGE_SIZE).build()
    }
}

/// a mutable page builder
pub struct PageOptionsBuilder(PageOptions);

impl PageOptionsBuilder {
    fn new() -> Self {
        PageOptionsBuilder(PageOptions::new())
    }

    pub fn page_size(&mut self, n: u32) -> &mut Self {
        self.0
            .params
            .insert("linked_partitioning", "true".to_string());
        self.0.params.insert("page_size", n.to_string());
        self
    }

    pub fn build(&self) -> PageOptions {
        PageOptions {
            params: self.0.params.clone(),
        }
    }
}

/// Paginated response
#[derive(Serialize, Deserialize, Debug)]
pub struct Page<T> {
    /// The collection
    pub collection: Vec<T>,
    /// The url to the next page of results
    pub next_href: Option<String>,
}

impl<T> Page<T> {
    pub fn next_query(&self) -> Result<Option<HashMap<String, String>>> {
        if self.next_href.is_none() {
            return Ok(None);
        }
        let url = Url::parse(self.next_href.as_ref().unwrap())?;
        let next_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

        match next_query.is_empty() {
            true => Ok(None),
            false => Ok(Some(next_query)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
}
