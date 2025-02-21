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
import { useLocation, Route, Routes, Navigate } from "react-router-dom";
import Sidebar from "components/Sidebar/Sidebar.js";
import Home from "views/examples/Home.js";

import routes from "routes.js";
import MyBreadcrumb from "views/examples/BreadCrumb";
import BindForm from "views/examples/BindForm";
import AddUserForm from "views/examples/AddUserForm";
import Submit from "views/examples/Submit";
import Execute from "views/examples/Execute.js";
import Upload from "views/examples/Upload";

const Admin = (props) => {

  const getRoutes = (routes) => {
    return routes.flatMap((prop, key) => {
      let mainRoute = prop.layout === "/admin" ? (
        <Route path={prop.path} element={prop.component} key={key} exact />
      ) : null;

      let subRoutes = prop.subItems
        ? prop.subItems.map((subItem, subKey) => (
          <Route
            path={prop.path + subItem.path}
            element={subItem.component || <div>{subItem.name}</div>}
            key={`${key}-${subKey}`}
            exact
          />
        ))
        : [];

      return [mainRoute, ...subRoutes].filter(Boolean);
    });
  };

  console.log("props", props);

  return (
    <>
      <Sidebar
        {...props}
        routes={routes}
      />
      <div className="main-content" >
        <MyBreadcrumb />
        <Routes>
          {getRoutes(routes)}
          <Route path="/home" element={<Home />} />
          <Route path="/users/bind" element={<BindForm />} />
          <Route path="/users/add" element={<AddUserForm />} />
          <Route path="/programs/upload" element={<Submit />} />
          <Route path="/programs/execute" element={<Execute />} />
          <Route path="/supported-token-drivers/upload" element={<Upload />} />
          <Route path="*" element={<Navigate to="/admin/home" replace />} />
        </Routes>
      </div>
    </>
  );
};

export default Admin;
