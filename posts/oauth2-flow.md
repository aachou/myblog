+++
title = "OAuth 2.0 Flows Explained with Code Examples"
date = "2023-11-09"
tags = ["oauth2", "security", "authorization"]
excerpt = "OAuth 2.0 is the industry-standard protocol for authorization. This guide explains the four authorization grant types, PKCE, and practical implementation patterns."
+++

OAuth 2.0 is not an authentication protocol — it is an authorization framework that enables applications to obtain limited access to user accounts on an HTTP service. Understanding each grant type is essential for building secure integrations.

## The Four Grant Types

| Grant Type | Use Case | Security Level |
|------------|----------|----------------|
| Authorization Code | Server-side web apps | Highest |
| Implicit (deprecated) | Single-page apps | Low |
| Resource Owner Password | Legacy / first-party apps | Medium |
| Client Credentials | Machine-to-machine | High |

## Authorization Code Flow (Recommended)

The most secure flow for applications with a backend:

```text
    ┌────────┐       ┌──────────┐       ┌────────┐
    │  User  │       │    App   │       │  Auth  │
    │        │       │          │       │ Server │
    └────┬───┘       └────┬─────┘       └────┬───┘
         │                │                   │
         │  Login Click   │                   │
         │───────────────>│                   │
         │                │  Auth Request     │
         │                │──────────────────>│
         │                │                   │
         │  Auth Prompt   │                   │
         │<───────────────────────────────────│
         │                │                   │
         │  Approve       │                   │
         │───────────────────────────────────>│
         │                │                   │
         │                │  Auth Code        │
         │                │<──────────────────│
         │                │                   │
         │                │  Code + Secret    │
         │                │──────────────────>│
         │                │                   │
         │                │  Access Token     │
         │                │<──────────────────│
         │                │                   │
```

## Implementation with Express

```javascript
const express = require("express");
const axios = require("axios");
const querystring = require("querystring");
const crypto = require("crypto");

const app = express();

const CLIENT_ID = process.env.OAUTH_CLIENT_ID;
const CLIENT_SECRET = process.env.OAUTH_CLIENT_SECRET;
const REDIRECT_URI = "http://localhost:3000/callback";

// Step 1: Redirect user to auth server
app.get("/auth/login", (req, res) => {
    const params = querystring.stringify({
        client_id: CLIENT_ID,
        redirect_uri: REDIRECT_URI,
        response_type: "code",
        scope: "openid profile email",
        state: crypto.randomBytes(16).toString("hex")
    });
    res.redirect(`https://auth.example.com/authorize?${params}`);
});

// Step 2: Handle callback with authorization code
app.get("/callback", async (req, res) => {
    const { code, state } = req.query;

    // Exchange code for token
    const tokenResponse = await axios.post(
        "https://auth.example.com/token",
        querystring.stringify({
            grant_type: "authorization_code",
            code,
            redirect_uri: REDIRECT_URI,
            client_id: CLIENT_ID,
            client_secret: CLIENT_SECRET
        }),
        { headers: { "Content-Type": "application/x-www-form-urlencoded" } }
    );

    const { access_token, refresh_token, id_token } = tokenResponse.data;

    // Fetch user info
    const userResponse = await axios.get(
        "https://auth.example.com/userinfo",
        { headers: { Authorization: `Bearer ${access_token}` } }
    );

    res.json({ user: userResponse.data, tokens: { access_token, refresh_token } });
});
```

## PKCE for Mobile and SPA

PKCE (Proof Key for Code Exchange) adds a secret that even a public client can use securely:

```javascript
// Generate code verifier and challenge
function generatePKCE() {
    const verifier = crypto.randomBytes(32).toString("base64url");
    const challenge = crypto
        .createHash("sha256")
        .update(verifier)
        .digest("base64url");
    return { verifier, challenge };
}

// Include challenge in auth request
const { verifier, challenge } = generatePKCE();
const authUrl = `https://auth.example.com/authorize?` +
    `response_type=code&` +
    `client_id=${CLIENT_ID}&` +
    `redirect_uri=${REDIRECT_URI}&` +
    `code_challenge=${challenge}&` +
    `code_challenge_method=S256`;
```

## Client Credentials Flow

For server-to-server communication without user context:

```bash
curl -X POST https://auth.example.com/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=client_credentials" \
  -d "client_id=$CLIENT_ID" \
  -d "client_secret=$CLIENT_SECRET" \
  -d "scope=api:read api:write"
```

## Common Mistakes

- Storing client secrets in mobile apps or browser bundles
- Not validating the `state` parameter (CSRF protection)
- Using Implicit flow for new apps (always use Auth Code + PKCE)
- Not validating token expiration before making API calls
- Forgetting to rotate client secrets and signing keys

OAuth 2.0 can feel complex, but the Authorization Code flow with PKCE covers nearly every modern application scenario. Leverage well-maintained libraries rather than implementing the protocol from scratch.
