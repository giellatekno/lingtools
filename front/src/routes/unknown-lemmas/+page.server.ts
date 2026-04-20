import type { Actions } from "./$types";
import { env } from "$env/dynamic/public";
import { fail } from "@sveltejs/kit";
import { gzipSync } from "zlib";

export const actions = {
    default: async ({ request, fetch }) => {
        const formData = await request.formData();
        console.debug(formData);

        // Extract form data
        const lang1 = formData.get("lang1") as string;
        const lang2 = formData.get("lang2") as string;
        const text = formData.get("text") as string;
        const files = formData.getAll("documents") as File[];

        if (!lang1 || !lang2) {
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
                lang1,
                lang2,
                typ: fileType,
                data: data,
            });
        } else if (text) {
            // Handle text input
            body = JSON.stringify({
                lang1,
                lang2,
                typ: "text",
                data: text,
            });
        } else {
            return fail(400, { error: "Either text input or file upload is required" });
        }

        // Call backend API
        const backendUrl = `${env.PUBLIC_API_ROOT}/unknown-lemmas-in-dict`;

        try {
            const response = await fetch(backendUrl, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body,
            });

            if (!response.ok) {
                const errorText = await response.text();
                return fail(response.status, {
                    error: `Backend error: ${response.status} - ${errorText}`,
                });
            }

            const result = await response.text();
            // console.log(result);
            return { success: true, result };
        } catch (error) {
            console.error("Backend communication failed:", error);
            return fail(500, {
                error: "Failed to communicate with backend service",
            });
        }
    },
} satisfies Actions;
