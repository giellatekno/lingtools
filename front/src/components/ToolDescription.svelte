<script lang="ts">
    import type { Tools } from "$lib/langs";
    import { m } from "$lib/paraglide/messages";
    import { ChevronDown, ChevronUp, CircleQuestionMarkIcon } from "@lucide/svelte";
    import { Accordion } from "@skeletonlabs/skeleton-svelte";
    let { tool }: { tool: Tools } = $props();

    const instructions = {
        analyze: m.analyze_instruction,
        dependency: m.dependency_instruction,
        disambiguate: m.disambiguate_instruction,
        generate: m.generate_instruction,
        hyphenate: m.hyphenate_instruction,
        num: m.num_instruction,
        paradigm: m.paradigm_instruction,
        transcribe: m.transcribe_instruction,
    };
</script>

<Accordion collapsible>
    <Accordion.Item value="1">
        <h3>
            <Accordion.ItemTrigger class="flex flex-row items-center justify-between">
                <span class="flex flex-row gap-2">
                    <CircleQuestionMarkIcon />
                    {m.instruction_title()}
                </span>
                <Accordion.ItemIndicator class="group">
                    <ChevronUp class="hidden size-4 group-data-[state=open]:block" />
                    <ChevronDown class="block size-4 group-data-[state=open]:hidden" />
                </Accordion.ItemIndicator>
            </Accordion.ItemTrigger>
            <Accordion.ItemContent>
                {instructions[tool]()}
            </Accordion.ItemContent>
        </h3>
    </Accordion.Item>
</Accordion>
