# Paradigm Tester

A standalone browser tool for testing paradigm API responses against the JSONC table schemas.

For each search result it shows:
- **In API, not in schema** (yellow) — tags returned by the API that no schema table row covers. These are the critical gaps.
- **Schema tables** — the paradigm rendered using the table structure defined in `client/src/lib/paradigms/`.
- **In schema, no API match** (collapsed) — schema tags with no returned form. Less critical, as many words simply lack certain forms.

## Running

Serve from the `lingtools/` root so the schema JSONC files are reachable:

```sh
# Against the production API (gtweb.uit.no/lingtools)
make run

# Against a local API instance (localhost:3000)
make run-local
```

Both commands start a Python HTTP server on port 8080 and open the browser automatically.
Override the port with `PORT=9000 make run`.

## Running a local API instance

The API is a Rust server in `../back/`. Requires the Giella language models installed
(typically at `/usr/share/giella/`).

```sh
cd ../back
cp .env.default .env   # first time only — set WP_LANGFOLDER in .env
just dev               # starts on port 3000 with auto-reload
```

See `../back/Readme.md` for more details.

## Schema files

The JSONC table schemas live in `../client/src/lib/paradigms/{lang}/`.
The tester fetches them at runtime — no build step needed.
Adding a new language only requires adding JSONC files there; the tester discovers them automatically.
