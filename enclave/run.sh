#!/bin/bash

set -e

# Assign local loopback address
ifconfig lo 127.0.0.1

# Redirect API endpoint to lo
echo "127.0.0.1    nitro-enclaves-demo.richardfan.xyz" >> /etc/hosts

# Start traffic forwarder
nohup /usr/bin/socat TCP-LISTEN:443,fork,reuseaddr VSOCK-CONNECT:3:8000 &

# Start app
/app/nitro-enclaves-rust-demo-enclave
