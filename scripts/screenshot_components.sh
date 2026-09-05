#!/usr/bin/env bash
# screenshot_components.sh
# Launches the widgetbook headlessly (via Xvfb) and screenshots every component
# page by driving a simple stdin protocol. Outputs one PNG per component.
#
# Prerequisites (installed by CI):
#   - xvfb  (virtual framebuffer)
#   - scrot  (screenshot utility)
#   - cargo (rust toolchain)
#
# Usage:
#   ./scripts/screenshot_components.sh [output_dir]
#
set -euo pipefail

OUT="${1:-screenshots}"
mkdir -p "$OUT"

DISPLAY_NUM=99
export DISPLAY=":${DISPLAY_NUM}"

# Start virtual framebuffer if not running
if ! pgrep -x Xvfb > /dev/null; then
  Xvfb ":${DISPLAY_NUM}" -screen 0 1280x800x24 &
  XVFB_PID=$!
  trap "kill $XVFB_PID 2>/dev/null || true" EXIT
  sleep 1
fi

echo "Building widgetbook (release)..."
cargo build --example widgetbook --release

# Component list — must match the ComponentPage enum in widgetbook.rs
COMPONENTS=(
  "Button"
  "ButtonVariants"
  "Checkbox"
  "Radio"
  "Switch"
  "ToggleGroup"
  "TextInput"
  "Textarea"
  "Slider"
  "SearchInput"
  "OtpInput"
  "TagInput"
  "Rating"
  "Badge"
  "Chip"
  "Avatar"
  "Card"
  "Table"
  "Keyboard"
  "CodeBlock"
  "ListItem"
  "Listbox"
  "StatsCard"
  "Timeline"
  "EmptyState"
  "Tabs"
  "Dropdown"
  "Breadcrumb"
  "Pagination"
  "SegmentedControl"
  "Stepper"
  "Tooltip"
  "Alert"
  "Toast"
  "Accordion"
  "LucideIcons"
  "Carousel"
  "TreeView"
  "Heatmap"
  "UserCard"
)

echo "Launching widgetbook headlessly..."
./target/release/examples/widgetbook &
APP_PID=$!
trap "kill $APP_PID $XVFB_PID 2>/dev/null || true" EXIT

# Wait for window to appear
sleep 2

# Screenshot each component using xdotool to click sidebar items
for COMP in "${COMPONENTS[@]}"; do
  echo "  Screenshotting: $COMP"

  # Use xdotool to find window and type the search query
  WID=$(xdotool search --name "hero-floem Widgetbook" 2>/dev/null | head -1 || true)
  if [ -z "$WID" ]; then
    echo "  [warn] Window not found, using full-screen screenshot"
    scrot "$OUT/${COMP}.png" -d 0
    continue
  fi

  # Focus the window
  xdotool windowfocus --sync "$WID"

  # Click and type in search box (top of sidebar, roughly x=110 y=60)
  xdotool mousemove --window "$WID" 110 60
  xdotool click 1
  sleep 0.1
  # Clear current search and type component name
  xdotool key ctrl+a
  xdotool type --clearmodifiers "$COMP"
  sleep 0.3

  # Click first result in sidebar (roughly x=110 y=120)
  xdotool mousemove --window "$WID" 110 120
  xdotool click 1
  sleep 0.4

  # Clear search so next iteration starts clean
  xdotool mousemove --window "$WID" 110 60
  xdotool click 1
  xdotool key ctrl+a
  xdotool type --clearmodifiers ""
  sleep 0.1

  # Take screenshot of just the content area
  GEOM=$(xdotool getwindowgeometry --shell "$WID" 2>/dev/null)
  eval "$GEOM" 2>/dev/null || true
  X="${X:-0}"; Y="${Y:-0}"; WIDTH="${WIDTH:-1280}"; HEIGHT="${HEIGHT:-800}"
  # Content starts after sidebar (220px) and topbar (~52px)
  CONTENT_X=$((X + 220))
  CONTENT_Y=$((Y + 52))
  CONTENT_W=$((WIDTH - 220))
  CONTENT_H=$((HEIGHT - 52))

  scrot "$OUT/${COMP}.png" -a "${CONTENT_X},${CONTENT_Y},${CONTENT_W},${CONTENT_H}"
  echo "  → Saved $OUT/${COMP}.png"
done

echo ""
echo "Done! Screenshots saved to: $OUT/"
echo "Total: ${#COMPONENTS[@]} components"
