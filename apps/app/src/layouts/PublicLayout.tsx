import { Navigate } from "react-router";
import { Outlet } from "react-router";
import { useSuspenseQuery } from "@tanstack/react-query";
import { sessionQueryOptions } from "../query/authQueries";

export default function PublicLayout() {
    try{
        const session = useSuspenseQuery(sessionQueryOptions);
        
        if (session) {
            return <Navigate to="/new" />
        }
    } catch (error) {
        return <Outlet/>
    }
}