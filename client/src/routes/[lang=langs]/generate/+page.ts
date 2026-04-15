import { env } from "$env/dynamic/public";
import { tools_for } from "$lib/langs";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { generate_parser } from "$lib/parsers";
import { GenerateResponse } from "$lib/types";

export const load: PageLoad = async ({ url, params, fetch }) => {
    if (!tools_for[params.lang].includes("generate")) {
        error(404, "Not Found");
    }
    const lang = params.lang;
    const query_params = url.searchParams;
    const q = query_params.get("q");

    if (q === null || q === "") {
        return {};
    }

    console.log(q);
    const backend_url = `${env.PUBLIC_API_ROOT}/generate/${lang}/${q}`;
    let response;
    try {
        response = await fetch(backend_url);
    } catch (e) {
        console.error(e);
        return { error: "fetch() from API failed" };
    }

    const text = await response.text();
    if (response.status !== 200) {
        return { error: `non-200 from API: ${text}` };
    }
    try {
        const parsed = GenerateResponse.parse(JSON.parse(text));
        return { q, parsed };
    } catch (e) {
        return { error: `Parsing JSON failed: ${e}` };
    }
};
