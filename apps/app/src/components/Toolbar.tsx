import { Fragment } from 'react'
import { TOOLBAR_CONFIG } from '../config/toolbarConfig'

export default function Toolbar() {
    return (
        <header className="toolbar">
            {TOOLBAR_CONFIG.map((category) => (
                <Fragment key={category.id}>
                    <button className="toolbar-button">{category.name}</button>
                </Fragment>
            ))}
        </header>
    )
}