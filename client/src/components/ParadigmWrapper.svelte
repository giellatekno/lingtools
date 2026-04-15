<script lang="ts">
    import { Dialog, Portal, Progress } from "@skeletonlabs/skeleton-svelte";
    import ParadigmTable from "./ParadigmTable.svelte";
    import ParadigmList from "$components/ParadigmList.svelte";
    import { m } from "$lib/paraglide/messages";
    import { paradigm_parser, type ParsedParadigm } from "$lib/parsers";
    import { getParadigmSchema } from "$lib/paradigms/registry";
    import { page } from "$app/state";
    import { resolve } from "$app/paths";
    import type { LanguageSchema } from "$lib/paradigms/paradigm_types";
    import { ChevronDown, ChevronUp } from "@lucide/svelte";

    interface Props {
        data: any;
        format: string;
        search: string;
    }

    let { data, format, search }: Props = $props();

    let lang = $derived(page.params.lang || "");

    const parsed_data = $derived(paradigm_parser(data));
    const paradigms = $derived(parsed_data.paradigms);
    const other_hits = $derived(parsed_data.other_hits);

    let schemaPromises = $derived(
        Promise.all(
            paradigms.map((paradigm) =>
                getParadigmSchema(lang, paradigm.pos, paradigm.subclass),
            ),
        ),
    );

    // Filter out hidden sections and add Ids for navigation
    function filterSchema(
        schema: LanguageSchema,
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

    // Reset value when paradigms changes
    let value = $derived(paradigms.length - paradigms.length);

    function onResultButtonClick(i: number) {
        value = i;
        const tables = document.getElementById("tables");
        if (tables) {
            tables.scrollIntoView({ behavior: "smooth" });
        }
    }

    // CSS for mobile navigation
    const animBackdrop =
        "transition transition-discrete opacity-0 starting:data-[state=open]:opacity-0 data-[state=open]:opacity-100";
    const animModal =
        "transition transition-discrete opacity-0 translate-y-full starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-full data-[state=open]:opacity-100 data-[state=open]:-translate-y-0";
</script>

<div class="grid w-full grid-cols-1 gap-8 lg:grid-cols-[auto_1fr_auto]">
    {#await schemaPromises}
        <div></div>
        <div class="flex w-full justify-center">
            <Progress class="w-fit items-center" value={null}>
                <Progress.Circle class="[--size:--spacing(12)]">
                    <Progress.CircleTrack />
                    <Progress.CircleRange />
                </Progress.Circle>
                <Progress.ValueText />
            </Progress>
        </div>
        <div></div>
    {:then schemas}
        {@const cur_schema = filterSchema(schemas[value], paradigms[value])}
        <div class="w-full place-self-center lg:w-64 lg:place-self-auto">
            {#if parsed_data}
                {@render results()}
            {/if}
        </div>
        <hr class="hr lg:hidden" />

        <div class="w-full scroll-mt-24" id="tables">
            {#if paradigms.length !== 0}
                {#if cur_schema && format === "table"}
                    <ParadigmTable schema={cur_schema} elem={paradigms[value]} />
                {:else}
                    <ParadigmList elem={paradigms[value]} />
                {/if}
            {/if}
        </div>

        <div class="w-64">
            {#if cur_schema && format == "table"}
                {@render desktop_nav(cur_schema)}
                {@render mobile_nav(cur_schema)}
            {/if}
        </div>
    {/await}
</div>

{#snippet results()}
    <div
        class="card lg:bg-surface-100-900 lg:border-surface-200-800 flex h-fit w-full flex-col items-center gap-2 p-2 lg:sticky lg:top-24 lg:shrink-0 lg:border lg:p-4 lg:shadow-sm"
    >
        <div class="flex w-max flex-col gap-4 text-sm">
            <div class="flex flex-col gap-2">
                <span class="font-bold uppercase opacity-80">
                    {m.paradigm_directhits()}:
                </span>
                {#each paradigms as paradigm_elem, i}
                    <button
                        onclick={() => (value = i)}
                        class="btn {value === i
                            ? 'preset-filled-primary-500'
                            : 'border-primary-200-800 hover:preset-tonal border'}"
                    >
                        {paradigm_elem.lemma}
                        ({paradigm_elem.pos}{paradigm_elem.subclass
                            ? "+" + paradigm_elem.subclass
                            : ""})
                    </button>
                {:else}
                    <span>{m.paradigm_noresults({ search })}</span>
                {/each}
            </div>
            {#if other_hits.length !== 0}
                <div class="flex flex-col gap-2">
                    <span class="font-bold uppercase opacity-80">
                        {m.paradigm_otherhits()}:
                    </span>
                    {#each other_hits as other_hit}
                        <a
                            class="btn border-secondary-200-800 hover:preset-tonal border text-center"
                            href={resolve(
                                `/${lang}/paradigm?word=${other_hit.lemma}&pos=${other_hit.pos}`,
                            )}
                        >
                            {other_hit.lemma}
                            ({other_hit.pos})
                        </a>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
{/snippet}

{#snippet desktop_nav(schema: LanguageSchema)}
    <nav
        class="card bg-surface-100-900 border-surface-200-800 sticky top-24 hidden h-fit w-64 shrink-0 border p-4 shadow-sm lg:block"
    >
        <div class="space-y-2 text-sm">
            <h3 class="text-base font-bold uppercase">{m.paradigm_jumpto()}</h3>
            <hr class="hr" />
            <ul class="space-y-4">
                {#each schema.sections as section}
                    {#if section.title}
                        <li>
                            <div>
                                <a
                                    href="#{section.sId}"
                                    class="anchor text-primary-500 my-1 block font-bold uppercase"
                                >
                                    {section.title()}
                                </a>
                                <ul
                                    class="border-surface-200 mb-2 ml-2 space-y-2 border-l-2 pl-1"
                                >
                                    {#each section.tables as table}
                                        {#if table.title}
                                            <li>
                                                <a
                                                    href="#{table.tId}"
                                                    class="anchor text-surface-700-300 my-1 block"
                                                >
                                                    {table.title()}
                                                </a>
                                            </li>
                                        {/if}
                                    {/each}
                                </ul>
                            </div>
                        </li>
                    {/if}
                {/each}
            </ul>
        </div>
    </nav>
{/snippet}

{#snippet mobile_nav(schema: LanguageSchema)}
    <div class="fixed right-6 bottom-6 z-40 lg:hidden">
        <Dialog>
            <Dialog.Trigger>
                <button class="btn-icon preset-filled-primary-500">
                    <ChevronUp class="size-5" />
                </button>
            </Dialog.Trigger>
            <Portal>
                <Dialog.Backdrop
                    class="bg-surface-50-950/50 fixed inset-0 z-50 transition transition-discrete {animBackdrop}"
                />
                <Dialog.Positioner class="fixed inset-0 z-50 flex justify-start">
                    <Dialog.Content
                        class="card bg-surface-100-900 fixed inset-x-0 bottom-0 max-h-3/5 w-full space-y-4 overflow-y-auto px-4 shadow-xl {animModal}"
                    >
                        <div class="flex h-full flex-col">
                            <div
                                class="bg-surface-100-900 sticky top-0 flex flex-col gap-2 pt-2"
                            >
                                <div class="flex items-center justify-between">
                                    <Dialog.Title class="text-2xl font-bold">
                                        {m.paradigm_jumpto()}
                                    </Dialog.Title>
                                    <Dialog.CloseTrigger class="btn-icon preset-tonal">
                                        <ChevronDown />
                                    </Dialog.CloseTrigger>
                                </div>

                                <hr class="hr" />
                            </div>
                            <div class="my-4 flex flex-col gap-2">
                                {#each schema.sections as section}
                                    {#if section.title}
                                        <div>
                                            <Dialog.Trigger>
                                                <a
                                                    href="#{section.sId}"
                                                    class="anchor text-primary-500 mb-2 block py-2 font-bold uppercase"
                                                >
                                                    {section.title()}
                                                </a>
                                            </Dialog.Trigger>
                                            <div
                                                class="border-surface-200 ml-4 flex flex-col space-y-2 border-l-2 pl-2"
                                            >
                                                {#each section.tables as table}
                                                    {#if table.title}
                                                        <Dialog.Trigger>
                                                            <a
                                                                href="#{table.tId}"
                                                                class="anchor text-surface-700-300 block w-full py-1 text-left text-sm"
                                                            >
                                                                {table.title()}
                                                            </a>
                                                        </Dialog.Trigger>
                                                    {/if}
                                                {/each}
                                            </div>
                                        </div>
                                    {/if}
                                {/each}
                            </div>
                        </div>
                    </Dialog.Content>
                </Dialog.Positioner>
            </Portal>
        </Dialog>
    </div>
{/snippet}
