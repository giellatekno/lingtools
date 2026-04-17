<script lang="ts">
    import type { LanguageSchema, Table, Section } from "$lib/paradigms/paradigm_types";
    import type { ParsedParadigm } from "$lib/parsers";
    import { get_entry } from "$lib/paradigms/paradigm_utils";
    import TableComponent from "./Table.svelte";

    let { schema, elem }: { schema: LanguageSchema; elem: ParsedParadigm } = $props();

    function findActiveRows(section: Section) {
        return section.validateRows
            ? section.tables[0].rows.map((row) => {
                  const anchor = get_entry(row.tags[0], elem);
                  return anchor && anchor !== "—";
              })
            : null;
    }

    const uncoveredTags = $derived.by(() => {
        const covered = new Set<string>();
        for (const section of schema.sections) {
            for (const table of section.tables) {
                for (const row of table.rows) {
                    for (const tag of row.tags) {
                        covered.add(tag);
                    }
                }
            }
        }
        return Array.from(elem.wordforms.keys()).filter((tag) => !covered.has(tag));
    });
</script>

<div class="mb-8 grid w-full grid-cols-1 gap-8 lg:grid-cols-[1fr_auto]">
    <div class="flex w-full flex-col gap-8 overflow-x-auto">
        {#each schema.sections as section}
            {@const activeRows = findActiveRows(section)}
            <section
                id={section.sId}
                class="highlight-target flex w-full scroll-mt-24 flex-col rounded-lg lg:p-2"
            >
                {#if section.title}
                    <h4 class="h4 text-primary-500 mb-6 w-full border-b-2 pb-2 font-bold">
                        {section.title()}
                    </h4>
                {/if}

                <div class="flex w-full flex-col gap-4 lg:flex-row lg:flex-wrap lg:gap-8">
                    {#each section.tables as table}
                        <div
                            id={table.tId}
                            class="highlight-target h-fit scroll-mt-24 rounded-lg p-2 lg:p-4"
                        >
                            {#if table.title}
                                <h5 class="h5 mb-2 font-semibold italic opacity-70">
                                    {table.title()}
                                </h5>
                            {/if}

                            {@render table_constructor(table, activeRows)}
                        </div>
                    {/each}
                </div>
            </section>
        {/each}

        {#if uncoveredTags.length > 0}
            <details class="group">
                <summary
                    class="text-warning-600-400 flex cursor-pointer items-center gap-2 text-sm font-semibold select-none"
                >
                    <span>
                        {uncoveredTags.length} tag{uncoveredTags.length === 1 ? "" : "s"} not
                        shown in tables
                    </span>
                </summary>
                <div class="mt-2">
                    <TableComponent>
                        <thead>
                            <tr>
                                <th>Tags</th>
                                <th>Wordform(s)</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each uncoveredTags as tag}
                                <tr>
                                    <td class="label header">{tag}</td>
                                    <td>
                                        <div class="flex flex-col gap-1">
                                            {#each elem.wordforms.get(tag) ?? [] as wf}
                                                <p
                                                    class="text-surface-900-100 text-sm text-nowrap lg:text-base"
                                                >
                                                    {wf}
                                                </p>
                                            {/each}
                                        </div>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </TableComponent>
                </div>
            </details>
        {/if}
    </div>
</div>

{#snippet table_constructor(table: Table, activeRows: (boolean | "")[] | null)}
    <TableComponent>
        <thead>
            {#if table.headers.length > 0}
                <tr>
                    {#each table.headers as header}
                        <th>
                            {header()}
                        </th>
                    {/each}
                </tr>
            {/if}
        </thead>
        <tbody>
            {#each table.rows as row, rowIndex}
                {@const isRowDisabled = activeRows && !activeRows[rowIndex]}
                <tr class={row.separator ? "separator" : ""}>
                    <td class={table.headers.length === 0 ? "label header" : "label"}>
                        <span class="flex h-full items-center">
                            {#if typeof row.label === "string"}
                                {row.label}
                            {:else}
                                {row.label()}
                            {/if}
                        </span>
                    </td>

                    {#each row.tags as tag, i}
                        {@const cellEntries = get_entry(tag, elem)}
                        {@const centerCell =
                            row.colspan && row.colspan > 1 ? "items-center" : ""}
                        <td colspan={row.colspan || 1}>
                            {#if isRowDisabled}
                                <span>—</span>
                            {:else}
                                <div class="flex flex-col gap-1 {centerCell}">
                                    {#each cellEntries as cellEntry}
                                        <p
                                            class="text-surface-900-100 text-sm text-nowrap lg:text-base"
                                        >
                                            {#if row.prefixes && row.prefixes[i]}
                                                <span
                                                    class="text-secondary-600-400 italic"
                                                >
                                                    {row.prefixes[i]}
                                                </span>
                                            {/if}
                                            {cellEntry}
                                        </p>
                                    {/each}
                                </div>
                            {/if}
                        </td>
                    {/each}
                </tr>
            {/each}
        </tbody>
    </TableComponent>
{/snippet}
