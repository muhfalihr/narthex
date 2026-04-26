<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>404 Not Found - Narthex</title>
</svelte:head>

<div class="min-h-[80vh] flex items-center justify-center p-4">
	<div class="max-w-md w-full bg-zinc-900 border border-zinc-800 rounded-2xl p-8 text-center space-y-6 shadow-xl">
		<div class="flex justify-center">
			<div class="w-20 h-20 bg-orange-500/10 rounded-full flex items-center justify-center border border-orange-500/20">
				<span class="text-3xl font-black text-orange-500 tracking-tighter">404</span>
			</div>
		</div>
		
		<div class="space-y-2">
			<h1 class="text-3xl font-extrabold text-white tracking-tight">Page Not Found</h1>
			<p class="text-zinc-400 text-sm">
				The page <code class="text-zinc-300 font-mono px-1 py-0.5 bg-zinc-950 rounded">{$page.url.pathname}</code> does not exist or has been moved.
			</p>
		</div>

		<div class="pt-4 border-t border-zinc-800/50">
			<a
				href="/"
				class="inline-flex items-center justify-center gap-2 bg-orange-500 text-white px-6 py-2.5 rounded-md font-semibold text-sm hover:bg-orange-400 transition-all hover:scale-105 active:scale-95 shadow-lg shadow-orange-500/20"
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
				Back to Dashboard
			</a>
		</div>
	</div>
</div>