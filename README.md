# Cross-Platform URL Router

A Rust application that routes URLs to specific browsers or applications based on regex rules.

## Setup

1. Copy the example configuration:
   ```bash
   cp config.example.json config.json
   ```
2. Edit `config.json` to define your routing rules.

## Build

To build and package the application as `URLRouter.app`:

```bash
./bundle.sh
```

## Usage

Set `URLRouter.app` as your default browser in System Settings.
