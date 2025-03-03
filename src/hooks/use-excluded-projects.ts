import { useState, useEffect } from 'react';

const STORAGE_KEY = 'excluded_projects';

export const useExcludedProjects = () => {
    const [excludedProjects, setExcludedProjects] = useState<string[]>(() => {
        // Initialize from localStorage if available, otherwise empty array
        if (typeof window !== 'undefined') {
            const stored = localStorage.getItem(STORAGE_KEY);
            return stored ? JSON.parse(stored) : [];
        }
        return [];
    });

    useEffect(() => {
        // Save to localStorage whenever excludedProjects changes
        localStorage.setItem(STORAGE_KEY, JSON.stringify(excludedProjects));
    }, [excludedProjects]);

    return [excludedProjects, setExcludedProjects] as const;
}; 