import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("num")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const query_params = url.searchParams;
    const q = query_params.get("q");
    const method = query_params.get("method");

    if (q === null || q === "") {
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

    const text = await response.text();
    if (response.status !== 200) {
        return { error: `non-200 from API: ${text}` };
    }
    // TODO: Parse
    const parsed = text;

    return { q: q, results: parsed };
};
