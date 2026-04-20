<script lang="ts">
    import { AppBar } from "@skeletonlabs/skeleton-svelte";
    import SelectLocale from "./SelectLocale.svelte";
    import { resolve } from "$app/paths";
    import { page } from "$app/state";
    import { GlobeIcon, InfoIcon, ToolboxIcon, WrenchIcon } from "@lucide/svelte";
    import MobileSideBar from "./MobileSideBar.svelte";
    import { m } from "$lib/paraglide/messages";

    let lang = $derived(page.params.lang || "");
</script>

<AppBar
    class="bg-primary-600 text-primary-contrast-950-50 border-surface-950-50 sticky top-0 z-50 border-b-2"
>
    <AppBar.Toolbar
        class="mx-auto w-full max-w-480 grid-cols-[1fr_auto_1fr] lg:grid-cols-[auto_auto]"
    >
        <AppBar.Lead class="lg:hidden">
            <MobileSideBar />
        </AppBar.Lead>
        <AppBar.Headline class="flex flex-row gap-4">
            <a href={resolve("/")} class="text-xl font-bold lg:text-3xl">
                {m.page_title()}
            </a>
            <a href={resolve("/")} class="lg:btn ml-4 hidden hover:underline">
                <GlobeIcon class="size-6" />
                {m.languages()}
            </a>
            <!-- <div class="group relative flex items-center"> -->
            <!--     <button class="btn hover:underline">{m.other_tools()}</button> -->
            <!---->
            <!--     <div -->
            <!--         class="invisible absolute top-full left-0 -->
            <!--     z-50 w-fit opacity-0 transition-all duration-200 group-hover:visible group-hover:opacity-100" -->
            <!--     > -->
            <!--         <div -->
            <!--             class="card preset-filled-primary-50-950 space-y-2 p-2 shadow-xl" -->
            <!--         > -->
            <!--             <a -->
            <!--                 href={resolve("/unknown-lemmas")} -->
            <!--                 class="hover:preset-tonal btn block w-full" -->
            <!--             > -->
            <!--                 {m.unknownlemmas_title()} -->
            <!--             </a> -->
            <!--             <a -->
            <!--                 href={resolve("/lemmacount")} -->
            <!--                 class="hover:preset-tonal btn w-full" -->
            <!--             > -->
            <!--                 {m.lemmacount_title()} -->
            <!--             </a> -->
            <!--         </div> -->
            <!--     </div> -->
            <!-- </div> -->
            {#if lang}
                <a href={resolve(`/${lang}`)} class="lg:btn hidden hover:underline">
                    <ToolboxIcon />
                    {m.toolspage()}
                </a>
            {/if}
        </AppBar.Headline>

        <AppBar.Trail class="hidden h-full flex-row gap-4 lg:flex">
            <a class="btn hover:underline" href={resolve("/about")}>
                <InfoIcon />
                {m.about()}
            </a>
            <SelectLocale />
        </AppBar.Trail>
    </AppBar.Toolbar>
</AppBar>
