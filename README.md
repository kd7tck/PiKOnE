# PiKOnE

PiKOnE is a randomizer engine that utilizes the University of Colorado Randomness Beacon (CURBy) to seed high-performance simulations. It searches for statistical anomalies in quantum noise to drive divination and decision-making results.

## Technical Overview

This program utilizes a Rust Axum server to display a modern, responsive GUI for user interaction, featuring a colorful vector-style design with 2D animations.

**Strict Randomness Requirement:** This application strictly uses **only** quantum randomness sourced directly from the CURBy-Q beacon. No pseudo-random number generators (PRNGs) are used for decision making.

## How It Works

1.  **Input**: The user enters two choices, Choice A and Choice B.
2.  **Configuration**: The user specifies the number of sessions ($N$) they wish to perform.
3.  **The Process**:
    *   The user sits down for $N$ sessions.
    *   In each session, two random numbers are displayed on cards.
    *   Each number is linked to either Choice A or Choice B.
    *   The association between the numbers and the choices randomly alternates each round, determined solely by quantum entropy fetched from CURBy.
    *   The user randomly picks one of the displayed numbers without knowing which choice it corresponds to.
4.  **Data Collection**: In the background, the server tracks how many times Choice A or Choice B is selected.
5.  **Analysis**: At the end of the sessions, an algorithm calculates a Z-score to determine if there is a statistical anomaly in the results.

## Features

*   **Quantum Randomness**: Strictly uses the CURBy-Q randomness beacon.
*   **Vector GUI**: A beautiful, 2D vector art style interface with smooth animations using CSS and SVG.
*   **Statistical Analysis**: Automatic calculation of Z-scores to detect anomalies.
*   **PDF Reports**: Generates downloadable PDF reports of the simulation results.
*   **Cross-Platform**: Built with Rust to be easily compilable on any machine.

## Building and Running

### Prerequisites
- Rust (latest stable)
- Internet connection (to access `random.colorado.edu`)

### Steps
1.  Clone the repository.
2.  Run `cargo run`.
3.  Open your browser to `http://localhost:3000`.

### Windows Users
For a very detailed, step-by-step guide on how to install and run this on Windows, please read [WINDOWS_GUIDE.md](WINDOWS_GUIDE.md).

## Development

The project structure is as follows:
- `src/`: Contains the Rust backend code.
    - `main.rs`: Entry point and Axum server setup.
    - `curby.rs`: Client for fetching quantum entropy from CURBy.
    - `simulation.rs`: Core logic for generating sessions using entropy.
    - `stats.rs`: Statistical analysis functions (Z-score).
    - `report.rs`: PDF generation logic.
- `static/`: Contains the frontend assets.
    - `index.html`: The single-page application entry point with embedded CSS and JS.
- `Cargo.toml`: Rust dependencies and configuration.

### Testing
Run `cargo test` to execute the unit tests for statistical functions.

## License

GNU 3.0
