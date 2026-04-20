<script lang="ts">
    import { langname } from "$lib/langnames";
    import { FileUpload } from "@skeletonlabs/skeleton-svelte";
    import ErrorBox from "$components/ErrorBox.svelte";
    import { enhance } from "$app/forms";
    import { getLocale } from "$lib/paraglide/runtime";
    import { FileExclamationPoint, FileIcon } from "@lucide/svelte";
    import { formatBytes } from "$lib/utils.js";
    import { m } from "$lib/paraglide/messages.js";
    import { Listbox, useListCollection } from "@skeletonlabs/skeleton-svelte";

    let { form } = $props();
    const LANG_PAIRS: Record<string, string[]> = {
        fin: ["nob", "sme", "smn", "sms"],
        fkv: ["nob"],
        nob: ["fin", "fkv", "sme", "sma"],
        sma: ["nob"],
        sme: ["nob", "fin", "smn"],
        smn: ["fin", "sme"],
    };

    let files = $state<File[]>([]);
    let textarea_value = $state("");
    let fromValue = $state(["sme"]);
    let toValue = $state(["nob"]);

    let is_processing = $state(false);
    let textinput_disabled = $derived(files.length > 0);
    let fileinput_disabled = $derived(textarea_value !== "");
    let filesize_large = $derived(files.length !== 0 && files[0].size > 1024 * 1024); // File is larger than 1 MiB

    $effect(() => {
        if (form != null) {
            is_processing = false;
        }
    });

    // $effect(() => {
    //     console.log(filesize_large);
    // });

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

    function reset() {
        textarea_value = "";
        files = [];
        textinput_disabled = false;
        fileinput_disabled = false;
        form = null;
        is_processing = false;
    }
</script>

<svelte:head>
    <title>{m.unknownlemmas_title()} | Giellatekno Webpipeline</title>
</svelte:head>

<div class="mb-32 grid grid-cols-2 items-start gap-4">
    <form
        method="POST"
        use:enhance={({ formData }) => {
            // Removes rejected files, so they don't get uploaded
            formData.delete("documents");
            files.forEach((f) => formData.append("documents", f));
            formData.set("lang1", fromValue[0]);
            formData.set("lang2", toValue[0]);
            return async ({ update }) => {
                update({ reset: false });
            };
        }}
        enctype="multipart/form-data"
        class="card preset-filled-surface-100-900 border-surface-200-800 flex w-2xl flex-col gap-2 justify-self-center rounded-md border p-4 shadow-md"
        onsubmit={() => (is_processing = true)}
    >
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

        <h5 class="h5">{m.fileform_text()}:</h5>
        <p>
            {m.fileform_options()}
        </p>

        <FileUpload
            maxFiles={1}
            accept={["text/plain", ".docx"]}
            name="documents"
            onFileChange={(details) => {
                files = details.acceptedFiles;
            }}
            disabled={fileinput_disabled}
            acceptedFiles={files}
        >
            <FileUpload.Label>
                {m.fileform_format_warning()}
            </FileUpload.Label>
            <FileUpload.Dropzone class="bg-surface-50-950 border-surface-950-50 py-4">
                <FileIcon class="size-6" />
                <span>{m.fileform_choose_or_drag()}</span>
                <FileUpload.Trigger>{m.fileform_browse_files()}</FileUpload.Trigger>
                <FileUpload.HiddenInput />
            </FileUpload.Dropzone>
            <FileUpload.ItemGroup>
                <FileUpload.Context>
                    {#snippet children(fileUpload)}
                        {#each fileUpload().acceptedFiles as file (file.name)}
                            <FileUpload.Item {file}>
                                <FileUpload.ItemName>{file.name}</FileUpload.ItemName>
                                <FileUpload.ItemSizeText>
                                    {formatBytes(file.size)}
                                </FileUpload.ItemSizeText>
                                <FileUpload.ItemDeleteTrigger />
                            </FileUpload.Item>
                        {/each}
                    {/snippet}
                </FileUpload.Context>
            </FileUpload.ItemGroup>
        </FileUpload>
        {#if filesize_large}
            <div
                class="preset-filled-warning-500 card flex flex-row items-center gap-2 rounded-sm p-2"
            >
                <FileExclamationPoint />
                <p>
                    {m.fileform_large_file_warning()}
                </p>
            </div>
        {/if}

        <textarea
            class="form-textarea w-full rounded-md"
            bind:value={textarea_value}
            rows="10"
            name="text"
            placeholder={m.fileform_text_placeholder()}
            disabled={textinput_disabled}
        ></textarea>

        <div class="flex justify-between">
            <button
                class="btn preset-outlined-error-500 hover:preset-tonal-surface"
                onclick={reset}
                type="button"
                disabled={is_processing}
            >
                {m.fileform_clear()}
            </button>
            <button
                class="btn preset-filled-primary-500"
                type="submit"
                disabled={is_processing}
            >
                {is_processing ? m.fileform_processing() : m.fileform_submit()}
            </button>
        </div>
    </form>

    <div class="flex w-2xl flex-col gap-4 justify-self-center">
        <h3 class="h3">{m.fileform_results()}:</h3>
        {#if form}
            <div>
                {#if form.error}
                    <ErrorBox error={form.error} />
                {/if}

                {#if form.success}
                    {#if form.result === ""}
                        <p>
                            {m.unknownlemmas_no_results()}
                        </p>
                    {/if}
                    <pre>{form.result}</pre>
                {/if}
            </div>
        {:else}
            <p class="text-surface-950/80">
                {m.fileform_not_submitted()}
            </p>
        {/if}
    </div>
</div>
