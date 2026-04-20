<script lang="ts">
    import type { PageData } from "./$types";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import TextForm from "$components/TextForm.svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "$lib/langnames";
    import { page } from "$app/state";

    interface Props {
        data: PageData;
    }

    let { data }: Props = $props();

    let lang = $derived(page.params.lang || "");

    let value = $derived(data.q || "");
</script>

<svelte:head>
    <title>
        {m.disambiguate_title()} • {langname(lang, getLocale())} • {m.page_title()}
    </title>
</svelte:head>

<div class="flex flex-col items-center gap-4">
    <h3 class="h4 lg:h3">{m.disambiguate_title()}</h3>
    <FormWrapper tool="disambiguate">
        <TextForm bind:value />
    </FormWrapper>

    <div>
        {#if data.error}
            <ErrorBox error={data.error} />
        {/if}

        {#if data.results}
            <div
                class="card border-primary-500 min-w-full overflow-x-auto border-2 px-6 py-4 whitespace-nowrap shadow-md lg:text-lg"
            >
                <div class="inline-block min-w-full whitespace-nowrap">
                    {#each data.results as result, i}
                        {#if i !== 0}
                            <hr class="hr" />
                        {/if}
                        <span class="">
                            <p class="text-red-800">
                                <b>{result.wordform}</b>
                            </p>
                            {#each result.analyses as analysis_group}
                                <div class="mb-2">
                                    {#each analysis_group as analysis, i}
                                        {@const tabs = "&emsp;".repeat(i)}
                                        <div class="flex flex-row gap-2">
                                            {@html tabs}
                                            <p class="text-red-700">
                                                [{analysis.lemma}]
                                            </p>
                                            <p>{analysis.verbtype}</p>
                                            <p class="text-blue-700">
                                                {analysis.tags}
                                            </p>
                                            <p class="text-green-700">
                                                {analysis.syntax}
                                            </p>
                                            <p>{analysis.relation}</p>
                                        </div>
                                    {/each}
                                </div>
                            {/each}
                        </span>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
</div>
