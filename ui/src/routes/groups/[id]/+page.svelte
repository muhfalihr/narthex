<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import { enhance } from '$app/forms';

	let { data, form }: { data: PageData; form: ActionData } = $props();
	let group = $derived(data.group);
	let targets = $derived(data.targets);
	let labels = $derived(data.labels);

	let showEditGroup = $state(false);
	let showAddTarget = $state(false);
	let showManageLabels = $state(false);
	let showDeleteConfirm = $state(false);

	$effect(() => {
		if (form?.updateGroup && 'success' in form.updateGroup) showEditGroup = false;
		if (form?.addTarget && 'success' in form.addTarget) showAddTarget = false;
		if (form?.upsertLabel && 'success' in form.upsertLabel) showManageLabels = false;
	});

	let deleteForm = $state<HTMLFormElement>();
</script>

{#if data.error || !group}
	<div class="space-y-4">
		<a href="/" class="flex items-center gap-2 text-sm text-zinc-500 hover:text-white">
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
		<div class="rounded-lg border border-red-900/50 bg-red-900/20 p-4 text-sm text-red-400">
			{data.error || 'Group not found'}
		</div>
	</div>
{:else}
	<div class="space-y-8">
		<header class="space-y-4">
			<a
				href="/"
				class="flex w-fit items-center gap-2 text-sm text-zinc-500 transition-colors hover:text-white"
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
			<div class="flex items-end justify-between">
				<div>
					<h1 class="text-3xl font-bold tracking-tight text-white">{group.name}</h1>
					<p class="mt-1 text-zinc-500">{group.description || 'No description provided.'}</p>
				</div>
				<div class="flex items-center gap-3">
					<button
						onclick={() => (showEditGroup = !showEditGroup)}
						class="rounded-md border border-zinc-800 px-4 py-2 text-sm font-medium text-zinc-400 transition-all hover:bg-zinc-800 hover:text-white"
					>
						{showEditGroup ? 'Cancel Edit' : 'Edit Group'}
					</button>
					<form bind:this={deleteForm} method="POST" action="?/deleteGroup" use:enhance>
						<button
							type="button"
							onclick={() => (showDeleteConfirm = true)}
							class="rounded-md border border-red-900/30 bg-red-950/30 px-4 py-2 text-sm font-medium text-red-400 transition-all hover:bg-red-900/40"
						>
							Delete
						</button>
					</form>
				</div>
			</div>

			{#if showEditGroup}
				<form
					method="POST"
					action="?/updateGroup"
					use:enhance
					class="space-y-4 rounded-xl border border-zinc-800 bg-zinc-900 p-6"
				>
					<div>
						<label for="name" class="mb-1 block text-sm font-medium text-zinc-300">Name</label>
						<input
							type="text"
							id="name"
							name="name"
							value={group.name}
							required
							class="w-full rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-white focus:border-orange-500 focus:ring-1 focus:ring-orange-500 focus:outline-none"
						/>
					</div>
					<div>
						<label for="description" class="mb-1 block text-sm font-medium text-zinc-300"
							>Description</label
						>
						<input
							type="text"
							id="description"
							name="description"
							value={group.description || ''}
							class="w-full rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-white focus:border-orange-500 focus:ring-1 focus:ring-orange-500 focus:outline-none"
						/>
					</div>
					<div class="flex justify-end">
						<button
							type="submit"
							class="rounded-md bg-orange-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-orange-400"
						>
							Save Changes
						</button>
					</div>
				</form>
			{/if}
		</header>

		<div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
			<!-- Targets Section -->
			<section class="overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900">
				<div
					class="flex items-center justify-between border-b border-zinc-800 bg-zinc-900/50 px-6 py-4"
				>
					<h2 class="text-xs font-bold tracking-widest text-zinc-400 uppercase">Targets</h2>
					<button
						onclick={() => (showAddTarget = !showAddTarget)}
						class="text-xs font-bold text-orange-400 hover:text-orange-300"
					>
						{showAddTarget ? 'Cancel' : 'Add Target'}
					</button>
				</div>

				{#if showAddTarget}
					<div class="border-b border-zinc-800 bg-zinc-900/80 p-6">
						<form method="POST" action="?/addTarget" use:enhance class="flex gap-3">
							<input
								type="text"
								name="address"
								required
								placeholder="192.168.1.100:8080"
								class="flex-1 rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-white focus:border-orange-500 focus:ring-1 focus:ring-orange-500 focus:outline-none"
							/>
							<button
								type="submit"
								class="rounded-md bg-orange-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-orange-400"
							>
								Add
							</button>
						</form>
					</div>
				{/if}

				<div class="divide-y divide-zinc-800">
					{#each targets as target}
						<div class="group/item flex items-center justify-between px-6 py-4">
							<code class="font-mono text-sm text-zinc-300">{target.address}</code>
							<form method="POST" action="?/removeTarget" use:enhance>
								<input type="hidden" name="address" value={target.address} />
								<button
									type="submit"
									aria-label="Delete target"
									class="p-1.5 text-zinc-500 opacity-0 transition-all group-hover/item:opacity-100 hover:text-red-400"
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="14"
										height="14"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										><path d="M3 6h18"></path><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"
										></path><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path></svg
									>
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
			<section class="overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900">
				<div
					class="flex items-center justify-between border-b border-zinc-800 bg-zinc-900/50 px-6 py-4"
				>
					<h2 class="text-sm font-semibold tracking-wider text-white uppercase">Labels</h2>
					<button
						onclick={() => (showManageLabels = !showManageLabels)}
						class="text-xs font-bold text-orange-400 hover:text-orange-300"
					>
						{showManageLabels ? 'Cancel' : 'Add Label'}
					</button>
				</div>

				{#if showManageLabels}
					<div class="border-b border-zinc-800 bg-zinc-900/80 p-6">
						<form method="POST" action="?/upsertLabel" use:enhance class="flex gap-3">
							<input
								type="text"
								name="key"
								required
								placeholder="env"
								class="w-1/3 rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-white focus:border-orange-500 focus:ring-1 focus:ring-orange-500 focus:outline-none"
							/>
							<input
								type="text"
								name="value"
								required
								placeholder="production"
								class="flex-1 rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-white focus:border-orange-500 focus:ring-1 focus:ring-orange-500 focus:outline-none"
							/>
							<button
								type="submit"
								class="rounded-md bg-orange-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-orange-400"
							>
								Add
							</button>
						</form>
					</div>
				{/if}

				<div class="p-6">
					<div class="flex flex-wrap gap-2">
						{#each labels as label}
							<div
								class="group/label flex items-center overflow-hidden rounded-md border border-zinc-700 text-xs"
							>
								<span class="border-r border-zinc-700 bg-zinc-800 px-2 py-1.5 text-zinc-400"
									>{label.key}</span
								>
								<span class="bg-zinc-900 px-2 py-1.5 text-zinc-200">{label.value}</span>
								<form
									method="POST"
									action="?/removeLabel"
									use:enhance
									class="flex h-full border-l border-zinc-700 bg-zinc-900"
								>
									<input type="hidden" name="key" value={label.key} />
									<button
										type="submit"
										class="h-full bg-zinc-800 px-2 text-zinc-500 transition-colors hover:bg-zinc-700 hover:text-red-400"
									>
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
		<section class="rounded-xl border border-zinc-800 bg-zinc-900 p-6">
			<h2 class="mb-4 text-sm font-semibold tracking-wider text-white uppercase">
				Discovery Metadata
			</h2>
			<div class="grid grid-cols-1 gap-6 md:grid-cols-3">
				<div>
					<span class="mb-1 block text-xs text-zinc-500">Group ID</span>
					<code class="font-mono text-xs break-all text-zinc-300">{group.id}</code>
				</div>
				<div>
					<span class="mb-1 block text-xs text-zinc-500">Created At</span>
					<span class="text-xs text-zinc-300">{new Date(group.created_at).toLocaleString()}</span>
				</div>
				<div>
					<span class="mb-1 block text-xs text-zinc-500">Last Modified</span>
					<span class="text-xs text-zinc-300">{new Date(group.updated_at).toLocaleString()}</span>
				</div>
			</div>
		</section>
	</div>
{/if}

{#if showDeleteConfirm}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-950/80 p-4 backdrop-blur-sm"
	>
		<div
			class="w-full max-w-md rounded-2xl border border-zinc-800 bg-zinc-900 p-8 shadow-2xl shadow-orange-500/10"
		>
			<div
				class="mb-6 flex h-12 w-12 items-center justify-center rounded-full border border-red-900/50 bg-red-900/20"
			>
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
					class="text-red-400"
					><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
						d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
					/></svg
				>
			</div>
			<h3 class="mb-2 text-xl font-bold text-white">Delete Target Group</h3>
			<p class="mb-8 text-zinc-400">
				Are you sure you want to delete <span class="font-semibold text-white">"{group?.name}"</span
				>? This action cannot be undone and all associated targets and labels will be removed.
			</p>
			<div class="flex gap-3">
				<button
					onclick={() => (showDeleteConfirm = false)}
					class="flex-1 rounded-lg border border-zinc-800 px-4 py-2.5 text-sm font-semibold text-zinc-400 transition-all hover:bg-zinc-800 hover:text-white"
				>
					Cancel
				</button>
				<button
					onclick={() => deleteForm?.requestSubmit()}
					class="flex-1 rounded-lg bg-red-600 px-4 py-2.5 text-sm font-semibold text-white transition-all hover:bg-red-500"
				>
					Confirm Delete
				</button>
			</div>
		</div>
	</div>
{/if}
