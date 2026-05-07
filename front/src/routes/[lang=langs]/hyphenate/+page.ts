import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { hyphenate_parser } from "$lib/parsers";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("hyphenate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/hyphenate/${lang}/${q}?format=text`;
    let response;
    try {
        response = await fetch(backend_url);
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }

    const text = await response.text();
    if (!response.ok) {
        return { error: `Non-200 from API: ${text}` };
    }
    if (text.startsWith("Error:")) {
        return { error: text };
    }

    return { q, results: hyphenate_parser(text) };
};
