<script lang="ts">
    import type { PageData } from "./$types";
    import Table from "$components/Table.svelte";
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
        {m.hyphenate_title()} • {langname(lang, getLocale())} • {m.page_title()}
    </title>
</svelte:head>

<div class="flex flex-col items-center gap-4">
    <h3 class="h4 lg:h3">{m.hyphenate_title()}</h3>
    <FormWrapper tool="hyphenate">
        <TextForm bind:value />
    </FormWrapper>

    <div>
        {#if data.error}
            Error: {data.error}
        {/if}
        {#if data.results}
            <Table>
                <thead>
                    <tr>
                        <th>Lemma</th>
                        <th>Hyphenated</th>
                        <th>Score</th>
                    </tr>
                </thead>
                <tbody>
                    {#each data.results as { input_word, variations }, i}
                        {#each variations as { hyphenated_word, score }, j}
                            <tr
                                class:separate={i !== data.results.length - 1 &&
                                    j === variations.length - 1}
                            >
                                <td>{input_word}</td>
                                <td>
                                    {#each hyphenated_word.split("") as char}
                                        {#if ["#", "-", "^"].includes(char)}
                                            <span class="text-red-800">
                                                {char}
                                            </span>
                                        {:else}
                                            {char}
                                        {/if}
                                    {/each}
                                </td>
                                <td>{score}</td>
                            </tr>
                        {/each}
                    {/each}
                </tbody>
            </Table>
        {/if}
    </div>
</div>
