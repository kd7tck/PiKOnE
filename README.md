# PiKOnE

PiKOnE is a randomizer engine that utilizes the University of Colorado Randomness Beacon (CURBy) to seed high-performance simulations. It searches for statistical anomalies in quantum noise to drive divination and decision-making results.

## Technical Overview

This program utilizes a Rust Axum server to display a GUI for user interaction.

## How It Works

1.  **Input**: The user enters two choices, Choice A and Choice B.
2.  **Configuration**: The user specifies the number of sessions ($N$) they wish to perform.
3.  **The Process**:
    *   The user sits down for $N$ sessions.
    *   In each session, two random numbers are displayed.
    *   Each number is linked to either Choice A or Choice B.
    *   The association between the numbers and the choices randomly alternates each round.
    *   The user randomly picks one of the displayed numbers without knowing which choice it corresponds to.
4.  **Data Collection**: In the background, the server tracks how many times Choice A or Choice B is selected.
5.  **Analysis**: At the end of the sessions, an algorithm calculates a Z-score to determine if there is a statistical anomaly in the results.

## Output

The results are generated into a PDF file, which can be saved to a location of the user's choice.

## Goal

The goal of PiKOnE is to determine if a person, when presented with random numbers associated with either of two choices, will produce an outcome that deviates from the normal distribution (i.e., a statistical anomaly).
