use crate::curby::CurbyClient;
use serde::Serialize;
use anyhow::{Result, bail};

#[derive(Clone, Debug, Serialize)]
pub struct SessionStep {
    pub left_number: u8,
    pub right_number: u8,
    // Determines if Choice A is on the Left (true) or Right (false).
    pub is_choice_a_left: bool,
}

#[derive(Debug)]
pub struct SimulationEngine {
    client: CurbyClient,
}

impl SimulationEngine {
    /// Initializes the simulation engine with a CURBy client.
    pub fn new() -> Self {
        Self {
            client: CurbyClient::new(),
        }
    }

    /// Generates the session steps using ONLY quantum entropy.
    ///
    /// This function fetches raw entropy from the CURBy beacon and uses it to determine:
    /// 1. The random placement of Choice A (Left or Right).
    /// 2. The random number displayed on the Left card.
    /// 3. The random number displayed on the Right card.
    ///
    /// No PRNG is used; the bytes are taken directly from the quantum stream.
    ///
    /// Each session requires:
    /// 1. 1 bit to decide if Choice A is Left or Right.
    /// 2. 8 bits for Left Number.
    /// 3. 8 bits for Right Number.
    /// Total: ~17 bits (round up to 3 bytes per session for simplicity).
    pub async fn generate_sessions(&mut self, n: usize) -> Result<Vec<SessionStep>> {
        let bytes_needed = n * 3;

        // Fetch enough entropy for all sessions at once
        let entropy_pool = self.client.fetch_entropy_stream(bytes_needed).await?;

        if entropy_pool.len() < bytes_needed {
            bail!("Could not fetch enough quantum entropy.");
        }

        let mut sessions = Vec::with_capacity(n);
        let mut idx = 0;

        for _ in 0..n {
            // Byte 1: Decision (Left/Right)
            let decision_byte = entropy_pool[idx];
            idx += 1;

            // Byte 2: Left Number
            let left_num = entropy_pool[idx];
            idx += 1;

            // Byte 3: Right Number
            let right_num = entropy_pool[idx];
            idx += 1;

            // Use LSB of decision_byte for placement (Odd = True/Left, Even = False/Right)
            let is_choice_a_left = (decision_byte & 1) == 1;

            sessions.push(SessionStep {
                left_number: left_num,
                right_number: right_num,
                is_choice_a_left,
            });
        }

        Ok(sessions)
    }
}
