import { handleCallback } from '$lib/auth';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url }) => {
    const code = url.searchParams.get('code');
    const state = url.searchParams.get('state');

    if (!code || !state) {
        throw redirect(303, '/');
    }

    const success = await handleCallback(code, state);
    throw redirect(303, success ? '/' : '/');
};
