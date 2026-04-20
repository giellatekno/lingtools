const show_each_dict = false;

const all_langs = new Set([
    "bxr", "ciw", "cor", "est", "evn",
    "fao", "fin", "fit", "fkv", "gle",
    "hdn", "ipk", "izh", "kal", "kca",
    "koi", "kpv", "liv", "mdf", "mhr",
    "mns", "mrj", "myv", "nio", "nob",
    "olo", "rmf", "rus", "sjd", "sje",
    "sma", "sme", "smj", "smn", "sms",
    "som", "udm", "vep", "vot", "vro",
    "yrk",
]);

const analysis_langs = new Set(all_langs);

const paradigm_langs = new Set([
    "bxr", "ciw", "cor", "evn", "fao",
    "fin", "fit", "fkv", "gle", "ipk",
    "izh", "kal", "kca", "koi", "kpv",
    "liv", "mdf", "mhr", "mns", "mrj",
    "myv", "nio", "nob", "olo", "rus",
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms", "som", "udm", "vep",
    "vot", "vro", "yrk",
]);

const generation_langs = new Set([
    "bxr", "ciw", "cor", "est", "evn",
    "fao", "fin", "gle", "hdn", "ipk",
    "izh", "kal", "kca", "koi", "kpv",
    "liv", "mdf", "mhr", "mns", "mrj",
    "myv", "nio", "nob", "olo", "rus",
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms", "som", "udm", "vep",
    "vot", "vro", "yrk",
]);

const num_langs = new Set([
    "fin", "hdn", "liv", "mdf", "mhr",
    "myv", "olo", "rus", "sjd", "sma",
    "sme", "smj", "smn", "sms", "yrk",
]);

const dict_langs = new Set([
    "fin", "fkv", "hdn", "izh", "koi",
    "kpv", "liv", "nob", "olo", "sjd",
    "sje", "sjt", "sma", "sme", "smj",
    "smn", "sms", "udm", "vep", "vot",
    "vro",
]);

const mordict_langs = new Set([
    "mdf", "myv",
]);

const maridict_langs = new Set([
    "mhr", "mrj",
]);

const vadadict_langs = new Set([
    "mns", "yrk",
]);

const geo_langs = new Set([
    "sma", "sme", "smj",
]);

const risten_langs = new Set([
    "sma", "sme", "smj",
]);

const webdict_langs = new Set([
    "fkv", "kpv", "mdf", "myv", "sjd",
    "sma", "sme", "smj", "sms",
]);

const vd_langs = new Set([
    "sme", "sma", "fkv",
]);


const dpage = f"d-{lang}.{pagelang}.html";
const ppage = f"d-{lang}.{pagelang}.html";
const gpage = f"d-{lang}.{pagelang}.html";
const numpage = f"/num.{pagelang}.html";
const npage = f"n-{lang}.{pagelang}.html";


let webdict_page = null
if (lang === "smj") {
    webdict_page = "ak/smj2nob";
}


let nds_page = null;
if (["fkv", "olo", "vep"].includes(lang)) {
    nds_page = "sanat";
} else if (["hdn"].includes(lang)) {
    nds_page = "guusaaw";
} else if (["izh", "liv", "vro"].includes(lang)) {
    nds_page = "sonad";
} else if (["kpv", "udm", "koi"].includes(lang)) {
    nds_page = "kyv";
} else if (["mdf", "myv"].includes(lang)) {
    nds_page = "valks";
}
elif lang in ("mrh", "mrj"):
    nds_page = "muter"
elif lang in ("sjd", ):
    nds_page = "sanj"
elif lang in ("sma", ):
    nds_page = "baakoeh"
elif lang in ("sme", ):
    nds_page = "sanit"
elif lang in ("smn", ):
    nds_page = "saanih"
elif lang in ("sms", ):
    nds_page = "saan"
elif lang in ("yrk", ):
    nds_page = "vada"


grammar_page = None
if lang == "sme" and
    pagelang in ("nob", "sma", "smj", "rus"):
    grammar_page = "grammatihkka.nob"
elif lang == "sme":
    if pagelang in ("sme", "fin"):
        grammar_page = "grammatihkka"
    elif pagelang in ("eng", ):
        grammar_page = "grammatihkka.eng"
    elif pagelang in ("rus", ):
        grammar_page = "grammatihkka.eng"
elif lang == "sma":
    grammar_page = "grammatikk.nob"


