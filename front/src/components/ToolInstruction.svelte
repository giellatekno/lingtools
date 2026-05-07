<script lang="ts">
    import type { Tools } from "$lib/langs";
    import { m } from "$lib/paraglide/messages";
    import { CircleQuestionMarkIcon, XIcon } from "@lucide/svelte";
    import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";

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

<Dialog>
    <Dialog.Trigger
        class="btn-icon hover:preset-tonal rounded-full"
        aria-label={m.instruction_title()}
    >
        <CircleQuestionMarkIcon class="size-6" />
    </Dialog.Trigger>
    <Portal>
        <Dialog.Backdrop class="bg-surface-950/50 fixed inset-0 z-40" />
        <Dialog.Positioner
            class="fixed inset-0 z-40 flex items-center justify-center p-4"
        >
            <Dialog.Content
                class="card bg-surface-50-950 border-surface-200-800 w-full max-w-md rounded-xl border p-6 shadow-xl"
            >
                <header class="flex items-center justify-between">
                    <Dialog.Title class="text-lg font-bold">
                        {m.instruction_title()}
                    </Dialog.Title>
                    <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                        <XIcon class="size-4" />
                    </Dialog.CloseTrigger>
                </header>
                <p class="text-surface-700-300 text-sm leading-relaxed">
                    {instructions[tool]()}
                </p>
            </Dialog.Content>
        </Dialog.Positioner>
    </Portal>
</Dialog>
