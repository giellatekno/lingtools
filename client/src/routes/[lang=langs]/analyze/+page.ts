import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { analyze_parser } from "$lib/parsers";
import { AnalyzeResponse } from "$lib/types";

export const load: PageLoad = async ({ params, url, fetch }) => {
    if (!tools_for[params.lang].includes("analyze")) {
        error(404, "Not Found");
    }

    const lang = params.lang;
    const query_params = url.searchParams;
    const q = query_params.get("q");

    if (!q) {
        return {};
    }

    const backend_route = `${env.PUBLIC_API_ROOT}/analyze/${lang}/${q}?format=json`;
    try {
        const response = await fetch(backend_route);
        const text = await response.text();
        if (!response.ok) {
            return { error: `Non-200 from API: ${text}` };
        }
        try {
            const json_data = JSON.parse(text);
            return { q: q, parsed: AnalyzeResponse.parse(json_data) };
        } catch (e) {
            return { error: `Parsing JSON failed: ${e}` };
        }
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }
};
