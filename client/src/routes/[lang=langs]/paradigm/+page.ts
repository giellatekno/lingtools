import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { ParadigmResponse } from "$lib/types";
import { paradigm_parser } from "$lib/parsers";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("paradigm")) {
        error(404, "Not Found");
    }

    const lang = params.lang;
    const search_params = url.searchParams;
    const word = search_params.get("word")?.trim() || "";
    // const size = search_params.get("size") || "standard";
    const pos = search_params.get("pos") || "any";

    if (word.length == 0) {
        return {};
    }

    const api_path = `paradigm/${lang}/${word}`;
    const api_url = `${env.PUBLIC_API_ROOT}/${api_path}?size=full&pos=${pos}&format=json&libhfst=1`;

    let response;
    try {
        response = await fetch(api_url);
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }

    if (response.status !== 200) {
        return { error: `non-200 from API: ${response.status}` };
    }
    try {
        const json = await response.json();
        const parsed = ParadigmResponse.parse(json);
        return { pos, word, parsed };
    } catch (e) {
        return { error: `Parsing JSON failed: ${e}` };
    }
};
