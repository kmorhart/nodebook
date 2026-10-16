import { useConfig } from '../config/config';
import styles from './Sidebar.module.css';
import * as Icons from '../components/icons'
import { useMemo } from 'react';
import { useNavigate } from 'react-router';

export default function Sidebar() {
    const { config, toggleSidebar } = useConfig();

    const navigate = useNavigate();

    return (
        <aside className={styles.sidebar}>
            {useMemo(() => <nav className={styles['sidebar-nav']} style={config.sidebarOpen ? { width: '14.444rem' } : { width: '2.2222rem' }}>
                <div className={styles['sidebar-header']}>
                    <button className={styles['sidebar-button']} onClick={() => toggleSidebar()}>
                        <Icons.PanelLeft />
                    </button>
                    <button className={styles['sidebar-button']}>
                        <Icons.Plus />
                    </button>
                </div>
                <div className={styles['sidebar-footer']}>
                    <button className={styles['sidebar-button']}>
                        <Icons.Settings />
                    </button>
                    <button className={styles['sidebar-button']}>
                        <Icons.User style={{stroke: "#4e88ce"}} />
                    </button>
                    <button className={styles['sidebar-button']}>
                        <Icons.LogOut style={{stroke: "#ce4e4e"}} onClick={() => {navigate('/logout')}} />
                    </button>
                </div>
            </nav>, [config.sidebarOpen, config.darkMode])}
        </aside>
    )
}