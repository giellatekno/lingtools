<script lang="ts">
    import type { PageData } from "./$types";
    import ParadigmWrapper from "$components/ParadigmWrapper.svelte";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import ParadigmForm from "$components/ParadigmForm.svelte";
    import { hasParadigmSchema } from "$lib/paradigms/registry";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "$lib/langnames";
    import { page } from "$app/state";

    interface Props {
        data: PageData;
    }
    let { data }: Props = $props();
    let word = $derived(data.word || "");
    let pos = $derived(data.pos || "");

    let lang = $derived(page.params.lang || "");

    let format = $state("table");
</script>

<svelte:head>
    <title>{m.paradigm_title()} • {langname(lang, getLocale())} • {m.page_title()}</title>
</svelte:head>

<div class="flex flex-col items-center gap-4">
    <h3 class="h4 lg:h3">{m.paradigm_title()}</h3>
    <FormWrapper tool="paradigm">
        <ParadigmForm {word} {pos} bind:format has_tables={hasParadigmSchema(lang)} />
    </FormWrapper>

    <hr class="hr my-2 lg:my-8" />

    {#if data.parsed}
        <ParadigmWrapper data={data.parsed} {format} search={word} />
    {:else if data.error}
        <ErrorBox error={data.error} />
    {/if}
</div>
