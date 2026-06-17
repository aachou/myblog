+++
title = "JWT Authentication: A Complete Guide"
date = "2023-03-10"
tags = ["security", "jwt", "authentication"]
excerpt = "JSON Web Tokens are the de facto standard for stateless authentication. This guide covers token structure, signing algorithms, refresh tokens, and security best practices."
+++

JSON Web Tokens (JWT) provide a compact, URL-safe means of representing claims between parties. They are widely used for authentication and information exchange in modern web applications.

## Token Structure

A JWT consists of three parts separated by dots: Header, Payload, and Signature.

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.
eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.
SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

### Header

The header typically consists of the token type and the signing algorithm:

```json

{
    "alg": "HS256",
    "typ": "JWT"
}
```

### Payload

The payload contains claims. Registered claims include `sub` (subject), `iat` (issued at), and `exp` (expiration):


```json
{
  "sub": "user_42",
  "name": "Jane Doe",
  "role": "admin",
  "iat": 1672531200,
  "exp": 1672617600
}
```

| Claim | Description | Required |
|-------|-------------|----------|
| `iss` | Issuer of the token | Optional |
| `sub` | Subject (usually user ID) | Recommended |
| `aud` | Audience | Optional |
| `exp` | Expiration time | Recommended |
| `iat` | Issued at | Recommended |

## Signing Algorithms

Choosing the right algorithm is critical:

1. **HS256 (HMAC + SHA-256)** — Symmetric; same secret signs and verifies. Fast but requires sharing the secret securely.
2. **RS256 (RSA + SHA-256)** — Asymmetric; private key signs, public key verifies. Suitable for microservice architectures where multiple services need to verify tokens without holding the signing secret.
3. **ES256 (ECDSA)** — Asymmetric with elliptic curves; smaller signatures than RSA.

```javascript
const jwt = require('jsonwebtoken');

const payload = { sub: 'user_42', role: 'admin' };
const secret = process.env.JWT_SECRET;

const token = jwt.sign(payload, secret, {
    algorithm: 'HS256',
    expiresIn: '1h'
});
```

## Refresh Token Flow

Access tokens are short-lived (15-30 minutes). A refresh token, stored securely client-side, obtains new access tokens without requiring re-authentication:

```text
1. Client sends refresh token to /auth/refresh
2. Server validates refresh token
3. Server issues new access token (+ optional new refresh token)
4. Client stores tokens and retries the original request
```

## Security Best Practices

- Store tokens in HTTP-only cookies, not `localStorage`, to mitigate XSS attacks.
- Always validate the signature and expiration on every request.
- Use short expiration times for access tokens (15 minutes).
- Implement token blacklisting for immediate revocation when needed.
- Never log tokens or include them in URLs.

```javascript
function authenticateToken(req, res, next) {
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (!token) return res.sendStatus(401);

    jwt.verify(token, process.env.JWT_SECRET, (err, user) => {
        if (err) return res.sendStatus(403);
        req.user = user;
        next();
    });
}
```

## Common Vulnerabilities

- **Algorithm confusion** — Always whitelist accepted algorithms and verify the token against the correct key.
- **None algorithm attack** — Never accept tokens with `"alg": "none"` in production.
- **Weak secrets** — Use secrets with at least 256 bits of entropy.

JWTs are a powerful tool, but they require disciplined implementation. When used correctly, they provide a scalable, stateless authentication mechanism suitable for distributed systems.
