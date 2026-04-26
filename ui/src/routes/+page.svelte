<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import { enhance } from '$app/forms';
	import { API_BASE } from '$lib/api';

	let { data, form }: { data: PageData; form: ActionData } = $props();
	let showCreateForm = $state(false);
	let copiedUrl = $state(false);

	$effect(() => {
		if (form?.success) {
			showCreateForm = false;
		}
	});

	function copyToClipboard() {
		navigator.clipboard.writeText(`${API_BASE}/targets`);
		copiedUrl = true;
		setTimeout(() => copiedUrl = false, 2000);
	}

	let copiedGroupId = $state<string | null>(null);

	function copyGroupUrl(e: Event, groupId: string) {
		e.preventDefault();
		e.stopPropagation();
		navigator.clipboard.writeText(`${API_BASE}/targets?group_id=${groupId}`);
		copiedGroupId = groupId;
		setTimeout(() => copiedGroupId = null, 2000);
	}
</script>

<div class="space-y-8">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-4xl font-extrabold text-white tracking-tight">Dashboard</h1>
			<p class="text-zinc-400 mt-2 text-base">Manage your service discovery target groups.</p>
		</div>
		<button
			onclick={() => showCreateForm = !showCreateForm}
			class="bg-white text-zinc-950 px-4 py-2 rounded-md font-semibold text-sm hover:bg-zinc-200 transition-colors"
		>
			{showCreateForm ? 'Cancel' : 'Create Group'}
		</button>
	</div>

	<div class="bg-zinc-900 border border-zinc-800 p-6 rounded-2xl flex items-center justify-between">
		<div>
			<h2 class="text-sm font-semibold text-white">Prometheus HTTP SD URL (All Groups)</h2>
			<p class="text-zinc-400 text-sm mt-1">Use this URL in your Prometheus configuration to discover all targets.</p>
		</div>
		<div class="flex items-center gap-2">
			<code class="px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-md text-orange-400 text-sm font-mono select-all">/targets</code>
			<button
				onclick={copyToClipboard}
				class="p-1.5 text-zinc-400 hover:text-white bg-zinc-950 border border-zinc-800 hover:border-zinc-700 rounded-md transition-all flex items-center justify-center relative"
				title="Copy full URL"
				aria-label="Copy full URL"
			>
				{#if copiedUrl}
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-green-400"><polyline points="20 6 9 17 4 12"/></svg>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
				{/if}
			</button>
		</div>
	</div>

	{#if data.error}
		<div class="p-4 bg-red-900/20 border border-red-900/50 rounded-lg text-red-400 text-sm">
			{data.error}
		</div>
	{/if}
	{#if form?.error}
		<div class="p-4 bg-red-900/20 border border-red-900/50 rounded-lg text-red-400 text-sm">
			{form.error}
		</div>
	{/if}

	{#if showCreateForm}
		<form method="POST" action="?/create" use:enhance class="bg-zinc-900 border border-zinc-800 p-6 rounded-xl space-y-4">
			<div>
				<label for="name" class="block text-sm font-medium text-zinc-300 mb-1">Name</label>
				<input type="text" id="name" name="name" required class="w-full bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" placeholder="my-service-group" />
			</div>
			<div>
				<label for="description" class="block text-sm font-medium text-zinc-300 mb-1">Description (Optional)</label>
				<input type="text" id="description" name="description" class="w-full bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" placeholder="Production instances" />
			</div>
			<div class="flex justify-end">
				<button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md font-semibold text-sm hover:bg-orange-400 transition-colors">
					Save Group
				</button>
			</div>
		</form>
	{/if}

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		{#each data.groups as group}
			<a
				href="/groups/{group.id}"
				class="group bg-zinc-900 border border-zinc-800 p-6 rounded-2xl hover:border-zinc-700 hover:bg-zinc-800/50 hover:shadow-lg hover:shadow-orange-500/5 transition-all"
			>
				<div class="flex items-start justify-between mb-4">
					<div class="w-10 h-10 bg-zinc-800 rounded-lg flex items-center justify-center border border-zinc-700 group-hover:border-zinc-600">
						<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-400"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4a2 2 0 0 0 1-1.73z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line></svg>
					</div>
					<div class="text-xs font-mono text-zinc-500">
						{new Date(group.created_at).toLocaleDateString()}
					</div>
				</div>
				<h3 class="text-xl font-bold text-white mb-2">{group.name}</h3>
				<p class="text-sm text-zinc-400 line-clamp-2">
					{group.description || 'No description provided.'}
				</p>
				<div class="mt-6 flex items-center justify-between">
					<div class="flex items-center gap-4 text-xs font-medium text-zinc-500">
						<span class="flex items-center gap-1.5">
							<div class="w-1.5 h-1.5 rounded-full bg-green-500"></div>
							Active
						</span>
						<span class="flex items-center gap-1.5">
							<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>
							Updated {new Date(group.updated_at).toLocaleDateString()}
						</span>
					</div>
					<button
						onclick={(e) => copyGroupUrl(e, group.id)}
						class="p-1.5 text-zinc-500 hover:text-white bg-zinc-900 border border-zinc-800 hover:border-zinc-700 rounded-md transition-all flex items-center justify-center"
						title="Copy Group SD URL"
						aria-label="Copy Group SD URL"
					>
						{#if copiedGroupId === group.id}
							<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-green-400"><polyline points="20 6 9 17 4 12"/></svg>
						{:else}
							<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
						{/if}
					</button>
				</div>
			</a>
		{:else}
			<div class="col-span-full py-12 flex flex-col items-center justify-center border border-dashed border-zinc-800 rounded-xl bg-zinc-900/20">
				<p class="text-zinc-500 text-sm mb-4">No target groups found.</p>
				<button onclick={() => showCreateForm = true} class="text-orange-400 text-sm font-semibold hover:text-orange-300">
					Get started by creating your first group
				</button>
			</div>
		{/each}
	</div>
</div>
