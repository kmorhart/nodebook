import * as Icons from '../components/icons'

export const TOOLBAR_CONFIG = [
    {
        id: "file",
        name: "File",
        tools: [
            {
                id: "new",
                name: "New",
                states: ["enabled"],
                icons: [Icons.FilePlus]
            },
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