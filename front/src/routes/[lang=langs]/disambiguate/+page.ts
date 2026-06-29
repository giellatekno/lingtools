import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { disambiguate_parser } from "$lib/parsers";
import { api_fetch } from "$lib/api";
import { StringResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("disambiguate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const api_path = `disambiguate/${lang}/${q}?format=text`;
    try {
        const result = await api_fetch(api_path, fetch, StringResponse);
        return { q, result: disambiguate_parser(result) };
    } catch (error) {
        return { error };
    }
};
