<script lang="ts">
	import './layout.css';
	import { page } from '$app/state';
	let { children, data } = $props();

	const isLoginPage = $derived(page.url.pathname === '/login');
</script>

<div class="flex min-h-screen bg-zinc-950">
	{#if !isLoginPage}
		<!-- Sidebar -->
		<aside class="w-64 border-r border-zinc-800 bg-zinc-900/50 backdrop-blur-sm">
			<div class="flex h-16 items-center border-b border-zinc-800 px-6">
				<a href="/" class="flex items-center gap-2 text-xl font-bold tracking-tight text-white">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="text-orange-500"
						><path
							d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"
						/></svg
					>
					Narthex
				</a>
			</div>
			<nav class="flex flex-col h-[calc(100vh-64px)] p-4">
				<a
					href="/"
					class="flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-zinc-300 transition-colors hover:bg-zinc-800 hover:text-white"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><rect width="7" height="7" x="3" y="3" rx="1" /><rect
							width="7"
							height="7"
							x="14"
							y="3"
							rx="1"
						/><rect width="7" height="7" x="14" y="14" rx="1" /><rect
							width="7"
							height="7"
							x="3"
							y="14"
							rx="1"
						/></svg
					>
					Dashboard
				</a>

				{#if data.authEnabled}
					<form action="/logout" method="POST" class="mt-auto pt-4">
						<button
							type="submit"
							class="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="18"
								height="18"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><polyline
									points="16 17 21 12 16 7"
								/><line x1="21" x2="9" y1="12" y2="12" /></svg
							>
							Logout
						</button>
					</form>
				{/if}
			</nav>
		</aside>

		<!-- Main Content -->
		<main class="flex-1 overflow-y-auto">
			<header
				class="sticky top-0 z-10 h-16 border-b border-zinc-800 bg-zinc-950/50 backdrop-blur-sm"
			>
				<div class="flex h-full items-center justify-between px-8">
					<div class="flex items-center gap-4">
						<span class="text-zinc-500">/</span>
						<span class="text-base font-medium text-zinc-200">Overview</span>
					</div>
					<div class="flex items-center gap-4">
						<a
							href="https://prometheus.io/docs/prometheus/latest/http_sd/"
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="18"
								height="18"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-book-open"
								><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" /><path
									d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"
								/></svg
							>
							Docs
						</a>
						<a
							href="https://github.com/muhfalihr/narthex.git"
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="18"
								height="18"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-github"
								><path
									d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
								/><path d="M9 18c-4.51 2-5-2-7-2" /></svg
							>
							Source
						</a>
					</div>
				</div>
			</header>
			<div class="p-8">
				{@render children()}
			</div>
		</main>
	{:else}
		<main class="flex flex-1 items-center justify-center">
			{@render children()}
		</main>
	{/if}
</div>
