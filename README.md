# Wirect

A small Rust service for redirecting one domain to another while preserving the path and query string.

## Configuration

Set these environment variables:

```env
NEW_DOMAIN=https://newdomain.com
HOST=0.0.0.0
PORT=8080
```

Notes:

- `NEW_DOMAIN` should include the target scheme, for example `https://newdomain.com`.
- `HOST` is optional. If omitted, the server uses `0.0.0.0`.
- `PORT` is optional. If omitted, the server uses port `8080`.

## Run

```bash
cargo run
```

## Behavior

- `GET /health` returns `200 OK`.
- Any incoming request is redirected with HTTP `308 Permanent Redirect`.
- The original path and query string are preserved.

## Example

```bash
curl -I \
  "http://127.0.0.1:8080/blog/post?id=42"
```

Expected response:

```text
HTTP/1.1 308 Permanent Redirect
location: https://newdomain.com/blog/post?id=42
```

## Docker

Build image:

```bash
docker build -t wirect .
```

Run container:

```bash
docker run --rm -p 8080:8080 -e NEW_DOMAIN=https://newdomain.com wirect
```
