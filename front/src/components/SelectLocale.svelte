<script lang="ts">
    import { Popover, Portal } from "@skeletonlabs/skeleton-svelte";
    import { LanguagesIcon } from "@lucide/svelte";
    import sme_flag from "$assets/flags/sme.svg";
    import nob_flag from "$assets/flags/nob.svg";
    import fin_flag from "$assets/flags/fin.svg";
    import eng_flag from "$assets/flags/eng.svg";
    import rus_flag from "$assets/flags/rus.svg";
    import { m } from "$lib/paraglide/messages";
    import { locales, setLocale } from "$lib/paraglide/runtime";
    import { langname } from "$lib/langnames";

    let flag_icons: Record<string, string> = {
        sme: sme_flag,
        nob: nob_flag,
        fin: fin_flag,
        eng: eng_flag,
        rus: rus_flag,
    };
</script>

<Popover>
    <Popover.Trigger
        class="btn preset-filled-primary-500 lg:preset-filled-primary-50-950"
    >
        <LanguagesIcon />
        {m.interfacelanguage()}
    </Popover.Trigger>
    <Portal>
        <Popover.Positioner>
            <Popover.Content
                class="card preset-filled-primary-50-950 z-50 w-fit p-4 shadow-xl"
            >
                <Popover.Description>
                    <ul class="w-full list-none">
                        {#each locales as iso}
                            <li>
                                <button
                                    class="btn hover:preset-tonal w-full justify-start"
                                    onclick={() => setLocale(iso)}
                                >
                                    <img
                                        src={flag_icons[iso]}
                                        alt={iso + " flag"}
                                        class="w-8 shadow-md"
                                    />
                                    {langname(iso, iso)}
                                </button>
                            </li>
                        {/each}
                    </ul>
                </Popover.Description>
                <Popover.Arrow
                    class="[--arrow-background:var(--color-primary-50-950)] [--arrow-size:--spacing(2)]"
                >
                    <Popover.ArrowTip />
                </Popover.Arrow>
            </Popover.Content>
        </Popover.Positioner>
    </Portal>
</Popover>
