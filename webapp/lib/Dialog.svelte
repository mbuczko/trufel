<script>
import Toggle from '$lib/Toggle.svelte';
import IconsComposer from './IconComposer.svelte';

/** @type {boolean} */
let iconComposer = false;

/** @type {String} - SVG icon definition */
let svgIcon = '';

</script>

<div class="dialog w-full flex min-w-[500px] min-h-[270px] max-w-screen-md max-h-[270px] shadow-md border-1">
    <div class="sidebar grid grid-cols-[64px_1fr] justify-items-start bg-slate-500 rounded-tl-lg rounded-bl-lg p-2">
        <button
            class="self-start icon w-14 h-14 border-2 border-slate-700 opacity-80 rounded-lg shadow-md active:shadow-none hover:opacity-100"
            on:click="{() => iconComposer = true}">
            {@html svgIcon}
        </button>
        <div class="self-start pt-2 min-w-40">
            <h3 class="text-gray-800 font-medium uppercase"> Proxmox </h3>
            <div class="text-xs text-gray-700"> Clustered Virtual Machines </div>
        </div>
        <div class="col-span-2 text-left">
            <Toggle id="shared" label="Shared with others" enabled={false} />
        </div>
        <div class="col-span-2 text-left">
            <Toggle id="searchable" label="Searchable" enabled={false} />
        </div>
        <div class="col-span-2 text-left">
            <Toggle id="disabled" label="Disabled" enabled={false} />
        </div>
    </div>
    <div class="details relative flex-1 bg-white border-1 rounded-tr-lg rounded-br-lg p-4">
        {#if iconComposer}
            <IconsComposer
                svg={svgIcon}
                on:close={() => iconComposer = false}
                on:apply={({detail}) => {
                         svgIcon = detail.svg;
                         iconComposer = false;
                         }}/>
        {:else}
                <form>
                    <label for="url"> URL </label>
                    <input name="url" id="url" type="text"/>


                    <label for="title"> Title </label>
                    <input name="title" id="title" type="text"/>

                    <label for="description"> Description </label>
                    <input name="description" id="description" type="text"/>

                    <div class="form-buttons">
                        <button> Cancel </button>
                        <input name="submit" type="submit" value="Save" class="submit-button" />
                    </div>
                </form>
        {/if}
    </div>
</div>

<style>
 .sidebar :last-child {
     margin-bottom: 40px;
 }
</style>
