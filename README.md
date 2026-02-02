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

## Releasing

To create a new release on GitHub:

1.  Commit your changes.
2.  Tag a release version (e.g., `v0.1.0`):
    ```bash
    git tag v0.1.0
    git push origin v0.1.0
    ```
3.  The GitHub Action will automatically build the application and upload a `URLRouter.zip` to the Releases page.

