import Sidebar from "../components/Sidebar";
import Toolbar from "../components/Toolbar";
import { Outlet } from "react-router";
import styles from './ProtectedLayout.module.css';


export default function ProtectedLayout() {
    return(
        <div className={styles.layout}>
            <Toolbar/>
            <Sidebar/>
            <Outlet/>
        </div>
    )
}