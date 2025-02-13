pub mod driver {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Driver {
        pub name: String,
        pub version: String,
        pub download_url: String,
    }
}