import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { ParadigmResponse } from "$lib/types";

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
        return { word, pos };
    }

    const api_path = `paradigm/${lang}/${word}`;
    const api_url = `${env.PUBLIC_API_ROOT}/${api_path}?size=full&pos=${pos}&format=json`;

    let response;
    try {
        response = await fetch(api_url);
    } catch (e) {
        console.error(e);
        return { word, pos, error: "fetch() from API failed" };
    }

    if (response.status !== 200) {
        return { word, pos, error: `non-200 from API: ${response.status}` };
    }
    try {
        const json = await response.json();
        const parsed = ParadigmResponse.parse(json);
        return { pos, word, parsed };
    } catch (e) {
        return { word, pos, error: `Parsing JSON failed: ${e}` };
    }
};
