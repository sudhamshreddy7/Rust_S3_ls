#!/bin/bash

# Update package list and install development tools
sudo dnf update -y
sudo dnf groupinstall "Development Tools" -y

# Install Rust
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

# Verify Rust installation
rustc -V

## Install AWS CLI
#sudo dnf install awscli -y

# Build the project
cargo build --release

# Move the compiled binary to /usr/local/bin and set executable permissions
sudo mv target/release/rusty_s3_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_s3_ls

# Done
echo "rusty_s3_ls has been successfully installed."
