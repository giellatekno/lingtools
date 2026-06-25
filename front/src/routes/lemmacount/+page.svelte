<script lang="ts">
    import { langname } from "@giellatekno/langnames";
    import { m } from "$lib/paraglide/messages.js";
    import { getLocale } from "$lib/paraglide/runtime";
    import { Listbox, useListCollection } from "@skeletonlabs/skeleton-svelte";
    import FileForm from "$components/FileForm.svelte";

    let { form } = $props();

    const LANGS = ["fin", "fit", "fkv", "nob", "sma", "sme", "smj", "smn", "sms"];
    let lang = $state(["nob"]);

    const collection = $derived(
        useListCollection({
            items: LANGS.map((lang) => ({
                label: langname(lang, getLocale()),
                value: lang,
            })),
        }),
    );
</script>

<svelte:head>
    <title>{m.lemmacount_title()} | Giellatekno Webpipeline</title>
</svelte:head>

<FileForm
    bind:form
    noResultsMessage={m.lemmacount_no_results}
    setFormData={(fd) => fd.set("lang", lang[0])}
>
    {#snippet langSelector()}
        <h5 class="h5">{m.fileform_choose_language()}:</h5>
        <div class="mb-4">
            <div class="w-fit">
                <Listbox
                    {collection}
                    value={lang}
                    onValueChange={(e) => (lang = e.value)}
                >
                    <Listbox.Content class="grid grid-cols-2 gap-x-4">
                        {#each collection.items as item}
                            <Listbox.Item
                                {item}
                                class="data-selected:preset-filled-primary-500 rounded pr-4"
                            >
                                <Listbox.ItemText>{item.label}</Listbox.ItemText>
                            </Listbox.Item>
                        {/each}
                    </Listbox.Content>
                </Listbox>
            </div>
        </div>
    {/snippet}
</FileForm>
