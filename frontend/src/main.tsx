import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Auth from './Pages/Auth';
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer } from 'react-toastify';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
        <Routes>
            <Route path="/auth" element={<Auth />} />
        </Routes>
    </BrowserRouter>
    <ToastContainer />
  </React.StrictMode>
)
