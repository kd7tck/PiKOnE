# How to Run PiKOnE on Windows (Step-by-Step Guide)

This guide is written for anyone, regardless of technical skill. Follow these steps exactly to get the program running on your computer.

## Part 1: Install Rust

Before you can run the program, you need to install a tool called **Rust**.

1.  **Download the Rust Installer:**
    *   Go to this website: [https://rustup.rs/](https://rustup.rs/)
    *   Click the button that says **"DOWNLOAD RUSTUP-INIT.EXE (64-BIT)"**.

2.  **Run the Installer:**
    *   Find the file you just downloaded (`rustup-init.exe`) and double-click it.
    *   **Important:** If it asks you to install "Visual Studio C++ Build Tools", you can **skip that step** if you plan to use our automated script. Just continue with the installation of Rust itself if possible, or press `y` to continue without them if prompted.
        *   *Note: If the installer forces you to install Build Tools, you can do so, but our script will set up a different method that is often easier.*
    *   When the black window asks for options, type `1` and press **Enter** (Proceed with installation).
    *   Wait for it to finish. It will say "Rust is installed now. Great!".
    *   Press **Enter** to close the window.

3.  **Update your Path (Important):**
    *   For the installation to take effect, you usually need to **restart your computer** or log out and log back in. **Please restart your computer now.**

## Part 2: Download the Program

1.  Go to the GitHub page where this project is hosted.
2.  Look for a green button that says **<> Code**. Click it.
3.  Click **Download ZIP**.
4.  Once downloaded, find the ZIP file (usually in your Downloads folder).
5.  Right-click the ZIP file and select **Extract All...**
6.  Choose a location (like your Desktop or Documents) and click **Extract**.
7.  You should now have a folder named `pikone-main` (or similar). Open it.

## Part 3: Run the Program (The Easy Way)

We have created a script that automatically downloads necessary prerequisites (like the compiler toolchain) for you.

1.  Open the folder where you extracted the files.
2.  Find the file named **`start_windows.bat`**.
3.  Double-click it.
4.  A black window will open.
    *   It will automatically download the "GNU Toolchain" (which allows compiling without Visual Studio).
    *   It will then download the program dependencies and build the application.
    *   **Be patient:** The first time you run this, it can take a few minutes.
5.  Once you see a message like "Listening on 0.0.0.0:3000", the program is ready!

## Part 4: Use the Program

1.  **Do not close the black window!** That window is the "engine" keeping the program running.
2.  Open your web browser (Chrome, Edge, Firefox, etc.).
3.  In the address bar at the top, type this address and press Enter:
    ```
    http://localhost:3000
    ```
    *   **Note:** If that doesn't work (especially on Edge), try this address instead:
        ```
        http://127.0.0.1:3000
        ```
4.  You should now see the PiKOnE interface!

## Troubleshooting

*   **"cargo is not recognized..."**: This means the Rust installation didn't finish correctly or you didn't restart your computer after installing Rust. Try restarting.
*   **The window closes immediately**: Try dragging the `start_windows.bat` file into a command prompt window to see the error message.
