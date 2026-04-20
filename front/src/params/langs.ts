import type { ParamMatcher } from "@sveltejs/kit";
import { langs } from "$lib/langs";

export const match: ParamMatcher = (param: string) => {
    return langs.includes(param);
};
