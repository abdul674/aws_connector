# AWS Connector - Feature Roadmap

## Completed Phases

### Phase 1: Project Setup
- [x] Tauri 2.0 + Svelte 5 + TypeScript
- [x] Tailwind CSS with dark theme
- [x] Basic UI shell (Sidebar, Header, MainContent, StatusBar)

### Phase 2: AWS Profile Management
- [x] Parse ~/.aws/credentials and ~/.aws/config
- [x] Profile selector in header
- [x] Add profile (Access Keys + SSO)
- [x] Credential validation via STS
- [x] SSO login support

### Phase 3: AWS Resource Discovery
- [x] ECS: clusters → services → tasks → containers
- [x] EC2: SSM-enabled instances
- [x] ResourceTree component with collapsible nodes

### Phase 4: Terminal Infrastructure
- [x] PTY management with portable-pty
- [x] Output streaming via Tauri events
- [x] xterm.js integration
- [x] Tab management (create, close, switch)

### Phase 5: ECS Exec Integration
- [x] Spawn `aws ecs execute-command` with PTY
- [x] Shell selection (/bin/sh, /bin/bash)
- [x] Session lifecycle management

### Phase 6: SSM Session Integration
- [x] Spawn `aws ssm start-session` with PTY
- [x] Connect to EC2 instances

### Phase 7: Port Forwarding
- [x] SSM port forwarding
- [x] Configuration modal
- [x] Service presets (PostgreSQL, MySQL, Redis)

### Phase 8: Settings & Configuration
- [x] Settings modal
- [x] Configurable options (font size, shell, sidebar width)
- [x] Keyboard shortcuts

### Phase 9: UI Polish
- [x] Welcome screen with action cards
- [x] Toast notifications
- [x] Loading states
- [x] Error handling

---

## New Features (In Progress)

### Phase 10: CloudWatch Logs Tailing ✅
- [x] List CloudWatch log groups
- [x] Real-time log streaming (2-second polling)
- [x] Log viewer with auto-scroll
- [x] Filter by pattern
- [x] Color-coded log levels (ERROR, WARN, INFO)

### Phase 11: S3 File Browser ✅
- [x] List S3 buckets
- [x] Browse folders with breadcrumb navigation
- [x] Back/Forward/Up navigation
- [x] File actions (preview, copy presigned URL, delete)
- [x] Text file preview with line numbers

---

## Planned Features

### Phase 12: Secrets Manager / Parameter Store
- [ ] List secrets from AWS Secrets Manager
- [ ] View/copy secret values
- [ ] List SSM Parameter Store parameters
- [ ] Hierarchical parameter view

### Phase 13: Lambda Invoke & Logs
- [ ] List Lambda functions
- [ ] Invoke with custom payload
- [ ] View execution logs
- [ ] Function metrics

### Phase 14: SQS Queue Management
- [ ] List SQS queues
- [ ] Send messages
- [ ] Receive/view messages
- [ ] Purge queue
- [ ] DLQ management

### Phase 15: ECR Image Browser
- [ ] List ECR repositories
- [ ] View image tags
- [ ] Image scan findings
- [ ] Copy image URI

### Phase 16: Multi-Account Switcher
- [ ] Quick account switching
- [ ] Account aliases/nicknames
- [ ] Favorite accounts

### Phase 17: Cost Dashboard
- [ ] Current month costs
- [ ] Cost breakdown by service
- [ ] Cost trends
- [ ] Budget alerts

### Phase 18: RDS Instance Management
- [ ] List RDS instances
- [ ] Connection info
- [ ] Start/Stop instances
- [ ] Snapshot management

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Svelte 5 + TypeScript |
| Styling | Tailwind CSS + CSS Variables |
| Terminal | xterm.js |
| Backend | Tauri 2.0 (Rust) |
| PTY | portable-pty |
| AWS SDK | aws-sdk-* (Rust) |

## AWS SDKs Used

| SDK | Purpose |
|-----|---------|
| aws-sdk-sts | Credential validation |
| aws-sdk-ecs | ECS resource discovery |
| aws-sdk-ec2 | EC2 instance discovery |
| aws-sdk-ssm | SSM session management |
| aws-sdk-cloudwatchlogs | Log tailing |
| aws-sdk-s3 | S3 file browser |
