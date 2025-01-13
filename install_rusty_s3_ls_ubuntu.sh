#!/bin/bash

# Update package list and install build essentials
sudo apt update
sudo apt install -y build-essential
sudo apt  install unzip

# Install Rust
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

# Verify Rust installation
rustc -V
# installing awscli

#build the project
cargo build --release
sudo mv target/release/rusty_s3_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_s3_ls
echo "rusty_s3_ls has been successfully installed."