# How to Run PiKOnE on Windows (Step-by-Step Guide)

This guide is written for anyone, regardless of technical skill. Follow these steps exactly to get the program running on your computer.

## Part 1: Install the Necessary Tools

Before you can run the program, you need to install a tool called **Rust**. Rust is the language this program is written in.

1.  **Download the Rust Installer:**
    *   Go to this website: [https://rustup.rs/](https://rustup.rs/)
    *   Click the button that says **"DOWNLOAD RUSTUP-INIT.EXE (64-BIT)"**.

2.  **Run the Installer:**
    *   Find the file you just downloaded (`rustup-init.exe`) and double-click it.
    *   A black window will appear. It might ask you to install "Visual Studio C++ Build Tools".
        *   **If it asks for Build Tools:** It will provide a link. Copy that link into your browser, download the "Visual Studio Build Tools", and install the workload named **"Desktop development with C++"**. This is required for Rust to work on Windows. Once that is done, run `rustup-init.exe` again.
    *   When the black window asks for options, you will see `1) Proceed with installation (default)`.
    *   Type `1` and press **Enter**.
    *   Wait for it to finish. It will say "Rust is installed now. Great!".
    *   Press **Enter** to close the window.

3.  **Update your Path (Important):**
    *   For the installation to take effect, you usually need to **restart your computer** or at least log out and log back in. **Please restart your computer now just to be safe.**

## Part 2: Download the Program

1.  Go to the GitHub page where this project is hosted.
2.  Look for a green button that says **<> Code**. Click it.
3.  Click **Download ZIP**.
4.  Once downloaded, find the ZIP file (usually in your Downloads folder).
5.  Right-click the ZIP file and select **Extract All...**
6.  Choose a location (like your Desktop or Documents) and click **Extract**.
7.  You should now have a folder named `pikone-main` (or similar). Open it. You should see files like `README.md`, `Cargo.toml`, and a `src` folder.

## Part 3: Run the Program

1.  **Open a Command Window:**
    *   Make sure you are inside the folder you just extracted (where you see `Cargo.toml`).
    *   **Windows 11:** Right-click anywhere in the empty white space of the folder window and select **Open in Terminal**.
    *   **Windows 10:** Hold down the **Shift** key on your keyboard, then Right-click in the empty white space. Select **Open PowerShell window here**.

2.  **Type the Command:**
    *   In the blue or black window that popped up, type exactly this:
        ```
        cargo run
        ```
    *   Press **Enter**.

3.  **Wait:**
    *   Since this is your first time, the computer has to download and build all the pieces of the program. You will see a lot of green text and progress bars downloading "crates". **This is normal.**
    *   It might take a few minutes.
    *   Eventually, the text will stop scrolling, and you might see a message saying something like "Listening on 0.0.0.0:3000". This means it is ready!

## Part 4: Use the Program

1.  **Do not close the black/blue window!** That window is the "engine" keeping the program running.
2.  Open your web browser (Chrome, Edge, Firefox, etc.).
3.  In the address bar at the top, type this address and press Enter:
    ```
    http://localhost:3000
    ```
4.  You should now see the PiKOnE interface!

## Troubleshooting

*   **"cargo is not recognized..."**: This means the Rust installation didn't finish correctly or you didn't restart your computer. Try restarting.
*   **"linker 'link.exe' not found"**: This means the "Visual Studio C++ Build Tools" were not installed correctly. You need to install the "Desktop development with C++" workload from the Visual Studio installer.
