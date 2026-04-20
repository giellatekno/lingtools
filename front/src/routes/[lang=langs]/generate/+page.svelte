<script lang="ts">
    import type { PageData } from "./$types";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import TextForm from "$components/TextForm.svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "$lib/langnames";
    import { page } from "$app/state";
    import Table from "$components/Table.svelte";

    interface Props {
        data: PageData;
    }

    let { data }: Props = $props();

    let lang = $derived(page.params.lang || "");

    // the search text
    let value = $derived(data.q || "");
</script>

<svelte:head>
    <title>{m.generate_title()} • {langname(lang, getLocale())} • {m.page_title()}</title>
</svelte:head>

<div class="flex flex-col items-center gap-4">
    <h3 class="h4 lg:h3">{m.generate_title()}</h3>
    <FormWrapper tool="generate">
        <TextForm bind:value rows={2} />
    </FormWrapper>
    <div>
        {#if data.error}
            <ErrorBox error={data.error} />
        {/if}

        {#if data.parsed && data.parsed.results.length > 0}
            <Table>
                <thead>
                    <tr>
                        <th>{m.analysis()}</th>
                        <th>{m.wordform()}</th>
                    </tr>
                </thead>
                <tbody>
                    {#each data.parsed.results as { analysis, wordforms }}
                        <tr>
                            <td>{analysis}</td>
                            <td>{wordforms.join(", ")}</td>
                        </tr>
                    {/each}
                </tbody>
            </Table>
        {:else}
            {m.noresults()}
        {/if}
    </div>
</div>
