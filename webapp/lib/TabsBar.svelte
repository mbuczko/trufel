<script>
import { createEventDispatcher, setContext } from "svelte";

const dispatch = createEventDispatcher();

/** @type {String} - currently selected tab Id */
let currentTabId;

/** @type {number} - number of added tabs */
let tabsCount = 0;

setContext('tabs', {
    registerTab: () => tabsCount++,
    select: (/** @type {String} */ id) => {
        currentTabId = id;
        dispatch('change', {id: currentTabId})
    }
})

</script>

<div class="tabs relative inline-grid items-center justify-center w-full h-10 grid-cols-{tabsCount} p-1 text-gray-500 bg-gray-100 rounded-lg select-none">
    <slot />
    <div class="absolute left-0 z-10 w-1/{tabsCount} h-full duration-300 ease-out"
         style="height: 32px; left: 192px;">
        <div class="w-full h-full bg-white rounded-md shadow-sm"></div>
    </div>
</div>
