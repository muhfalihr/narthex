import { logger } from '$lib/logger';
import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const username = env.APP_USERNAME;
	const password = env.APP_PASSWORD;
	const authEnabled = !!(username && password);

	if (event.url.pathname === '/login') {
		return resolve(event);
	}

	const sessionId = event.cookies.get('session_id');
	const authHeader = event.request.headers.get('Authorization');
	const expectedToken = authEnabled ? btoa(`${username}:${password}`) : null;

	let isAuthorized = !authEnabled;

	if (authEnabled) {
		if (sessionId === expectedToken) {
			isAuthorized = true;
		} else if (authHeader === `Basic ${expectedToken}`) {
			isAuthorized = true;
		}
	}

	// Protect routes
	if (authEnabled && !isAuthorized) {
		logger.warn({ url: event.url.pathname, authEnabled, isAuthorized }, 'Unauthorized access attempt');
		
		// If it's an API request from the browser, return 401
		if (event.url.pathname.startsWith('/api/v1') && event.request.headers.get('accept')?.includes('application/json')) {
			return new Response(JSON.stringify({ error: 'Unauthorized' }), {
				status: 401,
				headers: { 'Content-Type': 'application/json' }
			});
		}
		// For all other requests, redirect to login
		throw redirect(303, '/login');
	}

	// Proxy API requests to the backend
	if (event.url.pathname.startsWith('/api/v1')) {
		const host = env.APP_HOST || '127.0.0.1';
		const port = env.APP_PORT || '3000';
		const backendUrl = `http://${host}:${port}${event.url.pathname}${event.url.search}`;

		try {
			const requestHeaders = new Headers();
			const forbiddenHeaders = ['host', 'connection', 'content-length', 'content-encoding'];

			for (const [key, value] of event.request.headers.entries()) {
				if (!forbiddenHeaders.includes(key.toLowerCase())) {
					requestHeaders.set(key, value);
				}
			}

			// Inject Authorization header if auth is enabled
			if (authEnabled) {
				requestHeaders.set('Authorization', `Basic ${expectedToken}`);
			}

			const response = await fetch(backendUrl, {
				method: event.request.method,
				headers: requestHeaders,
				body: ['GET', 'HEAD', 'DELETE'].includes(event.request.method.toUpperCase())
					? undefined
					: event.request.body,
				// @ts-expect-error - duplex is needed for streaming bodies in node
				duplex: 'half'
			});

			return response;
		} catch (err) {
			logger.error({ err, url: backendUrl }, 'Proxy request failed');
			return new Response(JSON.stringify({ error: 'Backend unreachable' }), {
				status: 502,
				headers: { 'Content-Type': 'application/json' }
			});
		}
	}

	const start = Date.now();
	const response = await resolve(event);
	const duration = Date.now() - start;

	logger.info(
		{
			method: event.request.method,
			url: event.url.pathname,
			status: response.status,
			duration: `${duration}ms`,
			ip: event.getClientAddress()
		},
		'request completed'
	);

	return response;
};
