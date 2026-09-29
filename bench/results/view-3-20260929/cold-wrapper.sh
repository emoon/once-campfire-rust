#!/bin/sh
# The fragment cache stores nothing: every message renders its rich text on every request.
CAMPFIRE_FRAGMENT_CACHE_MB=0 exec /home/emoon/campfire-wt/view-3/target/base-cold-bin "$@"
