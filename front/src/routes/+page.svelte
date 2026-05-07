<script lang="ts">
    import { goto } from "$app/navigation";
    import { resolve } from "$app/paths";
    import { langs, sami_langs, nonsamiuralic_langs, other_langs } from "$lib/langs";
    import { langname } from "$lib/langnames";
    import { SearchIcon } from "@lucide/svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";

    let search = $state("");
    let selectedIndex = $state(0);

    let locale = $derived(getLocale());

    let all_langs = $derived(
        langs
            .map((iso) => ({ iso, name: langname(iso, locale) }))
            .sort((a, b) => a.name.localeCompare(b.name)),
    );

    let suggestions = $derived(
        search === ""
            ? []
            : all_langs.filter(({ iso, name }) => {
                  const lower = search.toLowerCase();
                  return iso.includes(lower) || name.toLowerCase().includes(lower);
              }),
    );

    $effect(() => {
        // reset selection when suggestions change
        suggestions;
        selectedIndex = 0;
    });

    async function onkeydown(ev: KeyboardEvent) {
        if (suggestions.length === 0) return;
        if (ev.key === "ArrowDown") {
            ev.preventDefault();
            selectedIndex = (selectedIndex + 1) % suggestions.length;
        } else if (ev.key === "ArrowUp") {
            ev.preventDefault();
            selectedIndex = (selectedIndex - 1 + suggestions.length) % suggestions.length;
        } else if (ev.key === "Enter") {
            ev.preventDefault();
            await goto(resolve(`/${suggestions[selectedIndex].iso}`));
        }
    }

    let groups = $derived([
        {
            title: m.samilanguages,
            langset: all_langs.filter((e) => sami_langs.has(e.iso)),
        },
        {
            title: m.nonsamiuralic,
            langset: all_langs.filter((e) => nonsamiuralic_langs.has(e.iso)),
        },
        {
            title: m.otherlanguages,
            langset: all_langs.filter((e) => other_langs.has(e.iso)),
        },
    ]);
</script>

<svelte:head>
    <title>{m.page_title()}</title>
</svelte:head>

<div class=" flex w-full flex-col items-center gap-8 p-6">
    <div class="flex flex-col items-center gap-6">
        <p class="max-w-lg text-center text-lg">
            {m.index_description()}
        </p>
        <label class="label w-full max-w-sm">
            <div class="relative">
                <span class="label-text">{m.searchforlangs()}</span>
                <div class="input-group bg-surface-50 grid-cols-[1fr_auto]">
                    <input
                        class="ig-input text-lg"
                        type="text"
                        {onkeydown}
                        bind:value={search}
                        placeholder=""
                        autocomplete="off"
                    />
                    <div class="ig-cell preset-tonal px-4">
                        <SearchIcon class="size-5" />
                    </div>
                </div>
                {#if suggestions.length > 0}
                    <ul
                        class="card preset-filled-surface-50-950 absolute top-full z-50 mt-1 w-full overflow-y-auto shadow-lg"
                        style="max-height: 16rem"
                    >
                        {#each suggestions as { iso, name }, i}
                            <li>
                                <a
                                    class="hover:preset-tonal block px-4 py-2 text-base {i ===
                                    selectedIndex
                                        ? 'preset-tonal'
                                        : ''}"
                                    href={resolve(`/${iso}`)}
                                >
                                    <div class="flex justify-between">
                                        <span>{name}</span>
                                        <span class="opacity-75">({iso})</span>
                                    </div>
                                </a>
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>
        </label>
    </div>
    <!-- <hr class="hr border-surface-400-600" /> -->

    <div class="grid w-fit grid-cols-1 gap-12 lg:grid-cols-3">
        {#each groups as group}
            {#if group.langset.length}
                <div class="flex flex-col gap-2">
                    <h4 class="lg:h4 h5">{group.title()}</h4>
                    <div class="grid grid-cols-2 gap-2">
                        {#each group.langset as { iso, name }}
                            <a
                                class="btn preset-outlined-primary-500 bg-surface-50 hover:preset-tonal-primary w-full text-center text-sm text-wrap lg:text-base"
                                href={resolve(`/${iso}`)}
                            >
                                {name}
                            </a>
                        {/each}
                    </div>
                </div>
            {/if}
        {/each}
    </div>
</div>
