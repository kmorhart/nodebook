import { useConfig } from '../config/config';
import styles from './Sidebar.module.css';
import * as Icons from '../components/icons'

export default function Sidebar() {
    const { config, toggleDarkMode } = useConfig();

    return (
        <aside className={styles.sidebar}>
            <div className={styles['toolbar-modeToggle']}>
                <button onClick={() => toggleDarkMode()}>
                    {config.darkMode ? <Icons.Sun color="#fff"/> : <Icons.Moon color="#000"/>}
                </button>
            </div>
        </aside>
    )
}