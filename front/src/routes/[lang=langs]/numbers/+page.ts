import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { numbers_parser } from "$lib/parsers";
import { api_fetch } from "$lib/api";
import { StringResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("numbers")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const q = url.searchParams.get("q");
    const method = url.searchParams.get("method");

    if (!q) {
        return {};
    }

    const method_map: Record<string, string> = { num: "numbers", time: "clock" };
    const api_method = method_map[method ?? ""] ?? method;

    const api_path = `numbers/${lang}/${q}?method=${api_method}&direction=digit2text`;
    try {
        const result = await api_fetch(api_path, fetch, StringResponse);
        return { q, method: method ?? "num", result: numbers_parser(result) };
    } catch (error) {
        return { error };
    }
};
