<script>
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher();

export let minW = 540;
export let maxH = 270;

/** @type {HTMLDialogElement} */
let ref;

export const open = () => {
    dispatch('init')
    ref.showModal();
}
</script>

<dialog class="modal w-full min-w-[{minW}px] max-w-screen-md max-h-[{maxH}px] bg-slate-500 shadow-md border-1 rounded-lg"
        bind:this={ref}>
    <div class="flex">
        <slot />
    </div>
</dialog>
