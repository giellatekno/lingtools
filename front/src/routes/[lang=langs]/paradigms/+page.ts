import { browser } from "$app/environment";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { ParadigmResponse } from "$lib/types";
import { api_fetch } from "$lib/api";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("paradigms")) {
        error(404, "Not Found");
    }

    const lang = params.lang;
    const search_params = url.searchParams;
    const word = search_params.get("word")?.trim() || "";

    if (!browser || word.length == 0) {
        return { word };
    }

    const api_path = `paradigm/${lang}/${word}?size=full&format=json`;

    try {
        const result = await api_fetch(api_path, fetch, ParadigmResponse);
        return { word, result };
    } catch (error) {
        return { word, error };
    }
};
