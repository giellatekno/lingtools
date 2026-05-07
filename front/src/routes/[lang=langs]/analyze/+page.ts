import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { AnalyzeResponse } from "$lib/types";

export const load: PageLoad = async ({ params, url, fetch }) => {
    if (!tools_for[params.lang].includes("analyze")) {
        error(404, "Not Found");
    }

    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const backend_route = `${env.PUBLIC_API_ROOT}/analyze/${lang}/${q}?format=json`;
    let response;
    try {
        response = await fetch(backend_route);
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

    try {
        return { q, parsed: AnalyzeResponse.parse(json) };
    } catch (e) {
        console.error(e);
        return { error: "Unexpected response format from API" };
    }
};
