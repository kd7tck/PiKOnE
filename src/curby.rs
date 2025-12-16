use anyhow::{Context, Result};
use base64::prelude::*;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CurbyClient {
    client: Client,
    base_url: String,
    chain_id_cache: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChainResponse {
    cid: Cid,
    data: ChainData,
}

#[derive(Debug, Deserialize)]
struct ChainData {
    content: ChainContent,
}

#[derive(Debug, Deserialize)]
struct ChainContent {
    meta: ChainMeta,
}

#[derive(Debug, Deserialize)]
struct ChainMeta {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Cid {
    #[serde(rename = "/")]
    slash: String,
}

#[derive(Debug, Deserialize)]
struct PulseResponse {
    data: PulseData,
}

#[derive(Debug, Deserialize)]
struct PulseData {
    content: PulseContent,
}

#[derive(Debug, Deserialize)]
struct PulseContent {
    payload: PulsePayload,
}

#[derive(Debug, Deserialize)]
struct PulsePayload {
    stage: String,
    round: u64,
    #[serde(default)]
    randomness: Option<RandomnessWrapper>,
}

#[derive(Debug, Deserialize)]
struct RandomnessWrapper {
    #[serde(rename = "/")]
    slash: RandomnessBytes,
}

#[derive(Debug, Deserialize)]
struct RandomnessBytes {
    bytes: String,
}

impl CurbyClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(std::time::Duration::from_secs(10)).build().unwrap(),
            base_url: "https://random.colorado.edu".to_string(),
            chain_id_cache: None,
        }
    }

    /// Retrieves the Chain ID for the "CURBy-Q" quantum source.
    async fn get_quantum_chain_id(&mut self) -> Result<String> {
        if let Some(id) = &self.chain_id_cache {
            return Ok(id.clone());
        }

        let url = format!("{}/api/chains", self.base_url);
        let response_text = self.client.get(&url)
            .send()
            .await?
            .text()
            .await?;

        // Parse list of chains
        let chains: Vec<ChainResponse> = serde_json::from_str(&response_text)
            .context("Failed to parse chains list")?;

        for chain in chains {
            if let Some(name) = &chain.data.content.meta.name {
                if name == "CURBy-Q" {
                    let id = chain.cid.slash;
                    self.chain_id_cache = Some(id.clone());
                    return Ok(id);
                }
            }
        }

        anyhow::bail!("CURBy-Q chain not found");
    }

    /// Fetches a stream of entropy bytes by walking backwards from the latest round.
    pub async fn fetch_entropy_stream(&mut self, bytes_needed: usize) -> Result<Vec<u8>> {
        let chain_id = self.get_quantum_chain_id().await?;

        // Get the latest round
        let latest_url = format!("{}/api/chains/{}/pulses/latest", self.base_url, chain_id);
        let latest_resp: PulseResponse = self.client.get(&latest_url)
            .send()
            .await?
            .json()
            .await?;

        let mut current_round = latest_resp.data.content.payload.round;
        let mut buffer = Vec::new();

        // Loop until we have enough bytes
        while buffer.len() < bytes_needed {
            let round_url = format!("{}/api/chains/{}/pulses/{}", self.base_url, chain_id, current_round);

            // We tolerate some failures (missed pulses)
            if let Ok(resp) = self.client.get(&round_url).send().await {
                if resp.status().is_success() {
                    if let Ok(pulse) = resp.json::<PulseResponse>().await {
                        let payload = pulse.data.content.payload;
                        if payload.stage == "randomness" {
                             if let Some(wrapper) = payload.randomness {
                                 let mut base64_string = wrapper.slash.bytes;
                                 while base64_string.len() % 4 != 0 { base64_string.push('='); }
                                 if let Ok(bytes) = BASE64_STANDARD.decode(&base64_string) {
                                     buffer.extend_from_slice(&bytes);
                                 }
                             }
                        }
                    }
                }
            }

            if current_round == 0 {
                break;
            }
            current_round -= 1;
        }

        Ok(buffer)
    }
}
