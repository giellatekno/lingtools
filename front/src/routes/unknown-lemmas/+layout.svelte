<script lang="ts">
    import { m } from "$lib/paraglide/messages";

    let { children } = $props();
</script>

<div class="mx-2 lg:mt-4">
    <h3 class="h5 lg:h3">{m.unknownlemmas_title()}</h3>
    <div class="w-full">
        {m.unknownlemmas_description()}
    </div>
</div>

<hr class="hr mt-4 mb-8" />

{@render children?.()}
