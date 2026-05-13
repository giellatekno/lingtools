import { browser } from "$app/environment";
import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { ParadigmResponse } from "$lib/types";

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

    const api_path = `paradigm/${lang}/${word}`;
    const api_url = `${env.PUBLIC_API_ROOT}/${api_path}?size=full&format=json`;

    let response;
    try {
        response = await fetch(api_url);
    } catch (e) {
        console.error(e);
        return { word, error: "fetch() from API failed" };
    }

    if (response.status !== 200) {
        return { word, error: `non-200 from API: ${response.status}` };
    }
    let json;
    try {
        json = await response.json();
    } catch (e) {
        console.error(e);
        return { word, error: "API returned invalid JSON" };
    }

    if (json.error) {
        return { word, error: json.error };
    }

    try {
        const parsed = ParadigmResponse.parse(json);
        return { word, parsed };
    } catch (e) {
        console.error(e);
        return { word, error: "Unexpected response format from API" };
    }
};
