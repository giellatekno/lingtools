<script lang="ts">
    import { goto } from "$app/navigation";
    import { m } from "$lib/paraglide/messages";
    import { SearchIcon } from "@lucide/svelte";
    import { Switch } from "@skeletonlabs/skeleton-svelte";

    let { word, format = $bindable(), has_tables } = $props();

    let input: HTMLInputElement;

    async function on_submit(ev: SubmitEvent) {
        ev.preventDefault();
        goto(`?word=${word}`);
        input.focus();
    }
    let checked = $state(true);
    $effect(() => {
        format = checked ? "table" : "list";
    });
</script>

<form onsubmit={on_submit} id="form" class="flex flex-col items-center gap-4">
    <div class="input-group w-full grid-cols-[1fr_auto]">
        <input
            class="ig-input h-12 text-lg"
            id="input"
            type="search"
            name="word"
            bind:value={word}
            bind:this={input}
            autocapitalize="off"
            spellcheck="false"
            placeholder={m.search() + "..."}
        />
        <button class="ig-btn preset-filled-primary-500" type="submit">
            <SearchIcon />
        </button>
    </div>

    {#if has_tables}
        <div class="flex items-center gap-2">
            <span class="label-text text-sm">{m.paradigm_format()}:</span>
            <Switch {checked} onCheckedChange={(details) => (checked = details.checked)}>
                <Switch.Control>
                    <Switch.Thumb />
                </Switch.Control>
                <Switch.HiddenInput />
            </Switch>
        </div>
    {/if}
</form>
