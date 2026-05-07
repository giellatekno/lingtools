<script lang="ts">
    import "../app.css";
    import AppBar from "$components/AppBar.svelte";
    import Footer from "$components/Footer.svelte";

    let { children } = $props();
</script>

<svelte:head>
    <title>LingTools</title>
    <meta
        name="description"
        content="Get access to Giellateknos linguistic tools through a simple webinterface"
    />
</svelte:head>

<div class="app flex h-full min-h-screen flex-col">
    <AppBar />
    <main class="mx-auto w-full max-w-480 flex-1 p-2 pb-16 lg:p-4">
        {@render children?.()}
    </main>
    <Footer />
</div>
