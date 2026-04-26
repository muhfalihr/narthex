import { fetchGroup, fetchTargets, fetchLabels, updateGroup, deleteGroup, addTarget, removeTarget, upsertLabel, removeLabel } from '$lib/api';
import type { PageServerLoad, Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, fetch }) => {
    const { id } = params;

    try {
        const [group, targets, labels] = await Promise.all([
            fetchGroup(id, fetch),
            fetchTargets(id, fetch),
            fetchLabels(id, fetch)
        ]);

        return {
            group,
            targets,
            labels
        };
    } catch (e) {
        console.error('Error in group load:', e);
        return {
            group: null,
            targets: [],
            labels: [],
            error: 'Failed to load group details'
        };
    }
};

export const actions: Actions = {
    updateGroup: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const name = data.get('name')?.toString();
        const description = data.get('description')?.toString() || null;

        if (!name) return fail(400, { updateGroup: { missingName: true } });
        
        try {
            await updateGroup(params.id, name, description, fetch);
            return { updateGroup: { success: true } };
        } catch (e) {
            return fail(500, { updateGroup: { error: 'Failed to update group' } });
        }
    },
    deleteGroup: async ({ params, fetch }) => {
        try {
            await deleteGroup(params.id, fetch);
        } catch (e) {
            return fail(500, { deleteGroup: { error: 'Failed to delete group' } });
        }
        throw redirect(303, '/');
    },
    addTarget: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const address = data.get('address')?.toString();
        if (!address) return fail(400, { addTarget: { missingAddress: true } });

        try {
            await addTarget(params.id, address, fetch);
            return { addTarget: { success: true } };
        } catch (e) {
            return fail(500, { addTarget: { error: 'Failed to add target' } });
        }
    },
    removeTarget: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const address = data.get('address')?.toString();
        if (!address) return fail(400, { removeTarget: { missingAddress: true } });

        try {
            await removeTarget(params.id, address, fetch);
            return { removeTarget: { success: true } };
        } catch (e) {
            return fail(500, { removeTarget: { error: 'Failed to remove target' } });
        }
    },
    upsertLabel: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const key = data.get('key')?.toString();
        const value = data.get('value')?.toString();
        if (!key || !value) return fail(400, { upsertLabel: { missingLabelData: true } });

        try {
            await upsertLabel(params.id, key, value, fetch);
            return { upsertLabel: { success: true } };
        } catch (e) {
            return fail(500, { upsertLabel: { error: 'Failed to upsert label' } });
        }
    },
    removeLabel: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const key = data.get('key')?.toString();
        if (!key) return fail(400, { removeLabel: { missingKey: true } });

        try {
            await removeLabel(params.id, key, fetch);
            return { removeLabel: { success: true } };
        } catch (e) {
            return fail(500, { removeLabel: { error: 'Failed to remove label' } });
        }
    }
};
