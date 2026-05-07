import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("num")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");
    const method = url.searchParams.get("method");

    if (!q) {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/numbers/${lang}/${q}?method=${method}&format=json`;
    let response;
    try {
        response = await fetch(backend_url);
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }

    if (!response.ok) {
        return { error: `Non-200 from API: ${response.status}` };
    }

    let json;
    try {
        json = await response.json();
    } catch (e) {
        console.error(e);
        return { error: "API returned invalid JSON" };
    }

    if (json.error) {
        return { error: json.error };
    }

    // TODO: add Zod schema for numbers response
    return { q, results: json };
};
