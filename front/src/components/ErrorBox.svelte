<script lang="ts">
    import { m } from "$lib/paraglide/messages";
    let { error } = $props();
</script>

<div
    class="card border-error-500 preset-filled-surface-50-950 flex w-full max-w-lg flex-col gap-2 rounded-lg border-2 p-4 shadow-lg"
>
    <h4 class="h4 text-error-500">{m.erroroccured()}</h4>
    <p>{error}</p>
</div>
