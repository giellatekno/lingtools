<script lang="ts">
    import { enhance } from "$app/forms";
    import ErrorBox from "$components/ErrorBox.svelte";
    import { langname } from "$lib/langnames";
    import { m } from "$lib/paraglide/messages.js";
    import { getLocale } from "$lib/paraglide/runtime";
    import { FileIcon } from "@lucide/svelte";
    import { FileUpload } from "@skeletonlabs/skeleton-svelte";
    import { Listbox, useListCollection } from "@skeletonlabs/skeleton-svelte";

    let { form } = $props();

    const LANGS = ["fin", "fit", "fkv", "nob", "sma", "sme", "smj", "smn", "sms"];

    let files = $state<File[]>([]);
    let lang = $state(["nob"]);
    let textarea_value = $state("");
    let is_processing = $state(false);
    let textinput_disabled = $derived(files.length > 0);
    let fileinput_disabled = $derived(textarea_value !== "");

    $effect(() => {
        if (form != null) {
            is_processing = false;
        }
    });

    const collection = $derived(
        useListCollection({
            items: LANGS.map((lang) => ({
                label: langname(lang, getLocale()),
                value: lang,
            })),
        }),
    );

    // $effect(() => {
    //     console.log(form);
    // });

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
    <title>{m.lemmacount_title()} | Giellatekno Webpipeline</title>
</svelte:head>

<div class="mb-32 grid grid-cols-2 items-start gap-4">
    <form
        method="POST"
        use:enhance={({ formData }) => {
            // Removes rejected files, so they don't get uploaded
            formData.delete("documents");
            files.forEach((f) => formData.append("documents", f));
            formData.set("lang", lang[0]);
            return async ({ update }) => {
                update({ reset: false });
            };
        }}
        enctype="multipart/form-data"
        class="card preset-filled-surface-100-900 border-surface-200-800 flex w-2xl flex-col gap-2 justify-self-center rounded-md border p-4 shadow-md"
        onsubmit={() => (is_processing = true)}
    >
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
                                    {file.size} bytes
                                </FileUpload.ItemSizeText>
                                <FileUpload.ItemDeleteTrigger />
                            </FileUpload.Item>
                        {/each}
                    {/snippet}
                </FileUpload.Context>
            </FileUpload.ItemGroup>
        </FileUpload>

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
                            {m.lemmacount_no_results()}
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
