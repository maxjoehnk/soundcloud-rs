use crate::error::Result;
use crate::PageOptions;
use futures::stream::BoxStream;
use serde::de::DeserializeOwned;

pub trait StreamingApiExt: StreamingApi {
    /// Return a stream of all [`StreamingApi::Model`].
    fn iter(&self, options: PageOptions) -> BoxStream<Result<Self::Model>> {
        self.fetch(&options, None)
    }

    /// Return a stream of [`StreamingApi::Model`] limited to the first num_pages pages
    fn get(&self, options: PageOptions, num_pages: u64) -> BoxStream<Result<Self::Model>> {
        self.fetch(&options, Some(num_pages))
    }
}

impl<T: ?Sized> StreamingApiExt for T where T: StreamingApi {}

pub trait StreamingApi {
    type Model: DeserializeOwned;

    fn path(&self) -> String;

    fn get_stream(&self, url: &str, pages: Option<u64>) -> BoxStream<Result<Self::Model>>;

    fn fetch(
        &self,
        options: &PageOptions,
        num_pages: Option<u64>,
    ) -> BoxStream<Result<Self::Model>> {
        let url = self.path();
        let url = if let Some(params) = options.serialize() {
            format!("{}?{}", url, params)
        } else {
            url
        };
        self.get_stream(&url, num_pages)
    }
}
