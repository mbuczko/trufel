<script>

/** @type {string} */
export let title;

</script>

<div class="command-section-items px-1 overflow-hidden text-left text-gray-700">
    <div class="px-2 py-1 my-1 text-xs font-medium text-neutral-500">{title}</div>
    <slot />
</div>

<style>
 .command-section-items:not(:has(div.command-item)) {
     display: none;
 }
</style>
