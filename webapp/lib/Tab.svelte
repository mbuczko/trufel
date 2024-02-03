<script>
import { getContext } from 'svelte';

/** @type {String} */
export let id;

/** @type {String} */
export let label;

/** Notifies a parent about being selected */
const select = (getContext('tabs') || {}).select;

getContext('tabs').registerTab();

</script>

<button id={id}         
        type="button" 
        class="relative z-20 inline-flex items-center justify-center w-full h-8 px-3 text-sm font-medium transition-all rounded-md cursor-pointer whitespace-nowrap"
        on:click={() => { if (select) select(id) }}>
    {label}
</button>
