import { dependency_parser } from "$lib/parsers";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { tools_for } from "$lib/langs";
import { api_fetch } from "$lib/api";
import { StringResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("dependency")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");

    if (!q) {
        return {};
    }

    const api_path = `dependency/${lang}/${q}?format=text`;
    try {
        const result = await api_fetch(api_path, fetch, StringResponse);
        return { q, result: dependency_parser(result) };
    } catch (error) {
        return { error };
    }
};
