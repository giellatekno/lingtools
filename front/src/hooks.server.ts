import type { Handle } from "@sveltejs/kit";
import { paraglideMiddleware } from "$lib/paraglide/server";
import { sequence } from "@sveltejs/kit/hooks";

const handleParaglide: Handle = ({ event, resolve }) =>
    paraglideMiddleware(event.request, ({ request, locale }) => {
        event.request = request;

        return resolve(event, {
            transformPageChunk: ({ html }) => html.replace("%paraglide.lang%", locale),
        });
    });

const handleHeaders: Handle = async ({ event, resolve }) => {
    const response = await resolve(event);
    if (event.url.pathname.endsWith("/paradigm")) {
        response.headers.delete("Link");
    }
    return response;
};

export const handle: Handle = sequence(handleParaglide, handleHeaders);
