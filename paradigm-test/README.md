# Paradigm Tester

A standalone browser tool for testing paradigm API responses against the JSONC table schemas.

For each search result it shows:
- **In API, not in schema** (yellow): tags returned by the API that no schema table row covers. These are the critical gaps.
- **Schema tables**: the paradigm rendered using the table structure defined in `front/src/lib/paradigms/`.
- **In schema, no API match** (collapsed): schema tags with no returned form. Less critical. Many words simply lack certain forms, but it may also indicate missing forms in the analyser. See [Why doesn't this wordform show up?](#why-doesnt-this-wordform-show-up).

## Running

```sh
make run
```

Open the printed URL. The API field at the top of the page defaults to the production API. Change it if needed (e.g. `http://localhost:3000` for a local instance).

## Running a local API instance

The API is a Rust server in `../back/`. Requires the Giella language models installed
(typically at `/usr/share/giella/`).

```sh
cd ../back
cp .env.default .env   # first time only. Set WP_LANGFOLDER in .env
just dev               # starts on port 3000 with auto-reload
```

See `../back/Readme.md` for more details.

## Schema files

The JSONC table schemas live in `../front/src/lib/paradigms/{lang}/`.
Adding a new language only requires adding JSONC files there. The tester discovers them automatically.
See the `README.md` in `../front/src/lib/paradigms/` for more details about the JSONC files.

## Why doesn't this wordform show up?

If an expected wordform is present in the JSONC files, but is not returned from the API, there are two likely causes:

### 1. The API does not generate the wordform

The API uses two files to determine which wordforms to generate for a given lemma, both found in each `lang-xxx` repository:

- `devtools/testdata/korpustags.xxx.txt`: groups related tags under keywords (e.g. all case tags under `Case`)
- `src/fst/morphology/test/paradigm_full.xxx.txt`: lists all tag combinations the API will attempt to generate

The API only returns combinations that produce an actual word. For example, `giella+N+Sg+Nom` is returned, but `giella+V+Inf` is not, because that combination doesn't exist.

If a wordform is missing, `paradigm_full.xxx.txt` is the most likely culprit. Check there first.

### 2. The wordform is missing from the analyser

If the relevant tags are present in the files above and the wordform should exist, it may be missing from the analyser. Double-check with the `hfst-lookup` commands (`huxxx` and `hdxxx`).
