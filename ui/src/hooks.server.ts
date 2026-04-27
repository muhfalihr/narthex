import { logger } from '$lib/logger';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    const start = Date.now();
    
    const response = await resolve(event);
    
    const duration = Date.now() - start;
    
    logger.info({
        method: event.request.method,
        url: event.url.pathname,
        status: response.status,
        duration: `${duration}ms`,
        ip: event.getClientAddress()
    }, 'request completed');
    
    return response;
};
