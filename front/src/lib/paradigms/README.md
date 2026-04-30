# Paradigm table definitions

These files define how paradigm tables are laid out on the LingTools webpage. The tool gets wordforms + morphological tags from the API and uses these files to arrange them into tables.

## Adding a language

1. Create a folder named with the ISO 639-3 code (e.g. `smj/`)
2. Add JSONC files for each part of speech (copy from `sme/` as a starting point)
3. Add the language code to `PARADIGM_LANGS` in `registry.ts`

## Filenames

Each file covers one part of speech or a subclass of a POS. The naming convention:

| File | POS |
|---|---|
| `noun.jsonc` | Noun |
| `verb.jsonc` | Verb |
| `verb_neg.jsonc` | Negative verb |
| `adjective.jsonc` | Adjective |
| `adjective_ord.jsonc` | Ordinal adjective |
| `numeral.jsonc` | Numeral |
| `pronoun_pers.jsonc` | Personal pronoun |
| `pronoun_dem.jsonc` | Demonstrative |
| `pronoun_refl.jsonc` | Reflexive |
| `pronoun_rel.jsonc` | Relative |
| `pronoun_indef.jsonc` | Indefinite |
| `pronoun_interr.jsonc` | Interrogative |
| `pronoun_recipr.jsonc` | Reciprocal |

Missing files will fall back to a plain list view.

## File structure

A file contains **sections** (e.g. "indicative", "conditional"), each with one or more **tables**, each with column **headers** and **rows**.

Each row has a `label` (shown in the first column) and `tags` — one FST tag per data column. The tool finds the wordform whose tag string contains that substring.

```jsonc
{
    "$schema": "../paradigm_schema.json",
    "sections": [
        {
            "title": "indicative",
            "validateRows": true,
            "tables": [
                {
                    "title": "present",
                    "headers": ["person", "positive", "negative"],
                    "rows": [
                        { "label": "mun", "tags": ["Ind+Prs+Sg1", "Ind+Prs+ConNeg"], "prefixes": ["", "in"] },
                        { "label": "don", "tags": ["Ind+Prs+Sg2", "Ind+Prs+ConNeg"], "prefixes": ["", "it"] },
                        { "label": "son", "tags": ["Ind+Prs+Sg3", "Ind+Prs+ConNeg"], "prefixes": ["", "ii"], "separator": true },
                    ]
                }
            ]
        }
    ]
}
```

### Row options

| Option | Meaning |
|---|---|
| `prefixes` | Text prepended to each form, e.g. `["", "in"]` for a neg column |
| `separator` | Draw a line below this row |
| `colspan` | Form spans multiple columns (e.g. essive with no number distinction) |

### validateRows

When `"validateRows": true` is set on a section, a row is only shown if its **first tag** (the positive/main form) has a result. This prevents orphaned negation or perfect forms from appearing for rows where the positive form is absent. For example, *árvit* (to rain) only has Sg3 and Pl3 forms — with `validateRows`, the 1st and 2nd person rows are hidden entirely rather than showing only a negation form with no corresponding positive.

### showIf

Both sections and tables accept `"showIf": ["Tag1", "Tag2"]`. The section/table is hidden unless all listed tags appear somewhere in the paradigm results.

## Translations

Every `title`, `label`, and `headers` string is automatically looked up in the translation system. The lookup adds the prefix `paradigm_`, so `"nominative"` looks for the key `paradigm_nominative`.

- If the key exists, the text is shown in the user's language (Norwegian, North Sami, etc.)
- If not, the string is shown literally. Correct for language-specific pronouns like `"mun"` or terms with no generic translation.

Most grammatical terms (cases, moods, tenses, persons, numbers) are already translated. To check if a key exists, or to add a new one, look in `src/lib/paraglide/messages/`.
