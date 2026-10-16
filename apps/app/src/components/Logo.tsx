import styles from './Logo.module.css';
import { useConfig } from '../config/config';
import { useMemo } from 'react';
import NodebookLogo from '../assets/nodebook.svg';
import NLogo from '../assets/n.svg';

export default function Logo() {
    const { config } = useConfig();

    return (


        useMemo(() => 
            <img src={config.sidebarOpen ? NodebookLogo : NLogo} className={styles.logo} style={
                { 
                    filter: config.darkMode ? 'invert(100%)' : 'none',
                    width: config.sidebarOpen ? '130px' : '17px',
                    objectFit: config.sidebarOpen ? "cover" : "none",
                    objectPosition: config.sidebarOpen ? "left center" : "center center"
                }} />
        , [config.sidebarOpen, config.darkMode])
    );
}