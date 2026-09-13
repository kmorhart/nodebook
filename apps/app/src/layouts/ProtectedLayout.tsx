import { useState } from "react";
import { redirect, useNavigate } from "react-router";
import Toolbar from "../components/Toolbar";


export default function ProtectedLayout() {
    const navigate = useNavigate();

    const [isAuthenticated, setIsAuthenticated] = useState(false);

    if (!isAuthenticated) {
        //redirect please
    }

    return(
        <>
            <Toolbar/>
        </>
    )
}