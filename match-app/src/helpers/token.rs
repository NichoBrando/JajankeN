use tonic::metadata::MetadataMap;

pub fn get_token_from_metadata(metadata: &MetadataMap) -> &str {
    match metadata.get("token") {
        Some(token) => token.to_str().unwrap(),
        None => "",
    }
}