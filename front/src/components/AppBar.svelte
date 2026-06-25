<script lang="ts">
    import { AppBar } from "@skeletonlabs/skeleton-svelte";
    import SelectLocale from "./SelectLocale.svelte";
    import { resolve } from "$app/paths";
    import { page } from "$app/state";
    import { InfoIcon } from "@lucide/svelte";
    import MobileSideBar from "./MobileSideBar.svelte";
    import { m } from "$lib/paraglide/messages";
    import { tools_for } from "$lib/langs";
    import AppBarMenu from "./AppBarMenu.svelte";

    let lang = $derived(page.params.lang || "");

    const lang_tools = $derived(
        (
            [
                { title: m.analyze_title, route: "analyze" },
                { title: m.dependency_title, route: "dependency" },
                { title: m.disambiguate_title, route: "disambiguate" },
                { title: m.generate_title, route: "generate" },
                { title: m.hyphenate_title, route: "hyphenate" },
                { title: m.numbers_title, route: "numbers" },
                { title: m.paradigms_title, route: "paradigms" },
                { title: m.transcribe_title, route: "transcribe" },
            ] as const
        ).filter((tool) => lang && tools_for[lang].includes(tool.route)),
    );

    const other_tools = [
        { title: m.unknownlemmas_title, route: "unknown-lemmas" },
        { title: m.lemmacount_title, route: "lemmacount" },
    ];
</script>

<AppBar
    class="bg-primary-600 text-primary-contrast-950-50 border-surface-950-50 sticky top-0 z-50 "
>
    <AppBar.Toolbar
        class="mx-auto w-full max-w-480 grid-cols-[1fr_auto_1fr] lg:grid-cols-[auto_auto]"
    >
        <AppBar.Lead class="lg:hidden">
            <MobileSideBar />
        </AppBar.Lead>
        <AppBar.Headline class="hidden flex-row items-center gap-1 lg:flex">
            <a href={resolve("/")} class="mr-4 text-xl font-bold lg:text-3xl">
                {m.page_title()}
            </a>

            <a
                href={resolve("/")}
                class="btn hover:preset-tonal hover:text-surface-50 flex items-center gap-2"
            >
                {m.languages()}
            </a>

            {#if lang}
                <AppBarMenu title={m.languagetools} items={lang_tools} {lang} />
            {/if}

            <AppBarMenu title={m.other_tools} items={other_tools} />
        </AppBar.Headline>

        <AppBar.Trail class="hidden h-full flex-row items-center gap-1 lg:flex">
            <a
                class="btn hover:preset-tonal hover:text-surface-50"
                href={resolve("/about")}
            >
                <InfoIcon class="size-5" />
                {m.about()}
            </a>
            <SelectLocale />
        </AppBar.Trail>
    </AppBar.Toolbar>
</AppBar>
