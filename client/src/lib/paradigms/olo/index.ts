import verbNeg from "./verb_neg.jsonc?raw";
import verb from "./verb.jsonc?raw";
import noun from "./noun.jsonc?raw";
import adjOrd from "./adjective_ord.jsonc?raw";
import adj from "./adjective.jsonc?raw";
import numeral from "./numeral.jsonc?raw";
import pronDem from "./pronoun_dem.jsonc?raw";
import pronIndef from "./pronoun_indef.jsonc?raw";
import pronInterr from "./pronoun_interr.jsonc?raw";
import pronPers from "./pronoun_pers.jsonc?raw";
import pronQu from "./pronoun_qu.jsonc?raw";
import pronRecipr from "./pronoun_recipr.jsonc?raw";
import pronRefl from "./pronoun_refl.jsonc?raw";
import pronRel from "./pronoun_rel.jsonc?raw";

export const mapping: Record<string, Record<string, string>> = {
    V: { default: verb, Neg: verbNeg },
    N: { default: noun },
    A: { default: adj, Ord: adjOrd },
    Num: { default: numeral },
    Pron: {
        Dem: pronDem,
        Indef: pronIndef,
        Interr: pronInterr,
        Pers: pronPers,
        Qu: pronQu,
        Recipr: pronRecipr,
        Refl: pronRefl,
        Rel: pronRel,
    },
};

