<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>404 Not Found - Narthex</title>
</svelte:head>

<div class="flex min-h-[80vh] items-center justify-center p-4">
	<div
		class="w-full max-w-md space-y-6 rounded-2xl border border-zinc-800 bg-zinc-900 p-8 text-center shadow-xl"
	>
		<div class="flex justify-center">
			<div
				class="flex h-20 w-20 items-center justify-center rounded-full border border-orange-500/20 bg-orange-500/10"
			>
				<span class="text-3xl font-black tracking-tighter text-orange-500">404</span>
			</div>
		</div>

		<div class="space-y-2">
			<h1 class="text-3xl font-extrabold tracking-tight text-white">Page Not Found</h1>
			<p class="text-sm text-zinc-400">
				The page <code class="rounded bg-zinc-950 px-1 py-0.5 font-mono text-zinc-300"
					>{$page.url.pathname}</code
				> does not exist or has been moved.
			</p>
		</div>

		<div class="border-t border-zinc-800/50 pt-4">
			<a
				href="/"
				class="inline-flex items-center justify-center gap-2 rounded-md bg-orange-500 px-6 py-2.5 text-sm font-semibold text-white shadow-lg shadow-orange-500/20 transition-all hover:scale-105 hover:bg-orange-400 active:scale-95"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"><path d="m15 18-6-6 6-6" /></svg
				>
				Back to Dashboard
			</a>
		</div>
	</div>
</div>
