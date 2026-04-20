import { m } from "$lib/paraglide/messages";

// Mapping from message keys to paraglide message functions
// This allows JSON files to use message keys that get resolved to actual localized strings

export function resolveLocaleKey(key: string): () => string {
    const paraglideKey = `paradigm_${key}`;

    // Check if it's a translation function in Paraglide
    if (paraglideKey in m) {
        return (m as any)[paraglideKey];
    }

    // If not found, it's likely a linguistic literal (e.g., "mun", "don")
    // We return a function to keep the API consistent
    return () => key;
}
