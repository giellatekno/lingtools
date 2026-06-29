<script lang="ts">
    import type { PageData } from "./$types";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import TextForm from "$components/TextForm.svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "@giellatekno/langnames";
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
        {m.transcribe_title()} • {langname(lang, getLocale())} • {m.page_title()}
    </title>
</svelte:head>

<div class="flex flex-col items-center gap-4 lg:gap-8">
    <FormWrapper tool="transcribe">
        <TextForm bind:value />
    </FormWrapper>

    {#if data.error}
        <ErrorBox error={data.error} />
    {/if}

    {#if data.result}
        <div
            class="card border-surface-200-800 bg-surface-50-950 w-fit rounded-lg border-2 px-4 py-6 text-wrap shadow-lg lg:min-w-sm"
        >
            <div class="flex flex-col gap-2 text-xl">
                {#each data.result as result}
                    <p>
                        {result}
                    </p>
                {:else}
                    [l6e] No analyses
                {/each}
            </div>
        </div>
    {/if}
</div>
