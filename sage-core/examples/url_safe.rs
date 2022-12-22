use base64::{engine, Engine as _};
use sage_core::CouncilRank;

fn main() {
    let council_rank = CouncilRank::default();

    let expected = serde_json::to_string(&council_rank).unwrap();

    dbg!(&expected);
    dbg!(&expected.len());

    let hex_string = hex::encode(&expected);

    dbg!(&hex_string);
    dbg!(&hex_string.len());

    let encode_bytes = hex_string.as_bytes();
    let encoded_url = engine::URL_SAFE.encode(encode_bytes);

    dbg!(&encoded_url);
    dbg!(&encoded_url.len());

    let decoded_bytes = engine::URL_SAFE.decode(&encoded_url).unwrap();
    assert_eq!(encode_bytes, &decoded_bytes);

    let decoded_hex_string = String::from_utf8(decoded_bytes).unwrap();

    assert_eq!(&hex_string, &decoded_hex_string);
    dbg!(&decoded_hex_string);

    let hex_decoded = hex::decode(&decoded_hex_string).unwrap();
    let actual = String::from_utf8(hex_decoded).unwrap();

    assert_eq!(&expected, &actual);
    dbg!(&actual);

    let other_council_rank = serde_json::from_str::<CouncilRank>(&actual).unwrap();
    dbg!(other_council_rank);
}
