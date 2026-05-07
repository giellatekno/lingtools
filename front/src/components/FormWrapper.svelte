<script lang="ts">
    import { m } from "$lib/paraglide/messages";
    import type { Snippet } from "svelte";
    import ToolDescription from "./ToolInstruction.svelte";
    import type { Tools } from "$lib/langs";

    let { children, tool }: { children: Snippet; tool: Tools } = $props();

    const titles = {
        analyze: m.analyze_title,
        dependency: m.dependency_title,
        disambiguate: m.disambiguate_title,
        generate: m.generate_title,
        hyphenate: m.hyphenate_title,
        num: m.num_title,
        paradigm: m.paradigm_title,
        transcribe: m.transcribe_title,
    };
</script>

<div
    class="card preset-filled-surface-50-950 border-surface-200-800 flex w-full max-w-xl flex-col gap-2 rounded-lg border p-4 shadow-lg lg:p-6"
>
    <div class="mb-2 flex items-center justify-between">
        <h3 class="h4 lg:h3">{titles[tool]()}</h3>

        <ToolDescription {tool} />
    </div>
    {@render children?.()}
</div>
