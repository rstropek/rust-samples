use spin_sdk::{
    http::Request,
};

pub struct Parameters {
    pub seed: String,
    pub depth: usize,
    pub trunk_length: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            seed: "Hello WASM".to_string(),
            depth: 13usize,
            trunk_length: 100f64,
        }
    }
}

pub fn extract_parameters(req: &Request) -> Parameters {
    let query = req.uri().query().unwrap_or("");
    let mut params = Parameters::default();

    for pair in query.split('&').filter(|s| !s.is_empty()) {
        let mut parts = pair.split('=');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        match key {
            "seed" => params.seed = value.to_string(),
            "depth" => params.depth = value.parse().unwrap_or(params.depth),
            "trunk" => params.trunk_length = value.parse().unwrap_or(params.trunk_length),
            _ => {},
        }
    }

    params
}