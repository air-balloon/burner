#!/bin/sh -eu

printf 'Running in: %s\n' "$PWD"
printf '> Listing migrations status...\n'
diesel migration list --migration-dir /app/migrations
printf '> Running migrations...\n'
diesel migration run --migration-dir /app/migrations
printf '> Launching...\n'
air-balloon-burner
