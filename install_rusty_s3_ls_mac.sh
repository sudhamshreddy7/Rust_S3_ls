#!/bin/bash

# Update Homebrew and install build tools
brew update
brew install rust

# Verify Rust installation
rustc -V

## Install AWS CLI
#brew install awscli

# Build the project
cargo build --release

# Move the compiled binary to /usr/local/bin and set executable permissions
sudo mv target/release/rusty_s3_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_s3_ls

# Done
echo "rusty_s3_ls has been successfully installed."
