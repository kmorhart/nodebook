import Sidebar from "../components/Sidebar";
import Toolbar from "../components/Toolbar";
import { Outlet } from "react-router";
import styles from './ProtectedLayout.module.css';
import Prompter from "../components/Prompter";
import Logo from "../components/Logo";


export default function ProtectedLayout() {
    return(
        <div className={styles.layout}>
            <Logo />
            <Toolbar/>
            <Sidebar/>
            <Outlet/>
            <Prompter/>
        </div>
    )
}