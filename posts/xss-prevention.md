+++
title = "XSS Prevention: The Complete Guide to Cross-Site Scripting Defense"
date = "2026-06-08"
tags = ["security", "javascript", "web"]
excerpt = "Cross-site scripting remains one of the most prevalent web vulnerabilities. Learn how to prevent stored, reflected, and DOM-based XSS attacks effectively."
+++

Cross-site scripting (XSS) is a vulnerability that allows attackers to inject malicious scripts into web pages viewed by other users. Despite being well understood for over two decades, it consistently appears in the OWASP Top 10.

## Types of XSS

| Type | Description | Example |
|---|---|---|
| **Stored** | Malicious script is saved on the server | Comment field with `<script>alert(1)</script>` |
| **Reflected** | Script is reflected in the response immediately | URL parameter rendered without escaping |
| **DOM-based** | Client-side script dynamically writes untrusted data | `innerHTML` with user-controlled input |

All three are dangerous, but stored XSS is the most severe because it affects every visitor who views the compromised page.

## Contextual Output Encoding

The most important defense is encoding data based on where it appears in the HTML.

```html
<!-- HTML context -->
<div><%= encodeHTML(user.name) %></div>

<!-- Attribute context -->
<input value="<%= encodeAttribute(user.name) %>" />

<!-- JavaScript context -->
<script>
  const data = <%= encodeJSON(user.data) %>;
</script>

<!-- URL context -->
<a href="<%= encodeURL(redirectUrl) %>">Link</a>
```

Using the wrong encoding for a context is a common source of XSS.

## Content Security Policy

CSP is a browser security mechanism that restricts which resources can be loaded and executed.

```
Content-Security-Policy: default-src 'self';
                        script-src 'self' https://cdn.example.com;
                        style-src 'self' 'unsafe-inline';
                        img-src 'self' data:;
```

A strict CSP without `'unsafe-inline'` for scripts stops most XSS attacks even if an injection point exists.

```javascript
// Report violations to a monitoring endpoint
Content-Security-Policy-Report-Only: default-src 'self';
                                     report-uri /csp-report;
```

Use report-only mode to validate your policy before enforcing it.

## Sanitization Libraries

When you must allow user-supplied HTML (rich text editors), use a dedicated sanitization library.

```javascript
import DOMPurify from "dompurify";

const clean = DOMPurify.sanitize(userInput, {
  ALLOWED_TAGS: ["b", "i", "em", "strong", "a"],
  ALLOWED_ATTR: ["href", "title"],
});
```

Never try to write your own HTML sanitizer. The edge cases are endless.

## Safe DOM Manipulation

Prefer safe DOM APIs over dangerous ones.

| Unsafe | Safe |
|---|---|
| `element.innerHTML = data` | `element.textContent = data` |
| `document.write(data)` | `document.createTextNode(data)` |
| `element.outerHTML = data` | `element.insertAdjacentText(...)` |
| `new Function(data)` | Parse JSON instead |

```javascript
// 鉂?Dangerous
document.getElementById("output").innerHTML = userInput;

// 鉁?Safe
document.getElementById("output").textContent = userInput;
```

## Template Engines

Modern frameworks automatically escape output by default. The danger comes from bypassing that protection.

```jsx
// React 鈥?safe by default
return <div>{userInput}</div>;

// React 鈥?dangerous bypass
return <div dangerouslySetInnerHTML={{ __html: userInput }} />;
```

Same pattern exists in Vue (`v-html`), Angular (`[innerHTML]`), and others. Only bypass escaping when you absolutely must, and sanitize the input first.

## HttpOnly Cookies

Marking cookies as `HttpOnly` prevents JavaScript from accessing them. This does not prevent XSS, but it mitigates the damage by protecting session tokens.

```
Set-Cookie: session=abc123; HttpOnly; Secure; SameSite=Strict
```

## Input Validation

Defense in depth means validating input even though output encoding is the primary defense.

```javascript
function validateEmail(email) {
  const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!re.test(email)) {
    throw new Error("Invalid email format");
  }
}
```

Reject input that does not match expected patterns rather than trying to filter dangerous characters.

## XSS in the Real World

Common attack vectors to audit:

- URL fragments and query parameters
- File upload metadata (file names, EXIF data)
- JSONP endpoints
- WebSocket messages
- Server-side rendered data injected into `<script>` tags

## Conclusion

Preventing XSS requires discipline at every layer: contextual encoding, CSP headers, safe DOM APIs, and input validation. No single defense is sufficient. Layer them together and you will stop the vast majority of attacks.
