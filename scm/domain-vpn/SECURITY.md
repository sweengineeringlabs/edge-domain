# Security Policy

## Reporting

Report security vulnerabilities to **security@swelabs.io**.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Notes

- `server_public_key` and `psk` are validated at config load time (base64 decode + length check).
- `OnError::Fail` ensures misconfigured VPN sections abort startup rather than silently running without a tunnel.
- No secrets are logged or included in error messages.
