import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";
import { numbers_parser } from "$lib/parsers";

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

    const backend_url = `${env.PUBLIC_API_ROOT}/numbers/${lang}/${q}?method=${api_method}&direction=digit2text`;
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

    return { q, method: method ?? "num", results: numbers_parser(json) };
};
