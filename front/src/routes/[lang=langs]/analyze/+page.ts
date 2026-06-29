import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { AnalyzeResponse } from "$lib/types";
import { api_fetch } from "$lib/api";

export const load: PageLoad = async ({ params, url, fetch }) => {
    if (!tools_for[params.lang].includes("analyze")) {
        error(404, "Not Found");
    }

    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const api_path = `analyze/${lang}/${q}?format=json`;
    try {
        const result = await api_fetch(api_path, fetch, AnalyzeResponse);
        return { q, result };
    } catch (error) {
        return { error };
    }
};
