

pub mod json_parser {

    use std::collections::HashMap;
    use bytes::Bytes;
    use serde_json;

    pub struct RequestBody {
        pub content: HashMap<String, serde_json::Value>,
        pub complete: bool
    }

    impl RequestBody {

        pub fn new(byte_stream: Bytes) -> RequestBody {
            let mut is_complete = true;
            let params: HashMap<String, serde_json::Value> = serde_json::from_slice(&byte_stream).unwrap();

            if params.get("message") == None {
                is_complete = false;
            }
            if params.get("type") == None {
                is_complete = false;
            }

            RequestBody {content: params, complete: is_complete}

        }
    }
}


#[cfg(test)]
mod json_parser_tests {
    use super::json_parser;

    
}
