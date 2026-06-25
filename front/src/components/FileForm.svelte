<script lang="ts">
    import { enhance } from "$app/forms";
    import { m } from "$lib/paraglide/messages";
    import { formatBytes } from "$lib/utils";
    import type { MessageFunction } from "@inlang/paraglide-js";
    import { FileExclamationPoint, FileIcon } from "@lucide/svelte";
    import { FileUpload } from "@skeletonlabs/skeleton-svelte";
    import type { Snippet } from "svelte";
    import ErrorBox from "./ErrorBox.svelte";

    interface Props {
        form: any;
        noResultsMessage: MessageFunction;
        setFormData: (formData: FormData) => void;
        langSelector: Snippet;
    }

    let {
        form = $bindable(null),
        noResultsMessage,
        setFormData,
        langSelector,
    }: Props = $props();

    let files = $state<File[]>([]);
    let textarea_value = $state("");
    let is_processing = $state(false);
    let textinput_disabled = $derived(files.length > 0);
    let fileinput_disabled = $derived(textarea_value !== "");
    let filesize_large = $derived(files.length !== 0 && files[0].size > 1024 * 1024); // File is larger than 1 MiB

    $effect(() => {
        if (form != null) {
            is_processing = false;
        }
    });

    let formElement: HTMLFormElement;

    async function on_textarea_keydown(ev: KeyboardEvent) {
        if ((ev.metaKey || ev.ctrlKey) && ev.key === "Enter") {
            ev.preventDefault();
            if (!(textarea_value === "")) formElement.requestSubmit();
        }
    }

    function reset() {
        textarea_value = "";
        files = [];
        form = null;
        is_processing = false;
    }
</script>

<div class="mb-32 grid grid-cols-2 items-start gap-4">
    <form
        method="POST"
        bind:this={formElement}
        use:enhance={({ formData }) => {
            // Removes rejected files, so they don't get uploaded
            formData.delete("documents");
            files.forEach((f) => formData.append("documents", f));
            setFormData(formData);
            return async ({ update }) => {
                update({ reset: false });
            };
        }}
        enctype="multipart/form-data"
        class="card preset-filled-surface-50-950 border-surface-200-800 flex w-full max-w-2xl flex-col gap-2 justify-self-center rounded-lg border p-4 shadow-lg lg:p-6"
        onsubmit={() => (is_processing = true)}
    >
        {@render langSelector()}

        <h5 class="h5">{m.fileform_text()}:</h5>
        <p>{m.fileform_options()}</p>

        <FileUpload
            maxFiles={1}
            accept={["text/plain", ".docx"]}
            name="documents"
            onFileChange={(details) => (files = details.acceptedFiles)}
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
                <p>{m.fileform_large_file_warning()}</p>
            </div>
        {/if}

        <textarea
            class="form-textarea w-full rounded-md"
            bind:value={textarea_value}
            rows="10"
            name="text"
            placeholder={m.fileform_text_placeholder()}
            disabled={textinput_disabled}
            onkeydown={on_textarea_keydown}
        ></textarea>

        <div class="flex justify-between">
            <button
                class="btn preset-outlined-surface-500 hover:preset-tonal"
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
            {#if form.error}
                <ErrorBox error={form.error} />
            {/if}

            {#if form.success}
                {#if form.results && form.results.length === 0}
                    <p>{noResultsMessage()}</p>
                {/if}
                <div
                    class="card preset-filled-surface-50-950 border-surface-200-800 flex w-fit min-w-sm flex-col gap-2 rounded-lg border p-4 font-mono shadow-lg lg:p-6"
                >
                    {#each form.results as result}
                        <span>{result}</span>
                    {/each}
                </div>
            {/if}
        {:else}
            <p class="text-surface-950/80">{m.fileform_not_submitted()}</p>
        {/if}
    </div>
</div>
