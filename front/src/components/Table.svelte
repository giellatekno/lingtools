<script lang="ts">
    let { children, theme = "paradigm" } = $props();
</script>

<div data-table-container>
    <table data-table-theme={theme}>
        {@render children?.()}
    </table>
</div>
