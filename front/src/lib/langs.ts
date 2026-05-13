// sjd is commented out because the open model is not good. The actual model used is still closed

// prettier-ignore
const all_langs = [
    "bxr", "chr", "ciw", "cor", "crk", "deu",
    "est", "evn", "fao", "fin", "fit", "fkv",
    "gle", "hdn", "hun", "ipk", "izh", "kal",
    "kca", "koi", "kom", "kpv", "lav", "liv",
    "lut", "mdf", "mhr", "mns", "mrj", "myv",
    "nio", "nno", "nob", "olo", "rmf", "tkl",
    "rup", "rus", "sje", "sju-x-sydlapsk", "sma", "sme", //"sjd",
    "smj", "smn", "sms", "som", "swe", "udm",
    "vep", "vot", "vro", "yrk"
];

// prettier-ignore
export const langs = [
    "bxr", "ciw", "cor", "est", "evn", "fao",
    "fin", "fit", "fkv", "gle", "hdn", "ipk",
    "izh", "kal", "kca", "koi", "kpv", "liv",
    "mdf", "mhr", "mns", "mrj", "myv", "nio",
    "nob", "olo", "rmf", "tkl", "rus", //"sjd",
    "sje", "sju-x-sydlapsk", "sma", "sme", "smj", "smn", "sms",
    "som", "udm", "vep", "vot", "vro", "yrk",
];

// for the filters when selecting language
// prettier-ignore
export const sami_langs = new Set([
    "sje", "sju-x-sydlapsk", //"sjd",
    "sma", "sme",
    "smj", "smn", 
    "sms",
]);

// prettier-ignore
export const nonsamiuralic_langs = new Set([
    "est", "fin", "fkv", "fit", "izh",
    "kca", "koi", "kpv", "liv", "mdf",
    "mhr", "mns", "mrj", "myv", "nio",
    "olo", "udm", "vep", "vot", "vro",
    "yrk",
]);

// prettier-ignore
export const other_langs = new Set([
    "bxr", "ciw", "cor", "crk",
    "evn", "fao", "gle", "hdn",
    "ipk", "kal", "nno", "nob",
    "rmf", "rus", "som", "tkl",
]);

// for determining which tools to show for a given language

// anders: TODO I couldn't find where these were defined. is this correct?
export const dependency_langs = new Set(all_langs);
export const disambiguate_langs = new Set(all_langs);
export const hyphenation_langs = new Set(all_langs);
export const transcription_langs = new Set(all_langs);

// these were defined in cgi-index.xsl
export const analysis_langs = new Set(all_langs);

// prettier-ignore
export const paradigm_langs = new Set([
    "bxr", "ciw", "cor", "evn", "fao", "fin",
    "fit", "fkv", "gle", "ipk", "izh", "kal",
    "kca", "koi", "kpv", "liv", "mdf", "mhr",
    "mns", "mrj", "myv", "nio", "nob", "olo",
    "rus", "sje", "sju-x-sydlapsk", "sma", "sme", "smj", //"sjd",
    "smn", "sms", "som", "udm", "vep", "vot",
    "vro", "yrk",
]);

// prettier-ignore
export const generation_langs = new Set([
    "bxr", "ciw", "cor", "est", "evn", "fao",
    "fin", "gle", "hdn", "ipk", "izh", "kal",
    "kca", "koi", "kpv", "liv", "mdf", "mhr",
    "mns", "mrj", "myv", "nio", "nob", "olo",
    "rus", "tkl", "sje", "sju-x-sydlapsk", "sma", "sme", //"sjd",
    "smj", "smn", "sms", "som", "udm", "vep",
    "vot", "vro", "yrk",
]);

// prettier-ignore
export const num_langs = new Set([
    "fin", "hdn", "kpv", "liv", "mdf", 
    "mhr", "mrj", "myv", "nob","olo", "rus",
    "sma", "sme", //"sjd",
    "smj", "smn", "sms", "tkl", "vro", "yrk",
]);

export const num_lang_details: Record<string, ("numbers" | "date" | "clock")[]> = {
    fin: ["numbers", "date", "clock"],
    hdn: ["numbers", "date", "clock"],
    kpv: ["numbers", "date", "clock"],
    liv: ["numbers"],
    mdf: ["numbers", "date", "clock"],
    mhr: ["numbers", "date"],
    mrj: ["numbers", "date"],
    myv: ["numbers", "date", "clock"],
    nob: ["numbers"],
    olo: ["numbers", "date", "clock"],
    rus: ["numbers", "date", "clock"],
    sma: ["numbers", "date", "clock"],
    sme: ["numbers", "date", "clock"],
    // sjd: ["numbers", "date", "clock"],
    smj: ["numbers", "date"],
    smn: ["numbers", "date"],
    sms: ["numbers", "date", "clock"],
    tkl: ["numbers"],
    vro: ["numbers", "date"],
    yrk: ["numbers", "date", "clock"],
};
export const tools = [
    "analyze",
    "dependency",
    "disambiguate",
    "generate",
    "hyphenate",
    "numbers",
    "paradigms",
    // "spellcheck",
    "transcribe",
    //"ortography",
    //"stedsnavnsordbok",
    //"tallordsgenerator",
] as const;

export type Tools = (typeof tools)[number];

export const tools_for: Record<string, Tools[]> = {};
for (const lang of langs) {
    tools_for[lang] = [];
    if (analysis_langs.has(lang)) tools_for[lang].push("analyze");
    if (dependency_langs.has(lang)) tools_for[lang].push("dependency");
    if (disambiguate_langs.has(lang)) tools_for[lang].push("disambiguate");
    if (generation_langs.has(lang)) tools_for[lang].push("generate");
    if (hyphenation_langs.has(lang)) tools_for[lang].push("hyphenate");
    if (paradigm_langs.has(lang)) tools_for[lang].push("paradigms");
    //if (spellcheck_langs.has(lang)) tools_for[lang].push("spellcheck");
    if (num_langs.has(lang)) tools_for[lang].push("numbers");
    if (transcription_langs.has(lang)) tools_for[lang].push("transcribe");
}
