<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import { enhance } from '$app/forms';
	import { API_BASE } from '$lib/api';

	let { data, form }: { data: PageData; form: ActionData } = $props();

	let group = $derived(data.group);
	let targets = $derived(data.targets);
	let labels = $derived(data.labels);

	let showEditGroup = $state(false);
	let showAddTarget = $state(false);
	let showManageLabels = $state(false);
	let copiedUrl = $state(false);

	$effect(() => {
		if (form?.updateGroup && 'success' in form.updateGroup) showEditGroup = false;
		if (form?.addTarget && 'success' in form.addTarget) showAddTarget = false;
		if (form?.upsertLabel && 'success' in form.upsertLabel) showManageLabels = false;
	});

	function copyToClipboard() {
		navigator.clipboard.writeText(`${API_BASE}/targets?group_id=${group?.id}`);
		copiedUrl = true;
		setTimeout(() => copiedUrl = false, 2000);
	}
</script>

{#if data.error || !group}
	<div class="space-y-4">
		<a href="/" class="text-sm text-zinc-500 hover:text-white flex items-center gap-2">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
			Back to Dashboard
		</a>
		<div class="p-4 bg-red-900/20 border border-red-900/50 rounded-lg text-red-400 text-sm">
			{data.error || 'Group not found'}
		</div>
	</div>
{:else}
	<div class="space-y-8">
		<header class="space-y-4">
			<a href="/" class="text-sm text-zinc-500 hover:text-white flex items-center gap-2 w-fit transition-colors">
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
				Back to Dashboard
			</a>
			<div class="flex items-end justify-between">
				<div>
					<h1 class="text-3xl font-bold text-white tracking-tight">{group.name}</h1>
					<p class="text-zinc-500 mt-1">{group.description || 'No description provided.'}</p>
				</div>
				<div class="flex items-center gap-3">
					<button onclick={() => showEditGroup = !showEditGroup} class="px-4 py-2 rounded-md text-sm font-medium border border-zinc-800 text-zinc-400 hover:text-white hover:bg-zinc-800 transition-all">
						{showEditGroup ? 'Cancel Edit' : 'Edit Group'}
					</button>
					<form method="POST" action="?/deleteGroup" use:enhance onsubmit={(e) => { if(!confirm('Are you sure?')) e.preventDefault(); }}>
						<button type="submit" class="px-4 py-2 rounded-md text-sm font-medium bg-red-950/30 text-red-400 border border-red-900/30 hover:bg-red-900/40 transition-all">
							Delete
						</button>
					</form>
				</div>
			</div>
			
			{#if showEditGroup}
				<form method="POST" action="?/updateGroup" use:enhance class="bg-zinc-900 border border-zinc-800 p-6 rounded-xl space-y-4">
					<div>
						<label for="name" class="block text-sm font-medium text-zinc-300 mb-1">Name</label>
						<input type="text" id="name" name="name" value={group.name} required class="w-full bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" />
					</div>
					<div>
						<label for="description" class="block text-sm font-medium text-zinc-300 mb-1">Description</label>
						<input type="text" id="description" name="description" value={group.description || ''} class="w-full bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" />
					</div>
					<div class="flex justify-end">
						<button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md font-semibold text-sm hover:bg-orange-400 transition-colors">
							Save Changes
						</button>
					</div>
				</form>
			{/if}
		</header>

		<div class="bg-zinc-900 border border-zinc-800 p-6 rounded-2xl flex items-center justify-between">
			<div>
				<h2 class="text-sm font-semibold text-white">Group SD URL</h2>
				<p class="text-zinc-400 text-sm mt-1">Use this URL to discover targets specifically for this group.</p>
			</div>
			<div class="flex items-center gap-2">
				<code class="px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-md text-orange-400 text-sm font-mono select-all">{API_BASE}/targets?group_id={group.id}</code>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<!-- Targets Section -->
			<section class="bg-zinc-900 border border-zinc-800 rounded-xl overflow-hidden">
				<div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between bg-zinc-900/50">
					<h2 class="text-xs font-bold text-zinc-400 uppercase tracking-widest">Targets</h2>
					<button onclick={() => showAddTarget = !showAddTarget} class="text-xs font-bold text-orange-400 hover:text-orange-300">
						{showAddTarget ? 'Cancel' : 'Add Target'}
					</button>
				</div>
				
				{#if showAddTarget}
					<div class="p-6 border-b border-zinc-800 bg-zinc-900/80">
						<form method="POST" action="?/addTarget" use:enhance class="flex gap-3">
							<input type="text" name="address" required placeholder="192.168.1.100:8080" class="flex-1 bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" />
							<button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md font-semibold text-sm hover:bg-orange-400 transition-colors">
								Add
							</button>
						</form>
					</div>
				{/if}

				<div class="divide-y divide-zinc-800">
					{#each targets as target}
						<div class="px-6 py-4 flex items-center justify-between group/item">
							<code class="text-sm text-zinc-300 font-mono">{target.address}</code>
							<form method="POST" action="?/removeTarget" use:enhance>
								<input type="hidden" name="address" value={target.address} />
								<button type="submit" aria-label="Delete target" class="opacity-0 group-hover/item:opacity-100 p-1.5 text-zinc-500 hover:text-red-400 transition-all">
									<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"></path><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path></svg>
								</button>
							</form>
						</div>
					{:else}
						<div class="px-6 py-12 text-center text-zinc-500 text-sm italic">
							No targets added to this group yet.
						</div>
					{/each}
				</div>
			</section>

			<!-- Labels Section -->
			<section class="bg-zinc-900 border border-zinc-800 rounded-xl overflow-hidden">
				<div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between bg-zinc-900/50">
					<h2 class="text-sm font-semibold text-white uppercase tracking-wider">Labels</h2>
					<button onclick={() => showManageLabels = !showManageLabels} class="text-xs font-bold text-orange-400 hover:text-orange-300">
						{showManageLabels ? 'Cancel' : 'Add Label'}
					</button>
				</div>
				
				{#if showManageLabels}
					<div class="p-6 border-b border-zinc-800 bg-zinc-900/80">
						<form method="POST" action="?/upsertLabel" use:enhance class="flex gap-3">
							<input type="text" name="key" required placeholder="env" class="w-1/3 bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" />
							<input type="text" name="value" required placeholder="production" class="flex-1 bg-zinc-950 border border-zinc-800 rounded-md px-3 py-2 text-white focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-sm" />
							<button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md font-semibold text-sm hover:bg-orange-400 transition-colors">
								Add
							</button>
						</form>
					</div>
				{/if}

				<div class="p-6">
					<div class="flex flex-wrap gap-2">
						{#each labels as label}
							<div class="flex items-center rounded-md overflow-hidden border border-zinc-700 text-xs group/label">
								<span class="bg-zinc-800 px-2 py-1.5 text-zinc-400 border-r border-zinc-700">{label.key}</span>
								<span class="bg-zinc-900 px-2 py-1.5 text-zinc-200">{label.value}</span>
								<form method="POST" action="?/removeLabel" use:enhance class="bg-zinc-900 border-l border-zinc-700 h-full flex">
									<input type="hidden" name="key" value={label.key} />
									<button type="submit" class="px-2 text-zinc-500 hover:text-red-400 transition-colors bg-zinc-800 hover:bg-zinc-700 h-full">
										&times;
									</button>
								</form>
							</div>
						{:else}
							<div class="w-full text-center text-zinc-500 text-sm italic py-6">
								No labels defined for this group.
							</div>
						{/each}
					</div>
				</div>
			</section>
		</div>

		<!-- Advanced Info -->
		<section class="bg-zinc-900 border border-zinc-800 rounded-xl p-6">
			<h2 class="text-sm font-semibold text-white uppercase tracking-wider mb-4">Discovery Metadata</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div>
					<span class="text-xs text-zinc-500 block mb-1">Group ID</span>
					<code class="text-xs text-zinc-300 font-mono break-all">{group.id}</code>
				</div>
				<div>
					<span class="text-xs text-zinc-500 block mb-1">Created At</span>
					<span class="text-xs text-zinc-300">{new Date(group.created_at).toLocaleString()}</span>
				</div>
				<div>
					<span class="text-xs text-zinc-500 block mb-1">Last Modified</span>
					<span class="text-xs text-zinc-300">{new Date(group.updated_at).toLocaleString()}</span>
				</div>
			</div>
		</section>
	</div>
{/if}
