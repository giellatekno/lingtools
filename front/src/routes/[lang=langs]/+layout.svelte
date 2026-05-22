<script lang="ts">
    import { page } from "$app/state";
    import { langname } from "@giellatekno/langnames";
    import { resolve } from "$app/paths";
    import { getLocale } from "$lib/paraglide/runtime";
    import { m } from "$lib/paraglide/messages";

    let { children } = $props();

    let lang = $derived(page.params.lang || "");

    let locale = $state(getLocale());

    let maturity_shield_url = $derived(
        `https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fgiellalt%2Flang-${lang}%2Fmain%2Fdocs%2Fbadgedata%2Ffst-maturity.json`,
    );
    let documentation_shield_url = `https://img.shields.io/badge/${m.documentation()}-blue`;
</script>

<div class="flex flex-col gap-4 lg:gap-6">
    <div class="mt-2 flex w-full flex-col items-center lg:mt-4">
        <div
            class="flex w-full flex-col gap-1 lg:flex-row lg:items-center lg:justify-between"
        >
            <a href={resolve(`/${lang}`)} class="lg:h3 h6 hover:underline">
                {langname(lang, locale)}
            </a>
            <div class="flex gap-2">
                <a href={`https://giellalt.github.io/lang-${lang}`} class="w-fit shadow">
                    <img
                        src={documentation_shield_url}
                        alt="[l6e] Link to language documentation"
                        class="h-5 lg:h-7"
                    />
                </a>

                <a
                    href="https://giellalt.github.io/MaturityClassification.html"
                    class="w-fit shadow"
                >
                    <img
                        src={maturity_shield_url}
                        alt="[l6e] Language maturity and link to maturity documentation"
                        class="h-5 lg:h-7"
                    />
                </a>
            </div>
        </div>
    </div>
    <hr class="hr border-surface-400-600" />
    {@render children?.()}
</div>
