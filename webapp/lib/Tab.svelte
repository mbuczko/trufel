<script>
import { getContext, onMount } from 'svelte';

/** @type {string} */
export let id;

/** @type {string} */
export let label;

/** Notifies a parent about being selected */
const {selectTab, registerTab} = (getContext('tabs') || {});

/** @type HTMLElement */
let ref;

onMount(() => {
    registerTab(ref);
})
</script>

<button id={id}         
        type="button" 
        class="relative z-20 inline-flex items-center justify-center w-full h-8 px-3 text-sm font-medium transition-all rounded-md cursor-pointer whitespace-nowrap"
        bind:this={ref}
        on:click={() => { selectTab(ref) }}>
    {label}
</button>
