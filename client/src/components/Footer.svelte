<script lang="ts">
    import giellatekno_logo from "$assets/giellatekno_logo.svg";
    import uit_logo_sme from "$assets/uit_logo/uit_logo_sme.png";
    import { m } from "$lib/paraglide/messages";
</script>

<footer class="bg-surface-800-200 text-surface-100-900 flex w-full justify-center p-4">
    <div
        class="grid grid-cols-[1fr_1fr] place-items-center gap-2 lg:grid-cols-[1fr_auto_1fr]"
    >
        <enhanced:img
            class="order-2 h-5 scale-100 invert lg:order-1 lg:block lg:h-10"
            src={giellatekno_logo}
            alt="Giellatekno logo"
        />
        <div
            class="order-1 col-span-2 flex flex-col justify-between gap-4 text-center text-xs lg:order-2 lg:col-span-1 lg:text-base"
        >
            <div>
                {m.feedback()}
                <a
                    href="mailto:giellatekno@uit.no"
                    class="text-primary-300-700 hover:underline"
                >
                    giellatekno@uit.no
                </a>
            </div>
            <div>
                {m.copyright()}
            </div>
        </div>
        <enhanced:img
            class="order-3 h-10 scale-100 lg:block lg:h-20"
            src={uit_logo_sme}
            alt="UiT logo"
        />
    </div>
</footer>
