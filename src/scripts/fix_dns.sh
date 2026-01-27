: <<'JSDOC'
/**
 * @purpose
 * Ensure local /etc/hosts maps blocked websocket domains to known IPs.
 *
 * @dependencies
 * - sudo (or root) access to update /etc/hosts
 *
 * @notes
 * - Rewrites existing mappings for the target hosts to ensure correct resolution.
 */
JSDOC
set -euo pipefail

HOSTS_FILE="/etc/hosts"
BINANCE_HOST="stream.binance.com"
BINANCE_IP="35.74.168.201"
POLYMARKET_HOST="ws-subscriptions-clob.polymarket.com"
POLYMARKET_IP="172.64.153.51"

if [ ! -f "$HOSTS_FILE" ]; then
  echo "Missing $HOSTS_FILE; cannot update hosts." >&2
  exit 1
fi

SUDO=""
if [ "$(id -u)" -ne 0 ]; then
  if ! command -v sudo >/dev/null 2>&1; then
    echo "sudo is required to update $HOSTS_FILE." >&2
    exit 1
  fi
  SUDO="sudo"
fi

tmp_file="$(mktemp)"
cleanup() {
  rm -f "$tmp_file"
}
trap cleanup EXIT

awk -v host1="$BINANCE_HOST" -v host2="$POLYMARKET_HOST" '
  {
    if ($0 ~ ("(^|[[:space:]])" host1 "([[:space:]]|$)") ||
        $0 ~ ("(^|[[:space:]])" host2 "([[:space:]]|$)")) {
      next
    }
    print
  }
' "$HOSTS_FILE" > "$tmp_file"

{
  echo ""
  echo "# Bankai Terminal websocket DNS overrides"
  echo "$BINANCE_IP $BINANCE_HOST"
  echo "$POLYMARKET_IP $POLYMARKET_HOST"
} >> "$tmp_file"

if cmp -s "$HOSTS_FILE" "$tmp_file"; then
  echo "$HOSTS_FILE already contains required mappings."
  exit 0
fi

$SUDO cp "$tmp_file" "$HOSTS_FILE"
echo "Updated $HOSTS_FILE with websocket DNS overrides."
