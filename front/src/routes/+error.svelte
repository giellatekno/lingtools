<script lang="ts">
    import { page } from "$app/state";
    import { m } from "$lib/paraglide/messages";
</script>

<div class="my-8 flex flex-col items-center">
    <div
        class="border-error-500 card flex max-w-xl flex-col items-center gap-2 border-2 p-4 text-center shadow-md"
    >
        <h3 class="h3 text-error-500 font-bold">
            {page.status}: {page.error?.message}
        </h3>
        {#if page.status === 404}
            <h4 class="h4">{m.notfound()}</h4>
        {/if}
    </div>
</div>
