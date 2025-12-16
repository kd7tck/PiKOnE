# PiKOnE

PiKOnE is a randomizer engine that utilizes the University of Colorado Randomness Beacon (CURBy) to seed high-performance simulations. It searches for statistical anomalies in quantum noise to drive divination and decision-making results.

## Technical Overview

This program utilizes a Rust Axum server to display a GUI for user interaction.

**Strict Randomness Requirement:** This application strictly uses **only** quantum randomness sourced directly from the CURBy-Q beacon. No pseudo-random number generators (PRNGs) are used for decision making.

## How It Works

1.  **Input**: The user enters two choices, Choice A and Choice B.
2.  **Configuration**: The user specifies the number of sessions ($N$) they wish to perform.
3.  **The Process**:
    *   The user sits down for $N$ sessions.
    *   In each session, two random numbers are displayed.
    *   Each number is linked to either Choice A or Choice B.
    *   The association between the numbers and the choices randomly alternates each round, determined solely by quantum entropy.
    *   The user randomly picks one of the displayed numbers without knowing which choice it corresponds to.
4.  **Data Collection**: In the background, the server tracks how many times Choice A or Choice B is selected.
5.  **Analysis**: At the end of the sessions, an algorithm calculates a Z-score to determine if there is a statistical anomaly in the results.

## Output

The results are generated into a PDF file, which can be saved to a location of the user's choice.

## Building and Running

### Prerequisites
- Rust (latest stable)
- Internet connection (to access `random.colorado.edu`)

### Steps
1.  Clone the repository.
2.  Run `cargo run`.
3.  Open your browser to `http://localhost:3000`.

## License

MIT
