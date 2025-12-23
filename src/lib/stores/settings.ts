import { writable } from 'svelte/store';

export interface AppSettings {
    repoPath: string;
}

const defaultSettings: AppSettings = {
    repoPath: '',
};

// Create the store
function createSettingsStore() {
    const { subscribe, set, update } = writable<AppSettings>(defaultSettings);

    // Load from localStorage on init
    if (typeof window !== 'undefined') {
        const saved = localStorage.getItem('blog-manager-settings');
        if (saved) {
            try {
                set(JSON.parse(saved));
            } catch {
                // Invalid JSON, use defaults
            }
        }
    }

    return {
        subscribe,
        set: (value: AppSettings) => {
            set(value);
            if (typeof window !== 'undefined') {
                localStorage.setItem('blog-manager-settings', JSON.stringify(value));
            }
        },
        update: (updater: (value: AppSettings) => AppSettings) => {
            update((current) => {
                const next = updater(current);
                if (typeof window !== 'undefined') {
                    localStorage.setItem('blog-manager-settings', JSON.stringify(next));
                }
                return next;
            });
        },
        setRepoPath: (path: string) => {
            update((s) => {
                const next = { ...s, repoPath: path };
                if (typeof window !== 'undefined') {
                    localStorage.setItem('blog-manager-settings', JSON.stringify(next));
                }
                return next;
            });
        },
    };
}

export const settings = createSettingsStore();
