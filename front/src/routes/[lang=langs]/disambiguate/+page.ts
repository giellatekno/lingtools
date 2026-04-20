import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { disambiguate_parser } from "$lib/parsers";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("disambiguate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const query_params = url.searchParams;
    const q = query_params.get("q");

    if (q === null || q === "") {
        return {};
    }
    const backend_url = `${env.PUBLIC_API_ROOT}/disambiguate/${lang}/${q}`;
    let response;
    try {
        response = await fetch(backend_url);
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }

    let text = await response.text();
    if (response.status !== 200) {
        return { error: `non-200 from API: ${text}` };
    }
    const parsed = disambiguate_parser(text);

    return { q: q, results: parsed };
};
