import { env } from "$env/dynamic/public";
import { dependency_parser } from "$lib/parsers";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { tools_for } from "$lib/langs";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("dependency")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const query_params = url.searchParams;
    const q = query_params.get("q");

    if (q === null || q === "") {
        return {};
    }

    const backend_url = `${env.PUBLIC_API_ROOT}/dependency/${lang}/${q}`;
    try {
        const response = await fetch(backend_url);
        const text = await response.text();
        if (!response.ok) {
            return { error: `non-200 from API: ${text}` };
        }
        return { q: q, results: dependency_parser(text) };
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }
};
