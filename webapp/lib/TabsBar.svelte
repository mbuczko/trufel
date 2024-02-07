<script>
import { createEventDispatcher, onMount, setContext } from "svelte";

/** @type {String | null} - currently selected tab Id or null for no selection */
export let currentTabId = null;

/** @type {number} - number of added tabs */
let tabsCount = 0;

/** @type HTMLElement - sliding tab selection */
let marker;

const dispatch = createEventDispatcher();

/**
 * Selects a tab or removes existing selection if
 * null was provided as a parameter.
 *
 * @param {HTMLElement | null} tab - tab to select
 * @fires on:change event with Id of selected tab or null when deselecting
 */
function selectTab(tab) {
    if (tab && tab.id) {
        marker.style.width = tab.offsetWidth + 'px';
        marker.style.height = tab.offsetHeight + 'px';
        marker.style.left = tab.offsetLeft + 'px';
        marker.style.display = 'block';
        currentTabId = tab.id;
    } else {
        currentTabId = null;
        marker.style.display = 'none';
    }
    dispatch('change', {id: currentTabId})
}

onMount(() => {
    requestAnimationFrame(() => {
        if (currentTabId) {
            selectTab(document.getElementById(currentTabId));
        }
    });

})

setContext('tabs', {
    registerTab: (/** @type {HTMLElement} */ tab) => {
        if (tabsCount++ === 0) {
            currentTabId = tab.id;
        }
    },
    selectTab: (/** @type {HTMLElement} */ tab) => {
        selectTab(tab);
    }
})
</script>

<div class="tabs relative inline-grid items-center justify-center w-full h-10 grid-cols-{tabsCount} p-1 text-gray-500 bg-gray-100 rounded-lg select-none">
    <div bind:this={marker}
         class="absolute left-0 z-10 w-1/{tabsCount} h-full duration-300 ease-out hidden">
        <div class="w-full h-full bg-white rounded-md shadow-sm"></div>
    </div>
    <slot />
</div>

