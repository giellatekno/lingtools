<script lang="ts">
    import type { ParsedAnalysis } from "$lib/parsers";

    let { results }: { results: ParsedAnalysis[] | undefined } = $props();
    // $effect(() => console.log(results));
</script>

<div class="w-full text-sm lg:w-fit lg:text-lg">
    {#if results}
        <div
            class="card border-surface-200-800 bg-surface-50-950 w-full overflow-x-auto rounded-lg border px-6 py-4 whitespace-nowrap shadow-lg"
        >
            {#each results as result, i}
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
    {/if}
</div>
