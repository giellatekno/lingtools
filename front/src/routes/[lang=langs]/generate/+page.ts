import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { GenerateResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("generate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/generate/${lang}/${q}`;
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

    try {
        return { q, parsed: GenerateResponse.parse(json) };
    } catch (e) {
        console.error(e);
        return { error: "Unexpected response format from API" };
    }
};
