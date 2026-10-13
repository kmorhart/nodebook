import * as Icons from '../components/icons'
import type { Config, ToolbarState } from './types'

export const TOOLBAR_CONFIG = [
    {
        id: "file",
        name: "File",
        groups: [
            {
                id: "document",
                name: "Document",
                tools: [
                    {
                        id: "new",
                        element: <span>New</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.FilePlus color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "open",
                        element: <span>Open</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.FileBraces color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "save",
                        element: <span>Save</span>,
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
                        element: <span>Save<br />As</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.SavePlus color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                ]
            },
            {
                id: "sharing",
                name: "Sharing",
                tools: [
                    {
                        id: "share",
                        element: <span>Share</span>,
                        states: ["default"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Share2 color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    }
                ]
            }
        ],
    },
    {
        id: "edit",
        name: "Edit",
        groups: [
            {
                id: "state",
                name: "State",
                tools: [
                    {
                        id: "undo",
                        element: <span>Undo</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Undo color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "redo",
                        element: <span>Redo</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Redo color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "selectall",
                        element: <span>Select<br />All</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.SquareDashed color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "deselect",
                        element: <span>De-<br />select</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.SquareDashedX color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                ]
            },
            {
                id: "clipboard",
                name: "Clipboard",
                tools: [
                    {
                        id: "cut",
                        element: <span>Cut</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Scissors color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "copy",
                        element: <span>Copy</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.ClipboardCopy color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "paste",
                        element: <span>Paste</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.ClipboardPaste color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                ]
            },
            {
                id: "delete",
                name: "Delete",
                tools: [
                    {
                        id: "delete",
                        element: <span>Delete</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.X color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "deleteall",
                        element: <span>Clear</span>,
                        states: ["available", "unavailable"],
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Trash color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    }
                ]
            }
        ]
    },
    {
        id: "run",
        name: "Run",
        groups: [
            {
                id: "execution",
                name: "Execution",
                tools: [
                    {
                        id: "runonce",
                        element: <span>Run<br />Once</span>,
                        icon: (config: Config): React.ReactNode => {
                            return <Icons.Play color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    },
                    {
                        id: "runcontinuous",
                        element: <span>Run<br />Continuously</span>,
                        states: ["running", "paused"],
                        icon: (config: Config, state: string): React.ReactNode => {
                            return state === "running" ? <Icons.MonitorPlay color={config.darkMode ? "#fff" : "#000"} /> : <Icons.MonitorPause color={config.darkMode ? "#fff" : "#000"} />
                        },
                        action: () => {}
                    }
                ]
            }
        ]
    },
    {
        id: "tools",
        name: "Tools",
        groups: []
    },
    {
        id: "help",
        name: "Help",
        groups: []
    }
]