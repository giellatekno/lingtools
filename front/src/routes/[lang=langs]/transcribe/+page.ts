import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { transcribe_parser } from "$lib/parsers";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("transcribe")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/transcribe/${lang}/${q}?format=text`;
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

    return { q, results: transcribe_parser(text) };
};
