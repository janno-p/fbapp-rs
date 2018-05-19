export default [
    {
        path: "/",
        component: () => import("layouts/default"),
        children: [
            { path: "", component: () => import("pages/index") }
        ]
    },

    {
        path: "/dashboard",
        component: () => import("layouts/dashboard"),
        children: [
        ]
    },

    // Always leave this as last one
    {
        path: "*",
        component: () => import("pages/404")
    }
]
