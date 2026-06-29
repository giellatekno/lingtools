import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { GenerateResponse } from "$lib/types";
import { api_fetch } from "$lib/api";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("generate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const api_path = `generate/${lang}/${q}`;
    try {
        const result = await api_fetch(api_path, fetch, GenerateResponse);
        return { q, result };
    } catch (error) {
        return { error };
    }
};
