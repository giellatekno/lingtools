<script lang="ts">
    import { goto } from "$app/navigation";
    import { m } from "$lib/paraglide/messages";
    import { SearchIcon } from "@lucide/svelte";
    import { Switch } from "@skeletonlabs/skeleton-svelte";

    let { word, pos, format = $bindable(), has_tables } = $props();

    const poses = {
        any: m.partofspeech_any,
        N: m.partofspeech_noun,
        V: m.partofspeech_verb,
        A: m.partofspeech_adjective,
        Pron: m.partofspeech_pronoun,
        Num: m.partofspeech_numeral,
        // adverb: "Adv",
        // Pcle: "Pcle",
        // Po: "Po",
        // Pr: "Pr",
        //Adp: "Adp",
    } as const;

    let input: HTMLInputElement;

    async function on_radio_change() {
        if (word) {
            goto(`?word=${word}&pos=${pos}`);
            input.focus();
        }
    }

    async function on_submit(ev: SubmitEvent) {
        ev.preventDefault();
        goto(`?word=${word}&pos=${pos}`);
        input.focus();
    }
    let checked = $state(true);
    $effect(() => {
        format = checked ? "table" : "list";
    });
</script>

<form onsubmit={on_submit} id="form" class="mb-2 flex flex-col items-center gap-2">
    <span class="flex justify-center">
        <div class="flex flex-col">
            <div class="input-group grid-cols-[auto_1fr_auto] shadow-sm">
                <input
                    class="ig-input preset-filled-surface-50-950 h-12 text-lg"
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
        </div>
    </span>
    <div class="mt-4 flex w-full flex-col items-center gap-2">
        <div class="grid w-full grid-cols-2 gap-4">
            <label
                for="pos-select"
                class="label-text place-self-start self-center text-base lg:place-self-end"
            >
                {m.partofspeech()}:
            </label>
            <select
                class="select bg-surface-50-950 w-full place-self-start lg:w-fit"
                bind:value={pos}
                onchange={on_radio_change}
                name="pos"
                id="pos-select"
            >
                {#each Object.entries(poses) as [value, title]}
                    <option {value}>
                        {title()}
                    </option>
                {/each}
            </select>
        </div>

        <div class="grid w-full grid-cols-2 gap-4">
            {#if has_tables}
                <label
                    for="format-select"
                    class="label-text place-self-start self-center text-base lg:place-self-end"
                >
                    {m.paradigm_format()}:
                </label>
                <Switch
                    class="place-self-center lg:place-self-start"
                    {checked}
                    onCheckedChange={(details) => (checked = details.checked)}
                >
                    <Switch.Control>
                        <Switch.Thumb />
                    </Switch.Control>
                    <Switch.HiddenInput />
                </Switch>
            {/if}
        </div>
    </div>
</form>