korp_page = None
if lang in ("sme", "sma", "smj", "sjd", "smn"):
    korp_page = "korp"
elif lang in ("fao", "fit", "fkv", "olo", "vep", "vro"):
    korp_page = "f_korp"
elif lang in ("kpv", "koi", "mdf", "mhr", "mrj", "myv", "udm"):
    korp_page = "u_korp"


/*
keyboard_page = None
if ( lang in (
    "sjd", "sje", "sma",
    "sme", "smj", "smn", "sms",
    ) and ( pagelang in ("eng", ) ) ):
    keyboard_page = "https://divvun.no/en/keyboards/index.html"
elif ( lang in (
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms",
    ) and ( pagelang in ("fin", ) ) ):
        keyboard_page = "https://divvun.no/fi/keyboards/index.html"
elif ( lang in (
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms",
    ) and ( pagelang in ("nob", ) ) ):
        keyboard_page = "https://divvun.no/no/keyboards/index.html"
elif ( lang in (
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms",
    ) and ( pagelang in ("rus", ) ) ):
        keyboard_page = "https://divvun.no/en/keyboards/index.html"
elif ( lang in (
    "sjd", "sje", "sma", "sme", "smj",
    "smn", "sms",
    ) and ( pagelang in ("sme", ) ) ):
        keyboard_page = "https://divvun.no/keyboards/index.html"
*/


// anders: the lang in check is all the same!
let keyboard_page = null
const lang_is_sami = [
    "sjd", "sje", "sma",
    "sme", "smj", "smn", "sms",
    ].includes(lang);
if (lang_is_sami) {
    const url_fmt = "https://divvun.no/{}keyboards/index.html"
    const langcode = {
        eng: "en/",
        fin: "fi/",
        nob: "no/",
        rus: "en/",
        sme: "",
    }[pagelang];
    keyboard_page = url_fmt.replace("{}", langcode);
}


let risten_page = null;
if (lang == "sma") {
    risten_page = "https://baakoe.org";
} else if (lang == "sme") {
    risten_page = "https://satni.org";
} else if (lang == "smj") {
    risten_page = "https://bahko.org";
}

let divvun_page = null;
if (lang == "sma") {
    divvun_page = "https://divvun.no/sma/index.html";
} else if (lang == "sme") {
    divvun_page = "https://divvun.no";
} else if (lang == "smj") {
    divvun_page = "https://divvun.no/smj/index.html";
}

let oahpa_page = null;
if (lang == "fkv") {
    oahpa_page = "https://oahpa.no/kveeni/";
} else if (lang == "myv") {
    oahpa_page = "https://oahpa.no/erzya/";
} else if (lang == "liv") {
    oahpa_page = "https://testing.oahpa.no/livokel/";
} else if (lang == "rus") {
    oahpa_page = "https://testing.oahpa.no/rusoahpa/";
//} else if (lang == "sjd") {
//    oahpa_page = "https://oahpa.no/kiilt/";
} else if (lang == "sma") {
    oahpa_page = "https://oahpa.no/aarjel/";
} else if (lang == "sme") {
    oahpa_page = "https://oahpa.no/davvi/";
} else if (lang == "smn") {
    oahpa_page = "https://oahpa.no/aanaar/";
} else if (lang == "sms") {
    oahpa_page = "https://oahpa.no/nuorti/";
}


let kursa_page = null;
if (lang == "sme") {
    kursa_page = "https://kursa.oahpa.no/";
} else if (lang == "sma") {
    kursa_page = "https://kuvsje.oahpa.no/";
}


let tts_page = null;
if (lang == "sme") {
    tts_page = "https://divvun.no/no/tale/tale.html";
}


let gielese_page = null;
if (lang == "sma") {
    gielese_page = "https://gielese.no/";
}


let visl_page = null;
if (lang == "sme") {
    visl_page = "https://beta.visl.sdu.dk/visl/" + lang;
}


let mt_page = null;
if (lang == "sme") {
    mt_page = "https://jorgal.uit.no/";
}


let mt2_page = null;
if (["sme", "smj", "smn"].includes(lang)) {
    mt2_page = "https://gtweb.uit.no/mt/";
}






// content !
/*
title = null
if (!lang) {
    // if no lang, use Giellatekno entity
    title = entities["gte"];
} else if (["nob", "rus"].includes(pagelang)) {
    title = LT + " ";
} else {
    title = " " + LT;
}
*/
