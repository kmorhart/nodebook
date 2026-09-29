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
                icon: (config: Config): React.ReactNode => {
                    return <Icons.FilePlus color={config.darkMode ? "#fff" : "#000"} />
                },
                action: () => {}
            },
            {
                id: "open",
                name: "Open from File",
                icon: (config: Config): React.ReactNode => {
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
                icon: (config: Config): React.ReactNode => {
                    return <Icons.SavePlus color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "share",
                name: "Share",
                states: ["default"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Share2 color={config.darkMode ? "#fff" : "#000"} />
                }
            }
        ]
    },
    {
        id: "edit",
        name: "Edit",
        tools: [
            {
                id: "undo",
                name: "Undo",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Undo color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "redo",
                name: "Redo",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Redo color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "selectall",
                name: "Select All",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.SquareDashed color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "deselect",
                name: "Deselect",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.SquareDashedX color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "cut",
                name: "Cut",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Scissors color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "copy",
                name: "Copy",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.ClipboardCopy color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "paste",
                name: "Paste",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.ClipboardPaste color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "delete",
                name: "Delete",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.X color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "deleteall",
                name: "Clear",
                states: ["available", "unavailable"],
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Trash color={config.darkMode ? "#fff" : "#000"} />
                }
            }
        ]
    },
    {
        id: "view",
        name: "View",
        tools: [
            {
                id: "zoomin",
                name: "Zoom In",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.ZoomIn color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "zoomout",
                name: "Zoom Out",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.ZoomOut color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "autoalign",
                name: "Auto Align",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.LayoutFreeform color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "spline",
                name: "Spline Connections",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Spline color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "highlight",
                name: "Highlight Connections",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Spotlight color={config.darkMode ? "#fff" : "#000"} />
                }
            }
        ]
    },
    {
        id: "run",
        name: "Run",
        tools: [
            {
                id: "runonce",
                name: "Run Once",
                icon: (config: Config): React.ReactNode => {
                    return <Icons.Play color={config.darkMode ? "#fff" : "#000"} />
                }
            },
            {
                id: "runcontinuous",
                name: "Run Continuously",
                states: ["running", "paused"],
                icon: (config: Config, state: string): React.ReactNode => {
                    return state === "running" ? <Icons.MonitorPlay color={config.darkMode ? "#fff" : "#000"} /> : <Icons.MonitorPause color={config.darkMode ? "#fff" : "#000"} />
                }
            }
        ]
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