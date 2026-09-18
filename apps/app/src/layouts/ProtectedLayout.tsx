import Toolbar from "../components/Toolbar";
import { Outlet } from "react-router";


export default function ProtectedLayout() {
    return(
        <>
            <Toolbar/>
            <Outlet/>
        </>
    )
}