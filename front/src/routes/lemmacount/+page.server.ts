import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { gzipSync } from "zlib";
import { api_post } from "$lib/api";

export const actions = {
    default: async ({ request, fetch }) => {
        const formData = await request.formData();

        // Extract form data
        const lang = formData.get("lang") as string;
        const text = formData.get("text") as string;
        const files = formData.getAll("documents") as File[];

        // Validate required fields
        if (!lang) {
            return fail(400, { error: "Language selection is required" });
        }

        let body;
        if (files.length !== 0) {
            // Handle file input
            const file = files[0];
            const buffer = Buffer.from(await file.arrayBuffer());

            let data: string;
            let fileType: string;

            if (
                file.type ===
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            ) {
                fileType = "docx";
                data = buffer.toString("base64");
            } else {
                fileType = "text+gz+b64";
                const compressed = gzipSync(buffer);
                data = compressed.toString("base64");
            }
            body = JSON.stringify({
                lang,
                typ: fileType,
                data: data,
            });
        } else if (text) {
            // Handle text input
            body = JSON.stringify({
                lang,
                typ: "text",
                data: text,
            });
        } else {
            return fail(400, { error: "Either text input or file upload is required" });
        }

        try {
            // TODO: json response
            const result = await api_post("lemma-count", body, fetch, false);
            return { success: true, results: result.split("\n") };
        } catch (error) {
            return fail(500, { error });
        }
    },
} satisfies Actions;
