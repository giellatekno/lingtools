<script lang="ts">
    import { langname } from "@giellatekno/langnames";
    import { getLocale } from "$lib/paraglide/runtime";
    import { m } from "$lib/paraglide/messages.js";
    import { Listbox, useListCollection } from "@skeletonlabs/skeleton-svelte";
    import FileForm from "$components/FileForm.svelte";

    let { form } = $props();
    const LANG_PAIRS: Record<string, string[]> = {
        fin: ["nob", "sme", "smn", "sms"],
        fkv: ["nob"],
        nob: ["fin", "fkv", "sme", "sma"],
        sma: ["nob"],
        sme: ["nob", "fin", "smn"],
        smn: ["fin", "sme"],
    };

    let fromValue = $state(["sme"]);
    let toValue = $state(["nob"]);

    const fromCollection = $derived(
        useListCollection({
            items: Object.keys(LANG_PAIRS).map((lang) => ({
                label: langname(lang, getLocale()),
                value: lang,
            })),
        }),
    );

    const toCollection = $derived(
        useListCollection({
            items:
                LANG_PAIRS[fromValue[0]]?.map((lang) => ({
                    label: langname(lang, getLocale()),
                    value: lang,
                })) ?? [],
        }),
    );

    $effect(() => {
        const validTargets = LANG_PAIRS[fromValue[0]];
        if (validTargets && !validTargets.includes(toValue[0])) {
            toValue = [validTargets[0]];
        }
    });
</script>

<svelte:head>
    <title>{m.unknownlemmas_title()} | Giellatekno Webpipeline</title>
</svelte:head>

<FileForm
    bind:form
    noResultsMessage={m.unknownlemmas_no_results}
    setFormData={(fd) => {
        fd.set("lang1", fromValue[0]);
        fd.set("lang2", toValue[0]);
    }}
>
    {#snippet langSelector()}
        <h5 class="h5">{m.fileform_choose_dictionary()}:</h5>
        <div class="mb-4 grid grid-cols-2 place-items-center gap-8">
            <div class="h-full w-full">
                <div class="label mb-1 font-bold">{m.fileform_from()}:</div>
                <div class="flex flex-col gap-2">
                    <Listbox
                        collection={fromCollection}
                        value={fromValue}
                        onValueChange={(e) => (fromValue = e.value)}
                    >
                        <Listbox.Content>
                            {#each fromCollection.items as item}
                                <Listbox.Item
                                    {item}
                                    class="data-selected:preset-filled-primary-500 rounded"
                                >
                                    <Listbox.ItemText>{item.label}</Listbox.ItemText>
                                </Listbox.Item>
                            {/each}
                        </Listbox.Content>
                    </Listbox>
                </div>
            </div>
            <div class="h-full w-full">
                <div class="label mb-1 font-bold">{m.fileform_to()}:</div>
                <div class="flex flex-col gap-2">
                    <Listbox
                        collection={toCollection}
                        value={toValue}
                        onValueChange={(e) => (toValue = e.value)}
                    >
                        <Listbox.Content>
                            {#each toCollection.items as item}
                                <Listbox.Item
                                    {item}
                                    class="data-selected:preset-filled-primary-500 rounded"
                                >
                                    <Listbox.ItemText>{item.label}</Listbox.ItemText>
                                </Listbox.Item>
                            {/each}
                        </Listbox.Content>
                    </Listbox>
                </div>
            </div>
        </div>
    {/snippet}
</FileForm>
