import Vue, { CreateElement, VNode } from "vue";

import router from "./router";

import App from "./App.vue";

Vue.config.productionTip = false;

/* tslint-disable no-unused-expression */
// tslint:disable-next-line:no-unused-expression
new Vue({
    el: "#app",
    render: (h: CreateElement): VNode => h(App),
    router
});
