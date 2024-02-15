<script>
import { slide } from 'svelte/transition';
import { quintInOut } from 'svelte/easing';
import { notification } from '$lib/store';

export let timeout = 5000;

/** @type {number} */
let timeoutId;

notification.subscribe(_ => {
    if (timeoutId) {
        clearTimeout(timeoutId)
    }
    timeoutId = setTimeout(() => notification.set(''), timeout);
})
</script>

{#if $notification}
<div class="notification bg-gradient-to-b from-red-500 to-red-600 absolute opacity-90 hover:opacity-100 border-1 top-0 left-0 w-full h-auto py-2 shadow-md z-30"
     in:slide={{ duration: 200, axis: 'y', easing: quintInOut }}>
    <div class="flex items-center px-3 ">
        <span class="flex-1 text-center text-xs leading-6 text-black opacity-80">
            <span class="block leading-none">
                {$notification}
            </span>
        </span>
        <button class="flex items-center flex-shrink-0 translate-x-1 justify-center w-6 h-6 p-1.5 text-black rounded-full hover:bg-red-400"
                on:click={() => notification.set('')}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-full h-full" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
        </button>
    </div>
</div>
{/if}
