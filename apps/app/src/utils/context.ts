import { createContext } from "react-router";
import type { Me } from "./types";

export const userContext = createContext<Me | null>(null);
