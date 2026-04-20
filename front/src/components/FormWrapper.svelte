<script lang="ts">
    import ToolDescription from "./ToolDescription.svelte";

    let { children, tool } = $props();
</script>

<div class="flex w-full max-w-md flex-col gap-2 lg:max-w-xl">
    <!-- <ToolDescription {tool} /> -->
    <div
        class="card preset-filled-surface-100-900 border-surface-200-800 w-full max-w-md border p-4 lg:max-w-xl"
    >
        {@render children?.()}
    </div>
</div>
