<script>
import SessionButton from '$lib/SessionButton.svelte';
import SecurityContext from '$lib/SecurityContext.svelte';
</script>

<SecurityContext>
    <header>
        <nav id="top-nav">
            <div class="flex items-center header p-2">
                <div class="flex-1">
                </div>
                <div class="text-slate-500">
                    <SessionButton />
                </div>
            </div>
        </nav>
    </header>
    <main class="flex-auto flex items-center justify-center">
        <slot />
    </main>
</SecurityContext>
