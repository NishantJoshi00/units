/*!

=========================================================
* Finternet Dashboard React - v1.2.4
=========================================================

* Product Page: https://www.creative-tim.com/product/finternet-dashboard-react
* Copyright 2024 Creative Tim (https://www.creative-tim.com)
* Licensed under MIT (https://github.com/creativetimofficial/finternet-dashboard-react/blob/master/LICENSE.md)

* Coded by Creative Tim

=========================================================

* The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

*/
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes, Navigate } from "react-router-dom";
import { NavbarBrand } from "reactstrap";

import "assets/plugins/nucleo/css/nucleo.css";
import "@fortawesome/fontawesome-free/css/all.min.css";
import "assets/scss/finternet-dashboard-react.scss";

import AdminLayout from "layouts/Admin.js";

const root = ReactDOM.createRoot(document.getElementById("root"));

root.render(
  <div>
    <header style={{ background: "#FFF", height: "50px", "--tw-shadow": "0 1px 3px 0 rgb(0 0 0 / .1), 0 1px 2px -1px rgb(0 0 0 / .1)", boxShadow: "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)" }}>
      <NavbarBrand className="pt-0" >
        <img
          className="navbar-brand-img"
          src="https://finternetlab.io/images/headers/finternet-favicon.png"
          onClick={() => window.location.href = "/admin/home"}
        />
      </NavbarBrand></header>
    <BrowserRouter>
      <Routes>
        <Route path="/admin/*" element={<AdminLayout />} />
        <Route path="*" element={<Navigate to="/admin/home" replace />} />
      </Routes>
    </BrowserRouter>
  </div>
);
