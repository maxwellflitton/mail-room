

pub mod json_parser {

    use std::collections::HashMap;
    use bytes::Bytes;
    use serde_json;

    /// Data that describes the data passed into the server.
    pub struct RequestBody {
        /// data passed into the server converted from bytes
        pub content: HashMap<String, serde_json::Value>
    }

    impl RequestBody {

        /// Creates a new RequestBody struct from bytes.
        ///
        /// # Arguments
        ///
        /// * `byte_stream` - bytes passed into the server
        ///
        /// # Returns
        /// Data packed in a HashMap
        pub fn new(byte_stream: Bytes) -> RequestBody {
            let params: HashMap<String, serde_json::Value> = serde_json::from_slice(&byte_stream).unwrap();
            RequestBody {content: params}
        }
    }
}


#[cfg(test)]
mod json_parser_tests {
    use super::json_parser;

    
}
