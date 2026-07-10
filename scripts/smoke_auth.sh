#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:8080}"
COOKIE_JAR="$(mktemp)"
trap 'rm -f "$COOKIE_JAR" "$BODY_FILE"' EXIT
BODY_FILE="$(mktemp)"

timestamp="$(date +%s)"
email="smoke-${timestamp}@example.test"
password="Password123!"
new_password="Password456!"

csrf_token() {
  awk '$6 == "vel_csrf_token" { value = $7 } END { print value }' "$COOKIE_JAR"
}

request() {
  local method="$1"
  local path="$2"
  local expected="$3"
  local data="${4:-}"
  local csrf="${5:-false}"
  local status
  local -a args=(
    -sS
    -X "$method"
    -b "$COOKIE_JAR"
    -c "$COOKIE_JAR"
    -o "$BODY_FILE"
    -w "%{http_code}"
    -H "Content-Type: application/json"
  )

  if [[ "$csrf" == "true" ]]; then
    args+=(-H "X-CSRF-Token: $(csrf_token)")
  fi

  if [[ -n "$data" ]]; then
    args+=(-d "$data")
  fi

  status="$(curl "${args[@]}" "${BASE_URL}${path}")"

  if [[ "$status" != "$expected" ]]; then
    echo "FAIL ${method} ${path}: expected ${expected}, got ${status}" >&2
    echo "Response body:" >&2
    cat "$BODY_FILE" >&2
    exit 1
  fi

  echo "OK   ${method} ${path} -> ${status}"
}

request GET /health 200
request GET /auth/csrf 204

request POST /auth/register 201 "{
  \"full_name\":\"Smoke Tester\",
  \"email\":\"${email}\",
  \"password\":\"${password}\",
  \"account_type\":\"customer\",
  \"default_location\":\"Abidjan\"
}"

request GET /auth/me 200

request POST /auth/change-password 204 "{
  \"current_password\":\"${password}\",
  \"new_password\":\"${new_password}\"
}" true

request POST /auth/logout 204 "" true

request POST /auth/login 200 "{
  \"identifier\":\"${email}\",
  \"password\":\"${new_password}\"
}"

request POST /auth/refresh 200 "" true

echo "Auth smoke test completed for ${email}"

