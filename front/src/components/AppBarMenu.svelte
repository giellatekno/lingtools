<script lang="ts">
    import { goto } from "$app/navigation";
    import { resolve } from "$app/paths";
    import type { MessageFunction } from "@inlang/paraglide-js";
    import { ChevronDown } from "@lucide/svelte";
    import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

    interface Props {
        title: MessageFunction;
        lang?: string;
        items: {
            title: MessageFunction;
            route: string;
        }[];
    }
    let { title, lang, items }: Props = $props();

    function on_select(route: string, lang: string | undefined) {
        if (lang) {
            goto(resolve(`/${lang}/${route}`));
        } else {
            goto(resolve(`/${route}`));
        }
    }
</script>

<Menu onSelect={(e) => on_select(e.value, lang)}>
    <Menu.Trigger class="btn hover:preset-tonal hover:text-surface-50">
        {title()}
        <ChevronDown class="size-5" />
    </Menu.Trigger>
    <Portal>
        <Menu.Positioner>
            <Menu.Content
                class="card preset-filled-surface-50-950 z-50 min-w-48 p-2 shadow-xl"
            >
                {#each items as { title, route }}
                    <Menu.Item value={route} class="btn w-full justify-start">
                        <Menu.ItemText>{title()}</Menu.ItemText>
                    </Menu.Item>
                {/each}
            </Menu.Content>
        </Menu.Positioner>
    </Portal>
</Menu>
