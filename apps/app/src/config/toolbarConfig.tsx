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
                        id: "open",
                        element: <span>Open</span>,
                        icon: (): React.ReactNode => {
                            return <Icons.FileBraces />
                        },
                        action: () => {}
                    },
                    {
                        id: "save",
                        element: <span>Save</span>,
                        states: ["unsaved", "saved"],
                        icon: (_config: Config, state: string): React.ReactNode => {
                            return state === "unsaved" ? <Icons.SavePen /> : <Icons.Save  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.SavePlus  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.Share2  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.Undo  />
                        },
                        action: () => {}
                    },
                    {
                        id: "redo",
                        element: <span>Redo</span>,
                        states: ["available", "unavailable"],
                        icon: ( ): React.ReactNode => {
                            return <Icons.Redo  />
                        },
                        action: () => {}
                    },
                    {
                        id: "selectall",
                        element: <span>Select<br />All</span>,
                        icon: ( ): React.ReactNode => {
                            return <Icons.SquareDashed  />
                        },
                        action: () => {}
                    },
                    {
                        id: "deselect",
                        element: <span>De-<br />select</span>,
                        icon: ( ): React.ReactNode => {
                            return <Icons.SquareDashedX  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.Scissors  />
                        },
                        action: () => {}
                    },
                    {
                        id: "copy",
                        element: <span>Copy</span>,
                        states: ["available", "unavailable"],
                        icon: ( ): React.ReactNode => {
                            return <Icons.ClipboardCopy  />
                        },
                        action: () => {}
                    },
                    {
                        id: "paste",
                        element: <span>Paste</span>,
                        states: ["available", "unavailable"],
                        icon: ( ): React.ReactNode => {
                            return <Icons.ClipboardPaste  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.X  />
                        },
                        action: () => {}
                    },
                    {
                        id: "deleteall",
                        element: <span>Clear</span>,
                        states: ["available", "unavailable"],
                        icon: ( ): React.ReactNode => {
                            return <Icons.Trash  />
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
                        icon: ( ): React.ReactNode => {
                            return <Icons.Play  />
                        },
                        action: () => {}
                    },
                    {
                        id: "runcontinuous",
                        element: <span>Run<br />Continuously</span>,
                        states: ["running", "paused"],
                        icon: (_config: Config, state: string): React.ReactNode => {
                            return state === "running" ? <Icons.MonitorPlay  /> : <Icons.MonitorPause  />
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