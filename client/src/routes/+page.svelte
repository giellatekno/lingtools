<script lang="ts">
    import { goto } from "$app/navigation";
    import { resolve } from "$app/paths";
    import { langs, sami_langs, nonsamiuralic_langs, other_langs } from "$lib/langs";
    import { langname } from "$lib/langnames";
    import { SearchIcon } from "@lucide/svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";

    let search = $state("");

    let locale = $derived(getLocale());
    function filter_langs(search: string) {
        let rootset = langs;

        const mappedLangs = rootset.map((iso) => ({
            iso,
            name: langname(iso, locale),
        }));

        if (search === "") {
            return mappedLangs.sort((a, b) => a.name.localeCompare(b.name));
        } else {
            return mappedLangs
                .filter(({ iso, name }) => {
                    const lower = search.toLowerCase();
                    return iso.includes(lower) || name.toLowerCase().includes(lower);
                })
                .sort((a, b) => a.name.localeCompare(b.name));
        }
    }

    async function onenter(ev: KeyboardEvent) {
        if (ev.key === "Enter" && visible_langs.length === 1) {
            const lang = visible_langs[0].iso;
            await goto(resolve(`/${lang}`));
        }
    }
    let visible_langs = $derived(filter_langs(search));

    let groups = $derived([
        {
            title: m.samilanguages,
            langset: visible_langs.filter((e) => sami_langs.has(e.iso)),
        },
        {
            title: m.nonsamiuralic,
            langset: visible_langs.filter((e) => nonsamiuralic_langs.has(e.iso)),
        },
        {
            title: m.otherlanguages,
            langset: visible_langs.filter((e) => other_langs.has(e.iso)),
        },
    ]);
</script>

<svelte:head>
    <title>{m.page_title()}</title>
</svelte:head>

<div class="flex w-full flex-col gap-4">
    <label class="label lg:ml-2">
        <span class="label-text">{m.showtoolsfor()}</span>
        <div class="input-group w-3xs grid-cols-[auto_1fr]">
            <div class="ig-cell preset-tonal">
                <SearchIcon class="size-4" />
            </div>
            <input
                class="ig-input"
                type="text"
                onkeydown={onenter}
                bind:value={search}
                placeholder=""
            />
        </div>
    </label>
    <hr class="hr" />

    <div class="flex w-full flex-row flex-wrap justify-evenly gap-8 lg:mx-2">
        {#if visible_langs.length}
            {#each groups as group}
                {#if group.langset.length}
                    <div class="flex w-full flex-col gap-2 lg:w-fit">
                        <h4 class="lg:h4 h5">{group.title()}</h4>
                        <div class="grid min-w-max grid-cols-2 gap-2">
                            {#each group.langset as { iso, name }}
                                <a
                                    class="btn preset-outlined-primary-500 hover:preset-tonal w-full text-center text-sm text-wrap lg:text-base"
                                    href={resolve(`/${iso}`)}
                                >
                                    {name}
                                </a>
                            {/each}
                        </div>
                    </div>
                {/if}
            {/each}
        {:else}
            <span>
                {m.noresults()}
            </span>
        {/if}
    </div>
</div>
