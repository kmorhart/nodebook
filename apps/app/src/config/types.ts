export type Config = {
    darkMode: boolean;
    toolbarCategory?: string;
}

export type ConfigContextValue = {
  config: Config;
  toggleDarkMode: () => void;
  setToolbarCategory: (category: string) => void;
  updateConfig: (partial: Partial<Config>) => void;
};

export type ToolbarState = {
    [key: string]: {
        [key: string]: string
    }
}

export type Me = {
    uuid: String;
    username: String;
    email: String;
    isActive: Boolean;
    isVerified: Boolean;
    createdAt: Date;
}