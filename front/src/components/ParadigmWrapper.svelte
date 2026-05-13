<script lang="ts">
    import ParadigmTable from "./ParadigmTable.svelte";
    import ParadigmList from "$components/ParadigmList.svelte";
    import { m } from "$lib/paraglide/messages";
    import { paradigm_parser, type ParsedParadigm } from "$lib/parsers";
    import { getParadigmSchema } from "$lib/paradigms/registry";
    import { page } from "$app/state";
    import { resolve } from "$app/paths";
    import { goto } from "$app/navigation";
    import type { LanguageSchema } from "$lib/paradigms/paradigm_types";
    import { Accordion } from "@skeletonlabs/skeleton-svelte";
    import { ChevronDownIcon, ChevronUpIcon } from "@lucide/svelte";

    interface Props {
        data: any;
        format: string;
        search: string;
    }

    let { data, format, search }: Props = $props();

    let lang = $derived(page.params.lang || "");

    const pos_names: Record<string, () => string> = {
        N: m.partofspeech_noun,
        V: m.partofspeech_verb,
        A: m.partofspeech_adjective,
        Pron: m.partofspeech_pronoun,
        Num: m.partofspeech_numeral,
    };

    const pos_order = Object.keys(pos_names);
    const pos_rank = (pos: string) => {
        const i = pos_order.indexOf(pos);
        return i === -1 ? pos_order.length : i;
    };
    const by_pos_subclass = <T extends { pos: string; subclass: string }>(a: T, b: T) =>
        pos_rank(a.pos) - pos_rank(b.pos) || a.subclass.localeCompare(b.subclass);

    const parsed_data = $derived(paradigm_parser(data));
    const paradigms = $derived([...parsed_data.paradigms].sort(by_pos_subclass));
    const other_hits = $derived([...parsed_data.other_hits].sort(by_pos_subclass));

    let schemaPromises = $derived(
        Promise.all(
            paradigms.map((paradigm) =>
                getParadigmSchema(lang, paradigm.pos, paradigm.subclass),
            ),
        ),
    );

    function filterSchema(
        schema: LanguageSchema | null,
        elem: ParsedParadigm,
    ): LanguageSchema | undefined {
        if (!schema || !elem) return undefined;
        schema.sections = schema.sections
            .filter((s) => !s.showIf || s.showIf(elem))
            .map((s, sIndex) => ({
                ...s,
                sId: `section-${sIndex}`,
                tables: s.tables
                    .filter((t) => !t.showIf || t.showIf(elem))
                    .map((t, tIndex) => ({
                        ...t,
                        tId: `table-${sIndex}-${tIndex}`,
                    })),
            }));
        return schema;
    }

    const url_word = $derived(page.url.searchParams.get("word") || "");
    const url_pos = $derived(page.url.searchParams.get("pos") || "");
    const url_subclass = $derived(page.url.searchParams.get("subclass") || "");

    const value = $derived.by(() => {
        if (!url_pos) return 0;
        const idx = paradigms.findIndex(
            (p) => p.pos === url_pos && p.subclass === url_subclass,
        );
        return idx >= 0 ? idx : 0;
    });
</script>

