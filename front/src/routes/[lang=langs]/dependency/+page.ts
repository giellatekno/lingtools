import { env } from "$env/dynamic/public";
import { dependency_parser } from "$lib/parsers";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { tools_for } from "$lib/langs";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("dependency")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/dependency/${lang}/${q}?format=text`;
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

    return { q, results: dependency_parser(text) };
};
