<script lang="ts">
    import GTLogoTextUnder from "./GTLogoTextUnder.svelte";
    import uit_logo_sme from "$assets/uit_logo/uit_logo_sme.png";
    import { m } from "$lib/paraglide/messages";
</script>

<footer
    class="bg-surface-800-200 text-surface-100-900 flex w-full items-center justify-center px-6 py-4 lg:py-3"
>
    <div class="flex flex-col items-center gap-3 lg:flex-row lg:gap-6">
        <div class="flex items-center gap-6">
            <a href="https://giellatekno.uit.no" aria-label="Giellatekno">
                <GTLogoTextUnder class="h-9" />
            </a>
            <a href="https://uit.no" class="lg:hidden" aria-label="UiT">
                <enhanced:img class="h-12 lg:h-14" src={uit_logo_sme} alt="UiT logo" />
            </a>
        </div>
        <div class="text-center text-xs opacity-70">
            {m.feedback()}
            <a
                href="mailto:giellatekno@uit.no"
                class="text-primary-300-700 hover:underline"
            >
                giellatekno@uit.no
            </a>
        </div>
        <a href="https://uit.no" class="hidden lg:block" aria-label="UiT">
            <enhanced:img class="h-12 lg:h-14" src={uit_logo_sme} alt="UiT logo" />
        </a>
    </div>
</footer>
