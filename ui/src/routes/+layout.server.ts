import { env } from '$env/dynamic/private';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	return {
		authEnabled: !!(env.APP_USERNAME && env.APP_PASSWORD)
	};
};
