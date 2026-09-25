import * as Icons from '../components/icons'
import type { Config, ToolbarState } from './types'

export const TOOLBAR_CONFIG = [
    {
        id: "file",
        name: "File",
        tools: [
            {
                id: "new",
                name: "New",
                icon: (config: Config) => {
                    return <Icons.FilePlus color={config.darkMode ? "#fff" : "#000"} />
                },
                action: () => {}
            },
            {
                id: "open",
                name: "Open from File",
                icon: (config: Config) => {
                    return <Icons.FileBraces color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "save",
                name: "Save",
                states: ["unsaved", "saved"],
                icon: (config: Config, state: string): React.ReactNode => {
                    return state === "unsaved" ? <Icons.SavePen color={config.darkMode ? "#fff" : "#000"} /> : <Icons.Save color={config.darkMode ? "#fff" : "#000"} />
                },  
                action: (setToolbarState: React.Dispatch<React.SetStateAction<ToolbarState>>) => {
                    setToolbarState((prev) => {
                        const newToolbarState = { ...prev };
                        newToolbarState['file']['save'] = 'saved';
                        return newToolbarState;
                    })
                }
            },
            {
                id: "saveas",
                name: "Save As",
                icon: (config: Config, state: string): React.ReactNode => {
                    return <Icons.SavePlus color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "share",
                name: "Share",
                states: ["default"],
                icon: (config: Config, state: string): React.ReactNode => {
                    return <Icons.Share2 color={config.darkMode ? "#fff" : "#000"} />
                }
            }
        ]
    },
    {
        id: "edit",
        name: "Edit",
        tools: []
    },
    {
        id: "view",
        name: "View",
        tools: []
    },
    {
        id: "run",
        name: "Run",
        tools: []
    },
    {
        id: "tools",
        name: "Tools",
        tools: []
    },
    {
        id: "help",
        name: "Help",
        tools: []
    }
]