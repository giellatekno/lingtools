<script lang="ts">
    import type { PageData } from "./$types";
    import ErrorBox from "$components/ErrorBox.svelte";
    import FormWrapper from "$components/FormWrapper.svelte";
    import TextForm from "$components/TextForm.svelte";
    import { m } from "$lib/paraglide/messages";
    import { getLocale } from "$lib/paraglide/runtime";
    import { langname } from "$lib/langnames";
    import { page } from "$app/state";
    import DisambResult from "$components/DisambResult.svelte";

    interface Props {
        data: PageData;
    }

    let lang = $derived(page.params.lang || "");

    let { data }: Props = $props();

    // the search text
    let value = $derived(data.q);
</script>

<svelte:head>
    <title>
        {m.dependency_title()} • {langname(lang, getLocale())} • {m.page_title()}
    </title>
</svelte:head>

<div class="flex flex-col items-center gap-4 lg:gap-8">
    <FormWrapper tool="dependency">
        <TextForm bind:value />
    </FormWrapper>

    {#if data.error}
        <ErrorBox error={data.error} />
    {/if}

    <DisambResult results={data.results} />
</div>
