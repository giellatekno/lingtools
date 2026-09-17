import { env } from "$env/dynamic/public";
import { z } from "zod";

export async function api_fetch<T extends z.ZodTypeAny = z.ZodUnknown>(
    path: string,
    fetchFn: typeof fetch,
    schema: T,
): Promise<z.infer<T>> {
    const backend_url = `${env.PUBLIC_API_ROOT}/${path}`;
    let response;
    try {
        response = await fetchFn(backend_url);
    } catch (e) {
        console.error(e);
        throw Error("Failed to communicate with API");
    }

    if (!response.ok) {
        const errorText = await response.text();
        console.error(errorText);
        throw Error(`API returned ${response.status}: ${response.statusText}`);
    }

    if (schema instanceof z.ZodString) {
        let text;
        try {
            text = await response.text();
        } catch (e) {
            console.error(e);
            throw Error("Couldn't extract text from API response");
        }

        if (text.startsWith("Error:")) {
            throw Error(text);
        }
        try {
            return schema.parse(text) as z.infer<T>;
        } catch (e) {
            console.error(e);
            throw Error("Unexpected response format from API");
        }
    } else {
        let json;
        try {
            json = await response.json();
        } catch (e) {
            console.error(e);
            throw Error("API returned invalid JSON");
        }

        if (json.error) {
            throw Error(json.error);
        }

        try {
            return schema.parse(json) as z.infer<T>;
        } catch (e) {
            console.error(e);
            throw Error("Unexpected response format from API");
        }
    }
}

export async function api_post(
    path: string,
    body: string,
    fetchFn: typeof fetch,
    expectJson = true,
) {
    let response;
    try {
        response = await fetchFn(`${env.PUBLIC_API_ROOT}/${path}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body,
        });
    } catch (error) {
        console.error(error);
        throw Error("Failed to communicate with API");
    }

    if (!response.ok) {
        const errorText = await response.text();
        console.error(errorText);
        throw Error(`API returned ${response.status}: ${response.statusText}`);
    }

    if (expectJson) {
        let json;
        try {
            json = await response.json();
        } catch (e) {
            console.error(e);
            throw Error("API returned invalid JSON");
        }

        if (json.error) {
            throw Error(json.error);
        }
        return json;
    } else {
        let text;
        try {
            text = await response.text();
        } catch (e) {
            console.error(e);
            throw Error("Couldn't extract text from API response");
        }

        if (text.startsWith("Error:")) {
            throw Error(text);
        }
        return text;
    }
}
