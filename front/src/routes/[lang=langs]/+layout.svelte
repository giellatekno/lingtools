<script lang="ts">
    import { page } from "$app/state";
    import { langname } from "$lib/langnames";
    import { resolve } from "$app/paths";
    import { getLocale } from "$lib/paraglide/runtime";

    let { children } = $props();

    let lang = $derived(page.params.lang || "");

    let locale = $derived(getLocale());

    let maturity_url = $derived(
        `https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fgiellalt%2Flang-${lang}%2Fmain%2Fdocs%2Fbadgedata%2Ffst-maturity.json`,
    );
</script>

<div class="flex flex-col gap-4 lg:gap-6">
    <div class="mt-2 flex w-full flex-col items-center lg:mt-4">
        <div class="flex w-full flex-row items-center justify-between gap-1">
            <a href={resolve(`/${lang}`)} class="lg:h3 h6 hover:underline">
                {langname(lang, locale)}
            </a>
            <a href={`https://giellalt.github.io/lang-${lang}`} class="shadow">
                <img src={maturity_url} alt="" class="h-5 lg:h-7" />
            </a>
        </div>
    </div>
    <hr class="hr border-surface-400-600" />
    {@render children?.()}
</div>
