use std::time::SystemTime;
// use serde::{Serializer, Deserialize, };

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_timestamp() {
        let ts = timestamp();

        assert_eq!(format!("{ts}").chars().count(), 10);
    }
}
