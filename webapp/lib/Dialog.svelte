<script>
import Toggle from './Toggle.svelte';
import IconsComposer from './IconComposer.svelte';
import Autocomplete from './Autocomplete.svelte';

/** @type {HTMLDialogElement} */
let ref;

/** @type {HTMLElement} */
let defaultInput;

/** @type {boolean} */
let iconComposer = false;

/** @type {string} - SVG icon definition */
let svgIcon = '';

/** @type {string} */
let url;

/** @type {string} */
let title;

/** @type {string} */
let description;

/** @type {boolean} */
let isShared;

/** @type {boolean} */
let isSearchable;

export const openDialog = () => {
    ref.showModal();
    defaultInput.focus()
}
</script>

<dialog id="dialog"
        class="create-dialog w-full min-w-[540px] max-w-screen-md max-h-[270px] bg-slate-500 shadow-md border-1 rounded-lg"
        bind:this={ref}>
    <div class="flex">
        <div class="sidebar p-2 grid grid-cols-[64px_1fr] justify-items-start">
            <button
                class="self-start icon w-14 h-14 mb-4 border-2 border-slate-700 opacity-80 rounded-lg shadow-md active:shadow-none hover:opacity-100"
                on:click={() => iconComposer = true}>
                {@html svgIcon}
            </button>
            <div class="self-start pt-2 min-w-40">
                <h3 class="title text-gray-800 font-medium uppercase"> {title || '---'} </h3>
                <div class="description text-xs text-gray-700"> {description || 'No description'} </div>
            </div>
            <div class="mt-14 col-span-2">
                <Toggle id="shared" label="Shared with others" hint="Other users can see this too" enabled={isShared} />
                <Toggle id="searchable" label="Intrinsic search" hint="Enables search within the app" enabled={isSearchable} />
            </div>
        </div>
        <div class="details relative p-4 flex-1 bg-white">
            {#if iconComposer}
                <IconsComposer
                    svg={svgIcon}
                    on:apply={({detail}) => { svgIcon = detail.svg; iconComposer = false; }}
                    on:close={() => iconComposer = false}>
                </IconsComposer>
            {:else}
                <form class="grid grid-cols-[1fr_1fr] gap-2">
                    <div class="col-span-2">
                        <label for="url"> URL </label>
                        <input bind:value={url} bind:this={defaultInput} name="url" id="url" type="text" class="text-sm" />
                    </div>
                    <div>
                        <label for="type"> Type </label>
                        <Autocomplete placeholder="Choose type..." />
                    </div>
                    <div>
                        <label for="category"> Category </label>
                        <Autocomplete placeholder="Choose category..." />
                    </div>
                    <div>
                        <label for="title"> Title </label>
                        <input bind:value={title} name="title" id="title" class="text-sm" type="text" maxlength="64"/>
                    </div>
                    <div>
                        <label for="description"> Description </label>
                        <input bind:value={description} name="description" id="description" class="text-sm" type="text" maxlength="64" />
                    </div>

                    <div class="form-buttons col-span-2">
                        <button> Cancel </button>
                        <input name="submit" type="submit" value="Save" class="submit-button" />
                    </div>
                </form>
            {/if}
        </div>
    </div>
</dialog>
