import { QueryClientProvider } from '@tanstack/react-query'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router' 
import { router } from './config/router.tsx'
import './index.css'
import { queryClient } from './query/queryClient.ts'
import { ConfigProvider } from './config/config.tsx'

const savedTheme = localStorage.getItem('config') ? (JSON.parse(localStorage.getItem('config')!).darkMode ? 'dark' : 'light') : 'dark';

document.documentElement.setAttribute('data-theme', savedTheme);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ConfigProvider>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </ConfigProvider>
  </StrictMode>
)