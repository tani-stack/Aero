# Contributing to Vortex OS

We welcome contributions to the Vortex OS project! Your help is invaluable in making Vortex a robust and feature-rich operating system for autonomous systems. This guide outlines the process for contributing to the project.

## Code of Conduct

By participating in this project, you are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to [your-email@example.com].

## How to Contribute

There are many ways to contribute, from reporting bugs to writing new features:

1.  **Report Bugs:** If you find a bug, please open an issue on GitHub with a clear description, steps to reproduce, and expected behavior.
2.  **Suggest Enhancements:** Have an idea for a new feature or improvement? Open an issue to discuss it.
3.  **Write Code:** This is where most contributions happen. We particularly welcome help with:
    *   Implementing new hardware drivers (sensors, actuators, communication modules).
    *   Improving existing driver implementations based on datasheets.
    *   Enhancing kernel components (scheduler, IPC, memory management).
    *   Optimizing the async runtime executor.
    *   Porting Vortex OS to new hardware platforms (e.g., STM32, ESP32, RISC-V).
4.  **Improve Documentation:** Help us keep the `README.md`, `CONTRIBUTING.md`, and other documentation up-to-date and clear.

## Getting Started

### 1. Fork the Repository

First, fork the [Vortex OS repository](https://github.com/tani-stack/Vortex.git) to your GitHub account.

### 2. Clone Your Fork

Clone your forked repository to your local machine:

```bash
git clone https://github.com/your-username/Vortex.git
cd Vortex
```

### 3. Install Prerequisites

Vortex OS is written in Rust. You'll need `rustup` to manage your Rust toolchains:

```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the nightly toolchain and target for AArch64 (QEMU simulation)
rustup toolchain install nightly
rustup target add aarch64-unknown-none
```

### 4. Build and Run (QEMU)

To build and run the kernel in QEMU (for development and testing):

```bash
# Build in release mode
cargo build --release

# Run on QEMU (ARM64 virt machine)
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -m 256M \
  -kernel target/aarch64-unknown-none/release/vortex-kernel \
  -nographic
```

## Code Style and Quality

We strive for high code quality and consistency. Please adhere to the following guidelines:

*   **Rustfmt:** Always format your code using `rustfmt`.
    ```bash
cargo fmt --all
    ```
*   **Clippy:** Run `clippy` to catch common mistakes and improve your code.
    ```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
    ```
*   **Memory Safety:** Prioritize memory safety. Avoid `unsafe` blocks where possible, and if necessary, clearly document their purpose and ensure their correctness.
*   **Tests:** Write unit and integration tests for new features and bug fixes.
    ```bash
cargo test --workspace
    ```

## Submitting Changes (Pull Requests)

1.  **Create a New Branch:** Create a new branch for your feature or bug fix.
    ```bash
git checkout -b feature/my-new-driver
    ```
2.  **Make Your Changes:** Implement your changes, ensuring they follow the code style and pass all tests.
3.  **Commit Your Changes:** Write clear and concise commit messages.
    ```bash
git commit -m "feat: Add support for new XYZ sensor driver"
    ```
4.  **Push to Your Fork:** Push your branch to your forked repository.
    ```bash
git push origin feature/my-new-driver
    ```
5.  **Open a Pull Request (PR):** Go to the original [Vortex OS repository](https://github.com/tani-stack/Vortex.git) on GitHub and open a new pull request from your branch. Provide a detailed description of your changes.

## Seeking Help

If you have any questions or need help, feel free to open an issue or reach out to the maintainers.

Thank you for contributing to Vortex OS!

