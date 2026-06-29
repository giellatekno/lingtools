import { tools_for } from "$lib/langs";
import { hyphenate_parser } from "$lib/parsers";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { api_fetch } from "$lib/api";
import { StringResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("hyphenate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const api_path = `hyphenate/${lang}/${q}?format=text`;
    try {
        const result = await api_fetch(api_path, fetch, StringResponse);
        return { q, result: hyphenate_parser(result) };
    } catch (error) {
        return { error };
    }
};
