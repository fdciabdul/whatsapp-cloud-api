# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2025-01-11

### Fixed

- GitHub Actions release workflow permissions for tag creation

## [0.1.2] - 2025-01-11

### Fixed

- GitHub Actions release workflow permissions for tag creation

## [0.1.1] - 2025-01-11

### Added

- **Products/Catalog API**:
  - Send single product messages
  - Send multi-product list messages
  - Send catalog messages
  - Get and update commerce settings

- **Flows API**:
  - Send flow messages
  - List flows for WABA
  - Get flow details
  - Create new flows
  - Update flow JSON
  - Publish flows
  - Delete flows
  - Deprecate flows
  - Get flow preview URL

- **Typing Indicators API**:
  - Show typing indicator to users

- **QR Codes API**:
  - Create QR codes with pre-filled messages
  - List all QR codes
  - Get QR code by ID
  - Update QR code message
  - Delete QR codes

- **Analytics API**:
  - Get conversation analytics
  - Get template analytics
  - Get phone number analytics
  - Support for different time granularities (half-hour, daily, monthly)

- **Block Users API**:
  - Block single or multiple users
  - Unblock single or multiple users
  - Get list of blocked users

- **WABA Management API**:
  - Get WABA details
  - Subscribe/unsubscribe webhooks
  - Get subscribed apps
  - Get phone numbers for WABA
  - Get assigned users
  - Get system users
  - Get message templates

- **Webhook Subscriptions API**:
  - Get current webhook subscriptions
  - Create/update webhook subscriptions
  - Delete webhook subscriptions

- **CI/CD**:
  - GitHub Actions for CI (test, fmt, clippy, docs, MSRV)
  - Auto-release workflow with tag generation
  - Auto-publish to crates.io

- **Documentation**:
  - LICENSE (MIT)
  - CONTRIBUTING.md
  - SECURITY.md
  - Comprehensive README with examples

### Changed

- Renamed crate from `whatsapp-cloud-api` to `wacloudapi`
- Updated MSRV to 1.83.0 (required by dependencies)

### Fixed

- Clippy warnings for too_many_arguments and uppercase acronyms
- Code formatting with rustfmt

## [0.1.0] - 2025-01-11

### Added

- Initial release of WhatsApp Cloud API SDK for Rust
- **Client**: HTTP client with Bearer token authentication
  - Support for custom API versions
  - Configurable base URL
  - Async/await support with Tokio

- **Messages API**:
  - Send text messages with optional URL preview
  - Send media messages (image, video, audio, document, sticker)
  - Send location messages
  - Send contact cards (vCard)
  - Send reactions to messages
  - Reply to messages with context
  - Send template messages with components
  - Send interactive messages (buttons, lists)
  - Mark messages as read

- **Media API**:
  - Upload media from file path
  - Upload media from bytes
  - Upload media from base64
  - Get media download URL
  - Download media content
  - Delete media

- **Phone Numbers API**:
  - List phone numbers for a WABA
  - Get phone number by ID
  - Register phone number
  - Deregister phone number
  - Request verification code (SMS/Voice)
  - Verify phone number with code
  - Set two-step verification PIN
  - Get and update business profile

- **Templates API**:
  - List message templates
  - Filter templates by status
  - Get template by name
  - Create new templates
  - Delete templates

- **Webhooks**:
  - Type-safe webhook payload parsing
  - Support for all message types
  - Message status updates (sent, delivered, read, failed)
  - Interactive message responses (button/list replies)
  - Reaction events
  - Error handling
  - Helper method to extract events from payload

- **Error Handling**:
  - Typed API errors with codes
  - Rate limit detection
  - Invalid token detection
  - Detailed error messages

### Dependencies

- `reqwest` for HTTP client
- `serde` / `serde_json` for serialization
- `tokio` for async runtime
- `thiserror` for error handling
- `url` for URL parsing
- `base64` for media encoding
- `mime` / `mime_guess` for MIME type detection

[Unreleased]: https://github.com/fdciabdul/whatsapp-cloud-api-rs/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/fdciabdul/whatsapp-cloud-api-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/fdciabdul/whatsapp-cloud-api-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/fdciabdul/whatsapp-cloud-api-rs/releases/tag/v0.1.0
