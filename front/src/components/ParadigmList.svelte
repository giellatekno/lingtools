<script lang="ts">
    import Table from "./Table.svelte";
    import type { ParsedParadigm } from "$lib/parsers";
    import { m } from "$lib/paraglide/messages";

    let { elem }: { elem: ParsedParadigm } = $props();
</script>

<div class="mx-auto flex w-fit justify-center">
    <Table>
        <thead>
            <tr>
                <th>{m.paradigm_tags()}</th>
                <th>{m.paradigm_wordform()}</th>
            </tr>
        </thead>
        <tbody>
            {#each elem.wordforms.entries() as [tags, wordforms]}
                <tr>
                    <td class="label header">
                        {elem.pos}+{elem.subclass ? elem.subclass + "+" : ""}{tags}
                    </td>
                    <td>
                        <div class="flex flex-col gap-1">
                            {#each wordforms as wordform}
                                <p
                                    class="text-surface-900-100 text-sm text-nowrap lg:text-base"
                                >
                                    {wordform}
                                </p>
                            {/each}
                        </div>
                    </td>
                </tr>
            {/each}
        </tbody>
    </Table>
</div>
