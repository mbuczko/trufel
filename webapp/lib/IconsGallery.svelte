<script>
import 'material-icons/iconfont/material-icons.css';
import TabsBar from './TabsBar.svelte';
import Tab from './Tab.svelte';

function onTabChange({id}) {
    console.log('Changed', id);
}
</script>

<div class="icons-gallery">
    <TabsBar on:change="{(e) => onTabChange(e.detail)}">
        <Tab id="outlined" label="Outlined"/>
        <Tab id="round" label="Round"/>
        <Tab id="sharp" label="Sharp"/>
        <Tab id="two-tone" label="Two-tone"/>
    </TabsBar>
    <span class="material-icons">pie_chart</span>
</div>
