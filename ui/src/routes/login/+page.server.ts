import { env } from '$env/dynamic/private';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { logger } from '$lib/logger';

export const load: PageServerLoad = async ({ cookies }) => {
	const username = env.APP_USERNAME;
	const password = env.APP_PASSWORD;

	logger.info({ authConfigured: !!(username && password) }, 'Login page load');

	// If auth is not configured, redirect to home
	if (!username || !password) {
		throw redirect(303, '/');
	}

	// If already logged in, redirect to home
	const session = cookies.get('session_id');
	if (session === btoa(`${username}:${password}`)) {
		throw redirect(303, '/');
	}
};

export const actions: Actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const username = data.get('username');
		const password = data.get('password');

		const expectedUsername = env.APP_USERNAME;
		const expectedPassword = env.APP_PASSWORD;

		if (username === expectedUsername && password === expectedPassword) {
			// Set a simple session cookie. 
			// In a real app, use a proper session store or JWT.
			// Here we use base64 encoded credentials as a "token" for simplicity 
			// since it matches what the backend expects for Basic Auth.
			const token = btoa(`${username}:${password}`);
			cookies.set('session_id', token, {
				path: '/',
				httpOnly: true,
				sameSite: 'lax',
				secure: process.env.NODE_ENV === 'production',
				maxAge: 60 * 60 * 24 * 7 // 1 week
			});

			throw redirect(303, '/');
		}

		return fail(400, { error: 'Invalid username or password' });
	}
};
