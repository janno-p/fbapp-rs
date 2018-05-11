<template>
    <q-layout view="hHh Lpr lFf">
        <q-layout-header>
            <q-toolbar color="primary" :glossy="$q.theme === 'mat'" :inverted="$q.theme === 'ios'">
                <q-icon name="mdi-soccer" size="24pt" />

                <q-toolbar-title>
                    Ennustusmäng
                    <div slot="subtitle">2018. aasta jalgpalli maailmameistrivõistlused</div>
                </q-toolbar-title>

                <template v-if="isGoogleReady">
                    <template v-if="isSignedIn">
                        <img :src="sizedImageUrl" />
                        <span class="q-pl-sm q-pr-sm text-weight-medium">{{ name }}</span>
                        <q-btn flat dense round @click="googleSignOut" title="Logi välja">
                            <q-icon name="mdi-logout" />
                        </q-btn>
                    </template>
                    <q-btn v-else flat dense round @click="googleSignIn" title="Logi sisse Google kontoga">
                        <q-icon name="mdi-google" />
                    </q-btn>
                </template>
            </q-toolbar>
        </q-layout-header>

        <q-page-container>
            <router-view />
        </q-page-container>
    </q-layout>
</template>

<script>
import { mapState, mapActions, mapGetters } from "vuex"

export default {
    name: "LayoutDefault",
    computed: {
        ...mapState([
            "isGoogleReady",
            "imageUrl",
            "name"
        ]),
        ...mapGetters([
            "isSignedIn"
        ]),
        sizedImageUrl () {
            return this.imageUrl + "?sz=32"
        }
    },
    methods: {
        ...mapActions([
            "googleSignIn",
            "googleSignOut"
        ])
    }
}
</script>

<style></style>
