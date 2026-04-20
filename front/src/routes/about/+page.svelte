<script>
    import { m } from "$lib/paraglide/messages";
</script>

<svelte:head>
    <title>{m.about()} • {m.page_title()}</title>
</svelte:head>

<div class="flex justify-center">
    <div
        class="border-primary-500 my-4 flex w-full max-w-2xl flex-col gap-8 rounded-lg border-2 p-4 shadow-lg lg:text-lg"
    >
        <div class="flex w-full flex-col gap-1">
            <h4 class="h5 lg:h4">{m.about_title()}</h4>
            <p>
                {m.about_body()}
            </p>
        </div>
        <div class="flex w-full flex-col gap-2">
            <h4 class="h5 lg:h4">{m.fordevelopers()}</h4>

            <span>
                <h5 class="h6 lg:h5">API:</h5>
                <span class="flex flex-col gap-2">
                    <p>
                        {m.api_paragraph()}
                    </p>
                    <a
                        class="text-primary-700-300 hover:underline"
                        rel="external"
                        href="https://gtweb.uit.no/lingtools-api/"
                        target="_blank"
                    >
                        gtweb.uit.no/lingtools-api/
                    </a>
                </span>
            </span>
            <span>
                <h5 class="h6 lg:h5 mt-2">{m.sourcecode()}:</h5>
                <span class="flex flex-col gap-2">
                    <p>
                        {m.developers_code_paragraph()}
                    </p>
                    <a
                        class="text-primary-700-300 hover:underline"
                        rel="external"
                        href="https://github.com/giellatekno/lingtools"
                        target="_blank"
                    >
                        github.com/giellatekno/lingtools
                    </a>
                    <p>{m.developers_code_paragraph2()}</p>
                </span>
            </span>
        </div>
    </div>
</div>
