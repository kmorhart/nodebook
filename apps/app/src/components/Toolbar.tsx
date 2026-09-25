import { useState, useMemo, Fragment } from 'react'
import { TOOLBAR_CONFIG } from '../config/toolbarConfig'
import { useConfig } from '../config/config'
import type { ToolbarState } from '../config/types'

function initToolbarState(): ToolbarState {
    const state: ToolbarState = {}
    for (const category of TOOLBAR_CONFIG) {
        state[category.id] = {}
        for (const tool of category.tools) {
            if (tool.states) {
                state[category.id][tool.id] = tool.states[0]
            }
        }
    }
    return state
}

export default function Toolbar() {
    const { config, toggleDarkMode } = useConfig();
    const [toolbarState, setToolbarState] = useState<ToolbarState>(initToolbarState)

    return (
        <header className="toolbar">
            <button onClick={() => toggleDarkMode()}>darkmode</button>

            <div className="toolbar-categories">
                {TOOLBAR_CONFIG.map((category) => (
                    <Fragment key={category.id}>
                        <button className="toolbar-category" id={category.id}>
                            {category.name}
                        </button>
                    </Fragment>
                ))}
            </div>
            <div className="toolbar-pages">
                {TOOLBAR_CONFIG.map((category) => (
                    <Fragment key={category.id}>
                        <div className={`toolbar-page`} id={`page-${category.id}`}>
                            {category.tools.map((tool) => (
                                <Fragment key={tool.id}>
                                    <button className="toolbar-function" id={`${category.id}-${tool.id}`} onClick={() => tool.action && tool.action(setToolbarState)}>
                                        {tool.name}
                                        {useMemo(() => tool.icon(config, toolbarState[category.id]?.[tool.id]), [config, toolbarState])}
                                    </button>
                                </Fragment>
                            ))}
                        </div>
                    </Fragment>
                ))}
            </div>
        </header>
    )
}