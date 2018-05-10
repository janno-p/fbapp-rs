<template>
    <q-layout view="hHh Lpr lFf">
        <q-layout-header>
            <q-toolbar color="primary" :glossy="$q.theme === 'mat'" :inverted="$q.theme === 'ios'">
                <!--<q-btn flat dense round @click="leftDrawerOpen = !leftDrawerOpen" aria-label="Menu">
                    <q-icon name="menu" />
                </q-btn>-->
                <q-icon name="mdi-soccer" size="24pt" />

                <q-toolbar-title>
                    Ennustusmäng
                    <div slot="subtitle">Running on Quasar v{{ $q.version }}.</div>
                </q-toolbar-title>
            </q-toolbar>
        </q-layout-header>

        <q-page-container>
            <router-view />
        </q-page-container>
    </q-layout>
</template>

<script>
import { openURL } from 'quasar'

export default {
    name: 'LayoutDefault',
    methods: {
        openURL
    }
}
</script>

<style></style>
