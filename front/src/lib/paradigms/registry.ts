import { convertJsonSchema } from "./json_converter";
import type { LanguageSchema } from "./paradigm_types";
import { parse } from "jsonc-parser";

const schemas = import.meta.glob<string>("./**/*.jsonc", { as: "raw" });

// When .jsonc files are done for lang, add it here
const PARADIGM_LANGS = ["sma", "sme", "fkv"];

const POS_NAMES: Record<string, string> = {
    V: "verb",
    N: "noun",
    A: "adjective",
    Num: "numeral",
    Pron: "pronoun",
};

async function loadRaw(
    lang: string,
    pos: string,
    subclass?: string,
): Promise<string | null> {
    const posName = POS_NAMES[pos];
    if (!posName) return null;
    const suffix = subclass ? `_${subclass.toLowerCase()}` : "";
    const loader = schemas[`./${lang}/${posName}${suffix}.jsonc`];
    return loader ? loader() : null;
}

export async function getParadigmSchema(
    lang: string,
    pos: string,
    subclass: string,
): Promise<LanguageSchema | null> {
    if (!PARADIGM_LANGS.includes(lang)) return null;

    try {
        const raw =
            (subclass ? await loadRaw(lang, pos, subclass) : null) ??
            (await loadRaw(lang, pos));
        if (!raw) return null;

        const schema = parse(raw);
        return schema ? convertJsonSchema(schema) : null;
    } catch (e) {
        console.error(`Failed to load schema for ${lang}`, e);
        return null;
    }
}

export function hasParadigmSchema(lang: string | undefined) {
    return !!lang && PARADIGM_LANGS.includes(lang);
}
