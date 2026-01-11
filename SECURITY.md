# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in wacloudapi, please report it responsibly:

1. **Do NOT** open a public GitHub issue
2. Email the maintainer at: cp@imtaqin.id
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggested fixes (optional)

## Response Timeline

- **Initial response**: Within 48 hours
- **Status update**: Within 7 days
- **Fix timeline**: Depends on severity

## Security Best Practices

When using this library:

1. **Never commit tokens**: Store access tokens in environment variables or secure vaults
2. **Use HTTPS**: Always use HTTPS endpoints (default behavior)
3. **Rotate tokens**: Regularly rotate your WhatsApp Cloud API access tokens
4. **Limit permissions**: Use tokens with minimal required permissions
5. **Validate webhooks**: Verify webhook signatures to prevent spoofing

## Acknowledgments

We appreciate security researchers who help keep wacloudapi safe. Contributors who report valid security issues will be acknowledged (with permission) in our release notes.
