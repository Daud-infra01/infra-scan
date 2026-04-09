#!/bin/bash
echo "=== RUNNING PROCESSES ==="
for pid in /proc/[0-9]*/; do
    name=$(cat "$pid/status" 2>/dev/null | grep "^Name" | awk '{print $2}')
    ram=$(cat "$pid/status" 2>/dev/null | grep "^VmRSS" | awk '{print $2, $3}')
    if [ -n "$name" ] && [ -n "$ram" ]; then
        echo "PID: $(basename $pid) | Name: $name | RAM: $ram"
    fi
done

