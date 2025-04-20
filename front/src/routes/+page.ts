export const prerender = false;
export const ssr = false;

export async function load() {
  // No longer need to load sim data on the homepage
  return {};
}
