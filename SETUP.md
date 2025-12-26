# AWS Connector - Setup Guide

## Required Software

| Software | Version | Installation |
|----------|---------|--------------|
| Node.js | v22.x or later | https://nodejs.org/ |
| pnpm | v10.x or later | `npm install -g pnpm` |
| Rust | 1.92.0 or later | https://rustup.rs/ |
| Cargo | 1.92.0 or later | Included with Rust |
| Tauri CLI | 2.9.x | Installed via pnpm |

## Tested Versions

```
Node.js:    v22.16.0
npm:        11.4.2
pnpm:       10.26.2
Rust:       1.92.0
Cargo:      1.92.0
Tauri CLI:  2.9.6
```

## Platform Requirements

### macOS
- macOS 10.15 (Catalina) or later
- Xcode Command Line Tools: `xcode-select --install`

### Windows
- Windows 10/11
- Microsoft Visual Studio C++ Build Tools
- WebView2 (usually pre-installed on Windows 10/11)

### Linux
- webkit2gtk-4.1
- libayatana-appindicator3-dev (Ubuntu/Debian)

## AWS CLI (Optional but Recommended)

For SSO login support:
- AWS CLI v2: https://aws.amazon.com/cli/
- Session Manager Plugin: https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html

## Installation Steps

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd aws_connector
   ```

2. **Install Node dependencies**
   ```bash
   pnpm install
   ```

3. **Run in development mode**
   ```bash
   pnpm tauri dev
   ```

4. **Build for production**
   ```bash
   pnpm tauri build
   ```

   The DMG/installer will be in `src-tauri/target/release/bundle/`

## Troubleshooting

### Disk Space
The Rust target folder can grow to 10-15GB during development. To clean:
```bash
cd src-tauri && cargo clean
```

### Port Already in Use
If port 1420 is in use:
```bash
lsof -ti:1420 | xargs kill -9
```

### AWS Credentials
Ensure you have valid AWS credentials in `~/.aws/credentials` or configure SSO profiles in `~/.aws/config`.
