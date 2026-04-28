<script lang="ts">
	import { enhance } from '$app/forms';
	let { form } = $props();
</script>

<div class="w-full max-w-md space-y-8 rounded-xl border border-zinc-800 bg-zinc-900/50 p-8 backdrop-blur-sm">
	<div class="text-center">
		<div class="flex justify-center">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
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
		</div>
		<h2 class="mt-6 text-3xl font-bold tracking-tight text-white">Sign in to Narthex</h2>
		<p class="mt-2 text-sm text-zinc-400">Enter your credentials to access the dashboard</p>
	</div>

	<form class="mt-8 space-y-6" method="POST" use:enhance>
		<div class="space-y-4 rounded-md shadow-sm">
			<div>
				<label for="username" class="block text-sm font-medium text-zinc-300">Username</label>
				<input
					id="username"
					name="username"
					type="text"
					required
					class="mt-1 block w-full rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-zinc-200 placeholder-zinc-500 focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 sm:text-sm"
					placeholder="admin"
				/>
			</div>
			<div>
				<label for="password" class="block text-sm font-medium text-zinc-300">Password</label>
				<input
					id="password"
					name="password"
					type="password"
					required
					class="mt-1 block w-full rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-zinc-200 placeholder-zinc-500 focus:border-orange-500 focus:outline-none focus:ring-1 focus:ring-orange-500 sm:text-sm"
					placeholder="••••••••"
				/>
			</div>
		</div>

		{#if form?.error}
			<div class="rounded-md bg-red-900/20 p-4 border border-red-900/50">
				<div class="flex">
					<div class="flex-shrink-0">
						<svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
						</svg>
					</div>
					<div class="ml-3">
						<p class="text-sm font-medium text-red-400">{form.error}</p>
					</div>
				</div>
			</div>
		{/if}

		<div>
			<button
				type="submit"
				class="group relative flex w-full justify-center rounded-md border border-transparent bg-orange-600 py-2 px-4 text-sm font-medium text-white transition-colors hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
			>
				Sign in
			</button>
		</div>
	</form>
</div>
