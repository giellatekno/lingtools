<script lang="ts">
    import type { PageData } from "./$types";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "@giellatekno/langnames";
    import { page } from "$app/state";
    import { goto } from "$app/navigation";
    import { num_lang_details } from "$lib/langs";

    interface Props {
        data: PageData;
    }

    let { data }: Props = $props();

    let lang = $derived(page.params.lang || "");
    let methods = $derived(num_lang_details[lang] ?? ["numbers"]);

    let method = $derived(
        data.method && (methods as string[]).includes(data.method)
            ? data.method
            : "numbers",
    );

    let num_value = $derived(data.q || "");
    let clock_h = $state("");
    let clock_m = $state("");
    let date_d = $state("");
    let date_m = $state("");

    const method_labels = {
        numbers: m.numbers_method_numbers,
        clock: m.numbers_method_clock,
        date: m.numbers_method_date,
    };

    function format_q(): string {
        if (method === "clock") {
            return `${String(clock_h).padStart(2, "0")}:${String(clock_m).padStart(2, "0")}`;
        }
        if (method === "date") {
            return `${date_d}.${date_m}.`;
        }
        return num_value;
    }

    async function on_submit(ev: SubmitEvent) {
        ev.preventDefault();
        const q = format_q();
        await goto(`?q=${encodeURIComponent(q)}&method=${method}`, {
            keepFocus: true,
        });
    }

    function on_clear(ev: MouseEvent) {
        ev.preventDefault();
        num_value = "";
        clock_h = "";
        clock_m = "";
        date_d = "";
        date_m = "";
    }

    function select_method(opt: string) {
        method = opt;
        on_clear(new MouseEvent("click"));
    }
</script>

<svelte:head>
    <title>
        {m.numbers_title()} • {langname(lang, getLocale())} • {m.page_title()}
    </title>
</svelte:head>

<div class="flex flex-col items-center gap-4 lg:gap-8">
    <FormWrapper tool="numbers">
        {#if methods.length > 1}
            <div class="mb-4 flex gap-2">
                {#each methods as opt}
                    <button
                        type="button"
                        class="btn flex-1 text-sm {method === opt
                            ? 'preset-filled-primary-500'
                            : 'preset-outlined-surface-500 hover:preset-tonal'}"
                        onclick={() => select_method(opt)}
                    >
                        {method_labels[opt]()}
                    </button>
                {/each}
            </div>
        {/if}

        <form class="flex flex-col items-center gap-8" onsubmit={on_submit}>
            {#if method === "clock"}
                <div class="flex items-end gap-2">
                    <label class="flex flex-col items-center gap-1">
                        <span class="text-xs opacity-60">{m.numbers_hour()}</span>
                        <input
                            type="number"
                            class="input w-20 rounded-lg"
                            min="0"
                            max="23"
                            bind:value={clock_h}
                        />
                    </label>
                    <span class="text-xl font-bold">:</span>
                    <label class="flex flex-col items-center gap-1">
                        <span class="text-xs opacity-60">{m.numbers_minute()}</span>
                        <input
                            type="number"
                            class="input w-20 rounded-lg"
                            min="0"
                            max="59"
                            bind:value={clock_m}
                        />
                    </label>
                </div>
            {:else if method === "date"}
                <div class="flex items-end gap-1">
                    <label class="flex flex-col items-center gap-1">
                        <span class="text-xs opacity-60">{m.numbers_day()}</span>
                        <input
                            type="number"
                            class="input w-20 rounded-lg"
                            min="1"
                            max="31"
                            bind:value={date_d}
                        />
                    </label>
                    <span class="text-xl">.</span>
                    <label class="flex flex-col items-center gap-1">
                        <span class="text-xs opacity-60">{m.numbers_month()}</span>
                        <input
                            type="number"
                            class="input w-20 rounded-lg"
                            min="1"
                            max="12"
                            bind:value={date_m}
                        />
                    </label>
                    <span class="text-xl">.</span>
                </div>
            {:else}
                <input
                    type="number"
                    class="input w-xs rounded-lg"
                    bind:value={num_value}
                />
            {/if}

            <div class="flex w-full justify-between">
                <button
                    class="btn preset-outlined-surface-500 hover:preset-tonal"
                    type="button"
                    onclick={on_clear}
                >
                    {m.clear()}
                </button>
                <button class="btn preset-filled-primary-500" type="submit">
                    {m.submit()}
                </button>
            </div>
        </form>
    </FormWrapper>

    <div>
        {#if data.error}
            <ErrorBox error={data.error} />
        {:else if data.result && data.result.length > 0}
            <div
                class="card preset-filled-surface-50-950 border-surface-200-800 rounded-lg border p-4 shadow-lg"
            >
                <div class="grid grid-cols-[1fr_auto] gap-4 text-base lg:text-lg">
                    {#each data.result as [input, output]}
                        <span class="font-bold">
                            {input}
                        </span>
                        <span class="break-all">
                            {output}
                        </span>
                    {/each}
                </div>
            </div>
        {:else if data.q}
            <p>{m.noresults()}</p>
        {/if}
    </div>
</div>
