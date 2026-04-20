<script lang="ts">
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";
    import { m } from "$lib/paraglide/messages";

    let { value = $bindable(), rows = 6 } = $props();

    let textarea: HTMLTextAreaElement;

    const isMac = $derived.by(() => {
        const ua = browser ? window.navigator.userAgent : "";
        return ua ? ua.includes("Mac") : false;
    });
    const ctrl = $derived(isMac ? "⌘" : "Ctrl");

    async function on_textarea_keydown(ev: KeyboardEvent) {
        if ((ev.metaKey || ev.ctrlKey) && ev.key === "Enter") {
            ev.preventDefault();
            await goto(`?q=${encodeURIComponent(value)}`, { keepFocus: true });
            textarea.focus();
        }
    }

    async function on_submit(ev: SubmitEvent) {
        ev.preventDefault();
        await goto(`?q=${encodeURIComponent(value)}`, { keepFocus: true });
        textarea.focus();
    }

    function on_clear(ev: MouseEvent) {
        ev.preventDefault();
        value = "";
        textarea.focus();
    }
</script>

<form class="flex flex-col gap-2" onsubmit={on_submit}>
    <textarea
        class="form-textarea w-full rounded-sm"
        {rows}
        bind:this={textarea}
        bind:value
        name="q"
        onkeydown={on_textarea_keydown}
        placeholder={m.writehere()}
    ></textarea>
    <div class="flex flex-row items-center justify-between gap-2 lg:justify-start">
        <button
            class="btn preset-outlined-error-600-400 hover:preset-tonal-error"
            type="button"
            onclick={on_clear}
        >
            {m.clear()}
        </button>
        <button class="btn preset-filled-primary-500" type="submit">
            {m.submit()}
        </button>
        <span class="hidden items-center gap-1 opacity-70 lg:flex">
            {m.submit_keys_1()}
            <kbd class="kbd preset-filled-surface-200-800">{ctrl}</kbd>
            +
            <kbd class="kbd preset-filled-surface-200-800">↵</kbd>
            {m.submit_keys_2()}
        </span>
    </div>
</form>
