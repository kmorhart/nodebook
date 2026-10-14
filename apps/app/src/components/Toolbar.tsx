import styles from './Toolbar.module.css';
import { useState, useMemo, Fragment } from 'react'
import { TOOLBAR_CONFIG } from '../config/toolbarConfig'
import { useConfig } from '../config/config'
import type { ToolbarState } from '../config/types'

function initToolbarState(): ToolbarState {
    const state: ToolbarState = {}
    for (const category of TOOLBAR_CONFIG) {
        state[category.id] = {}
        for (const tool of category.groups.flatMap((group) => group.tools)) {
            if (tool.states) {
                state[category.id][tool.id] = tool.states[0]
            }
        }
    }
    return state
}

export default function Toolbar() {
    const { config, setToolbarCategory } = useConfig();
    const [toolbarState, setToolbarState] = useState<ToolbarState>(initToolbarState)

    return (
        <header className={styles.toolbar}>
            <div className={styles['toolbar-categories']}>
                {TOOLBAR_CONFIG.map((category) => (
                    <Fragment key={category.id}>
                        <button className={styles['toolbar-category']} id={category.id} onClick={() => setToolbarCategory(category.id)}>
                            {category.name}
                        </button>
                    </Fragment>
                ))}
            </div>
            <div className={styles['toolbar-pages']}>
                {TOOLBAR_CONFIG.map((category) => (
                    <Fragment key={category.id}>
                        { useMemo(() => { return (config.toolbarCategory == category.id &&  <div className={styles['toolbar-page']} id={styles[`page-${category.id}`]}>
                            {category.groups.map((group) => (
                                <Fragment key={group.id}>
                                    <div className={styles['toolbar-group']} id={`${category.id}-${group.id}`}>
                                        {group.tools.map((tool) => (
                                            <div className={styles['toolbar-function']} key={tool.id}>
                                                <button onClick={() => tool.action && tool.action(setToolbarState)}>
                                                    {tool.icon(config, toolbarState[category.id]?.[tool.id])}
                                                    {tool.element}
                                                </button>
                                            </div>
                                        ))}
                                        <p>{group.name}</p>
                                    </div>
                                </Fragment>
                            ))}
                        </div> ) }, [config, toolbarState])}
                    </Fragment>
                ))}
                
            </div>
        </header>
    )
}