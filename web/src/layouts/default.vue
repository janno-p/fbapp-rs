<template>
    <q-layout view="hHh Lpr lFf">
        <app-site-toolbar />

        <q-page-container>
            <router-view />
        </q-page-container>
    </q-layout>
</template>

<script>
import AppSiteToolbar from "../components/site-toolbar.vue"

export default {
    name: "LayoutDefault",
    components: {
        AppSiteToolbar
    }
}
</script>

<style></style>
