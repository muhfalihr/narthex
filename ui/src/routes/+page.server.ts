import { fetchGroups, createGroup } from '$lib/api';
import type { PageServerLoad, Actions } from './$types';
import { fail } from '@sveltejs/kit';
import { logger } from '$lib/logger';

export const load: PageServerLoad = async ({ fetch }) => {
    try {
        const groups = await fetchGroups(fetch);
        return {
            groups
        };
    } catch (e) {
        logger.error({ err: e }, 'Error in load');
        return {
            groups: [],
            error: 'Failed to load target groups'
        };
    }
};

export const actions: Actions = {
    create: async ({ request, fetch }) => {
        const data = await request.formData();
        const name = data.get('name')?.toString();
        const description = data.get('description')?.toString() || null;

        if (!name) {
            return fail(400, { name, missing: true });
        }

        try {
            await createGroup(name, description, fetch);
            return { success: true };
        } catch (e) {
            logger.error({ err: e }, 'Failed to create group');
            return fail(500, { error: 'Failed to create group' });
        }
    }
};
