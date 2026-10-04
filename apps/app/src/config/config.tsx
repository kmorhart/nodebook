import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';
import type { Config, ConfigContextValue } from './types';

const ConfigContext = createContext<ConfigContextValue | undefined>(undefined);

const prefersDarkMode = (window.matchMedia('(prefers-color-scheme: dark)')).matches;

const STORAGE_KEY = 'config';
const DEFAULT_CONFIG: Config = { darkMode: prefersDarkMode, toolbarCategory: 'file' };

function loadConfig(): Config {
    
    try {
        const raw = localStorage.getItem(STORAGE_KEY);

        return raw ? { ...DEFAULT_CONFIG, ...JSON.parse(raw) } : DEFAULT_CONFIG;
    } catch {
        return DEFAULT_CONFIG;
    }
}

export function ConfigProvider({ children }: { children: ReactNode }) {
    const [config, setConfig] = useState<Config>(loadConfig);

    useEffect(() => {
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
        } catch (err) {
            console.error('Failed to save config:', err);
        }
    }, [config]);

    const toggleDarkMode = () =>
        setConfig((prev) => {
            document.documentElement.setAttribute('data-theme', prev.darkMode ? 'light' : 'dark');

            return { ...prev, darkMode: !prev.darkMode };
        });

    const setToolbarCategory = (category: string) =>
        setConfig((prev) => ({ ...prev, toolbarCategory: category }));

    const updateConfig = (partial: Partial<Config>) =>
        setConfig((prev) => ({ ...prev, ...partial }));

    return (
        <ConfigContext.Provider value={{ config, toggleDarkMode, setToolbarCategory, updateConfig }}>
            {children}
        </ConfigContext.Provider>
    );
}

export function useConfig(): ConfigContextValue {
    const ctx = useContext(ConfigContext);

    if (!ctx) throw new Error('useConfig must be used within a ConfigProvider');

    return ctx;
}