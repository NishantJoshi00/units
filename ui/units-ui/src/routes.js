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
import SupportTokens from "views/SupportTokens.js";
import Users from "views/examples/Users.js";
import Submit from "views/examples/Submit.js";
import Execute from "views/examples/Execute.js";
import Program from "views/examples/Programs";

var routes = [
  {
    path: "/users",
    name: "Users",
    icon: "fa-solid fa-user-gear",
    component: <Users />,
    layout: "/admin"
  },
  {
    path: "/supported-token-drivers",
    name: "Supported token drivers",
    icon: "fa-solid fa-coins",
    component: <SupportTokens />,
    layout: "/admin"
  },
  {
    name: "Programs",
    icon: "fa-solid fa-code",
    path: "/programs",
    layout: "/admin",
    component: <Program />,
    // subItems: [
    //   {
    //     name: "Upload",
    //     icon: "fa-solid fa-cloud-arrow-up",
    //     component: <Submit />,
    //     path: "/upload"
    //   },
    //   {
    //     name: "Execute",
    //     icon: "fa-solid fa-gear",
    //     component: <Execute />,
    //     path: "/execute"
    //   }
    // ]
  }
];
export default routes;
