import { logger } from '$lib/logger';
import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

export const handle: Handle = async ({ event, resolve }) => {
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
