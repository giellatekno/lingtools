import { convertJsonSchema } from "./json_converter";
import type { LanguageSchema } from "./paradigm_types";
import { parse } from "jsonc-parser";

type LangMapping = Record<string, Record<string, string>>;

const REGISTRY: Record<string, () => Promise<LangMapping>> = {
    // Once a new language folder is implemented, it can be added here
    sma: () => import("./sma").then((m) => m.mapping),
    sme: () => import("./sme").then((m) => m.mapping),
    // smj: () => import("./smj").then((m) => m.mapping),
    // smn: () => import("./smn").then((m) => m.mapping),
    fkv: () => import("./fkv").then((m) => m.mapping),
};

export async function getParadigmSchema(
    lang: string,
    pos: string,
    subclass: string,
): Promise<LanguageSchema | null> {
    const loader = REGISTRY[lang];
    if (!loader) return null;

    try {
        const langMap = await loader();
        const category = langMap[pos];
        if (!category) return null;

        const rawData = category[subclass] ?? category["default"];
        if (!rawData) return null;

        const schema = parse(rawData);
        return schema ? convertJsonSchema(schema) : null;
    } catch (error) {
        console.error(`Failed to load schema for ${lang}`, error);
        return null;
    }
}

export function hasParadigmSchema(lang: string | undefined) {
    if (!lang) return false;
    return Object.keys(REGISTRY).includes(lang);
}
