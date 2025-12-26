# AWS Connector - Development Context

## Project Overview
A cross-platform desktop app for connecting to AWS ECS containers and EC2 instances via SSM Session Manager.

**Tech Stack:** Tauri 2.0 (Rust) + Svelte 5 (TypeScript) + xterm.js

## Project Structure

```
aws_connector/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs               # Entry point, registers Tauri commands
│   │   ├── aws/
│   │   │   ├── mod.rs
│   │   │   ├── credentials.rs   # AWS profile parsing/management
│   │   │   ├── ecs.rs           # ECS discovery
│   │   │   └── ec2.rs           # EC2 discovery
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── aws_commands.rs  # Profile management commands
│   │       └── resource_commands.rs # Resource discovery commands
├── src/                          # Svelte frontend
│   ├── lib/
│   │   ├── api/                 # Tauri command bindings
│   │   │   └── aws.ts
│   │   ├── stores/              # Svelte stores
│   │   │   ├── profiles.ts
│   │   │   └── resources.ts
│   │   ├── types/
│   │   │   └── aws.ts
│   │   └── components/
│   │       ├── layout/          # Sidebar, Header, MainContent, StatusBar
│   │       └── aws/             # ResourceTree, AddProfileModal
│   └── routes/
│       └── +page.svelte
```

## Completed Phases

### Phase 1: Project Setup
- Tauri 2.0 + Svelte 5 + TypeScript
- Tailwind CSS with dark theme CSS variables
- Basic UI shell components

### Phase 2: AWS Profile Management
- Parse ~/.aws/credentials and ~/.aws/config
- Profile selector in header
- Add profile functionality (Access Keys + SSO)
- Credential validation via STS
- SSO login support via AWS CLI

### Phase 3: AWS Resource Discovery
- ECS discovery: clusters -> services -> tasks -> containers
- EC2 discovery: SSM-enabled instances
- ResourceTree component with collapsible nodes
- Connect buttons for exec-enabled containers and online instances

## Remaining Phases

### Phase 4: Terminal Infrastructure (HIGH PRIORITY)
- PTY management with `portable-pty` crate
- Output streaming via Tauri events
- xterm.js integration with FitAddon, WebLinksAddon
- TerminalPane component with resize handling
- Tab management (create, close, switch)

### Phase 5: ECS Exec Integration (HIGH PRIORITY)
- Spawn `aws ecs execute-command` with PTY
- Shell selection (/bin/sh, /bin/bash)
- Session lifecycle management
- Connect to container from ResourceTree

### Phase 6: SSM Session Integration
- Spawn `aws ssm start-session` with PTY
- Connect to EC2 instances from ResourceTree

### Phase 7: Port Forwarding
- SSM port forwarding
- Modal for configuration
- Service presets (PostgreSQL, MySQL, Redis)

### Phase 8-10: Configuration, Polish, Testing

## Key Commands

```bash
# Development
pnpm tauri dev

# Build
pnpm tauri build

# Check Rust
cargo check --manifest-path src-tauri/Cargo.toml
```

## Important Notes

- User prefers NOT to use corepack
- pnpm is used as package manager (installed via npm)
- Tauri 2.0 shell plugin config: only `"shell": { "open": true }` (no scope)
- xterm.js versions: @xterm/xterm@^5.5.0, @xterm/addon-fit@^0.10.0

## Current State

Just completed the "Add Profile" feature:
- AddProfileModal component supports both Access Key and SSO profiles
- "+" button added next to profile selector in Header
- Profiles are added to ~/.aws/credentials (access keys) or ~/.aws/config (SSO)
- Existing profiles are never modified, only new ones added
- SSO login can be triggered via `aws sso login --profile <name>`

## Next Step

Start Phase 4: Terminal Infrastructure
- Create `src-tauri/src/terminal/pty.rs` for PTY management
- Set up Tauri event streaming for terminal output
- Create TerminalPane.svelte with xterm.js