<div class="grid w-full grid-cols-1 gap-8 lg:grid-cols-[16rem_1fr_16rem]">
    {#await schemaPromises then schemas}
        {@const cur_schema = filterSchema(schemas[value], paradigms[value])}

        <div class="w-full place-self-center lg:place-self-auto">
            {#if parsed_data}
                {@render results()}
            {/if}
        </div>

        <div class="flex w-full flex-col items-center">
            {#if paradigms.length === 0 && other_hits.length === 0 && search !== ""}
                <div class="mt-4 text-lg">
                    {m.paradigm_noresults({ search })}
                </div>
            {/if}

            {#if paradigms.length !== 0}
                {@const cur = paradigms[value]}
                <div class="w-full lg:w-fit">
                    <div
                        class="mx-2 mb-2 flex scroll-mt-24 items-baseline gap-4"
                        id="top"
                    >
                        <h2 class="h2">{cur.lemma}</h2>
                        <span class="text-lg opacity-80">
                            {pos_names[cur.pos]?.() ?? cur.pos}{cur.subclass
                                ? ` (${cur.subclass})`
                                : ""}
                        </span>
                    </div>

                    {#if cur_schema && format === "table"}
                        <div class="lg:hidden">
                            {@render accordion_nav(cur_schema)}
                        </div>
                    {/if}

                    {#if cur_schema && format === "table"}
                        <div
                            class="card border-surface-200-800 bg-surface-50-950 mb-8 w-full rounded-lg border p-2 shadow-lg"
                        >
                            <ParadigmTable schema={cur_schema} elem={cur} />
                        </div>
                    {:else}
                        <ParadigmList elem={cur} />
                    {/if}
                </div>
            {/if}
        </div>

        <div class="hidden lg:block">
            {#if cur_schema && format === "table" && paradigms.length !== 0}
                {@render sticky_nav(cur_schema)}
            {/if}
        </div>
    {/await}
</div>

{#snippet results()}
    <div
        class="card bg-surface-50-950 border-surface-200-800 flex h-fit w-full flex-col gap-2 rounded-lg p-2 shadow-lg lg:sticky lg:top-24 lg:shrink-0 lg:border lg:p-4"
    >
        <div class="flex flex-col gap-4 text-sm">
            <div class="flex flex-col gap-1">
                <span class="font-bold uppercase opacity-80">
                    {m.paradigm_directhits()}:
                </span>
                {#each paradigms as paradigm_elem, i}
                    <button
                        onclick={() =>
                            goto(
                                `?word=${url_word}&pos=${paradigm_elem.pos}${paradigm_elem.subclass ? `&subclass=${paradigm_elem.subclass}` : ""}`,
                            )}
                        class="btn flex-col items-start gap-0 px-3 py-2 text-left {value ===
                        i
                            ? 'border-primary-500 border'
                            : 'hover:preset-tonal'}"
                    >
                        <span class="font-semibold">{paradigm_elem.lemma}</span>
                        <span class="text-surface-500 text-xs">
                            {pos_names[paradigm_elem.pos]?.() ??
                                paradigm_elem.pos}{paradigm_elem.subclass
                                ? ` · ${paradigm_elem.subclass}`
                                : ""}
                        </span>
                    </button>
                {:else}
                    {m.paradigm_nodirecthits()}
                {/each}
            </div>
            {#if other_hits.length !== 0}
                <div class="flex flex-col gap-1">
                    <span class="font-bold uppercase opacity-80">
                        {m.paradigm_otherhits()}:
                    </span>
                    {#each other_hits as other_hit}
                        <a
                            class="btn hover:preset-tonal flex-col items-start gap-0 px-3 py-2 text-left"
                            href={resolve(
                                `/${lang}/paradigms?word=${other_hit.lemma}&pos=${other_hit.pos}${other_hit.subclass ? `&subclass=${other_hit.subclass}` : ""}`,
                            )}
                        >
                            <span class="font-semibold">{other_hit.lemma}</span>
                            <span class="text-surface-500 text-xs">
                                {pos_names[other_hit.pos]?.() ??
                                    other_hit.pos}{other_hit.subclass
                                    ? ` · ${other_hit.subclass}`
                                    : ""}
                            </span>
                        </a>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
{/snippet}

{#snippet sticky_nav(schema: LanguageSchema)}
    {#if !schema.sections.every((s) => !s.title)}
        <nav
            class="card bg-surface-50-950 border-surface-200-800 sticky top-24 h-fit w-full rounded-lg border p-4 shadow-lg"
        >
            <div class="space-y-3 text-sm">
                <h3 class="font-bold uppercase">{m.paradigm_jumpto()}</h3>
                <hr class="hr" />
                <ul class="space-y-3">
                    {#each schema.sections as section}
                        {#if section.title}
                            <li>
                                <a
                                    href="#{section.sId}"
                                    class="anchor text-primary-500 block font-semibold"
                                >
                                    {section.title()}
                                </a>
                                <ul
                                    class="border-surface-200-800 mt-1 ml-2 space-y-1 border-l-2 pl-2"
                                >
                                    {#each section.tables as table}
                                        {#if table.title}
                                            <li>
                                                <a
                                                    href="#{table.tId}"
                                                    class="anchor text-surface-600-400 block"
                                                >
                                                    {table.title()}
                                                </a>
                                            </li>
                                        {/if}
                                    {/each}
                                </ul>
                            </li>
                        {/if}
                    {/each}
                </ul>
            </div>
        </nav>
    {/if}
{/snippet}

{#snippet accordion_nav(schema: LanguageSchema)}
    {#if !schema.sections.every((s) => !s.title)}
        <div
            class="border-surface-200-800 bg-surface-50-950 mb-3 overflow-hidden rounded-lg border shadow-lg"
        >
            <Accordion collapsible>
                <Accordion.Item value="nav">
                    <Accordion.ItemTrigger
                        class="border-surface-200-800 flex min-h-12 items-center justify-between font-bold uppercase data-[state=open]:border-b"
                    >
                        {m.paradigm_jumpto()}
                        <Accordion.ItemIndicator class="group">
                            <ChevronDownIcon
                                class="block size-5 group-data-[state=open]:hidden"
                            />
                            <ChevronUpIcon
                                class="hidden size-5 group-data-[state=open]:block"
                            />
                        </Accordion.ItemIndicator>
                    </Accordion.ItemTrigger>
                    <Accordion.ItemContent>
                        {#snippet element(attributes)}
                            <div {...attributes} class="flex flex-col px-2 pb-2">
                                {#each schema.sections as section}
                                    {#if section.title}
                                        <div class="flex flex-col">
                                            <a
                                                href="#{section.sId}"
                                                class="anchor text-primary-500 flex min-h-11 items-center px-2 font-semibold"
                                            >
                                                {section.title()}
                                            </a>
                                            <div class="flex flex-col pl-4">
                                                {#each section.tables as table}
                                                    {#if table.title}
                                                        <a
                                                            href="#{table.tId}"
                                                            class="anchor text-surface-600-400 flex min-h-11 items-center px-2"
                                                        >
                                                            {table.title()}
                                                        </a>
                                                    {/if}
                                                {/each}
                                            </div>
                                        </div>
                                    {/if}
                                {/each}
                            </div>
                        {/snippet}
                    </Accordion.ItemContent>
                </Accordion.Item>
            </Accordion>
        </div>
    {/if}
{/snippet}
