// import { useState } from "react";
// import { useNavigate } from "react-router";
import Toolbar from "../components/Toolbar";
import { useSuspenseQuery } from "@tanstack/react-query";
import { meQueryOptions } from "../query/authQueries";


export default function ProtectedLayout() {
    const { data: me } = useSuspenseQuery(meQueryOptions)

    return(
        <>
            <Toolbar/>
            <p>{me?.username}</p>
        </>
    )
}