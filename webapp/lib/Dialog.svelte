<script>
import Toggle from '$lib/Toggle.svelte';
import IconsGallery from './IconsGallery.svelte';

/** @type {boolean} */
let iconSearch = false;

</script>

<div class="dialog flex min-w-80 max-w-screen-md shadow-md border-1">
    <div class="sidebar grid grid-cols-[64px_1fr] justify-items-start bg-slate-500 rounded-tl-lg rounded-bl-lg p-2">
        <button
            class="self-start icon w-14 h-14 border-2 border-rose-500"
            on:click="{() => iconSearch = !iconSearch}" />
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
    <div class="details bg-white border-1 rounded-tr-lg rounded-br-lg p-4">
        {#if iconSearch}
            <div>
                <IconsGallery />
            </div>
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
 .details {
     min-width: 410px;
     min-height: 270px;
 }
</style>
