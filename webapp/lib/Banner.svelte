<script>
export let closable = true;
export let style = '';
</script>

<div class="banner absolute border-1 top-0 left-0 w-full h-auto py-2 bg-white shadow-sm sm:py-0 sm:h-10 {style}">
    <div class="flex items-center justify-between w-full h-full px-3 mx-auto max-w-7xl ">
        <span class="flex flex-col w-full h-full text-xs leading-6 text-black sm:flex-row sm:items-center opacity-80 hover:opacity-100">
            <span class="flex items-center">
                <slot name="title"/>
                <span class="hidden w-px h-4 mx-3 rounded-full sm:block bg-neutral-200"></span>
            </span>
            <span class="block pt-1 pb-2 leading-none sm:inline sm:pt-0 sm:pb-0">
                <slot name="description"/>
            </span>
        </span>
        {#if closable}
            <button class="flex items-center flex-shrink-0 translate-x-1 justify-center w-6 h-6 p-1.5 text-black rounded-full hover:bg-neutral-100">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-full h-full" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
        {/if}
    </div>
</div>
