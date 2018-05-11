<template>
    <div id="q-app">
        <router-view />
    </div>
</template>

<script>
import { mapActions } from "vuex"

export default {
    name: "App",
    created () {
        const gapi = window.gapi
        gapi.load("auth2", () => {
            gapi.auth2.init({
                client_id: "83610951178-d4jm3o26r9r40aspvbe9730pjj3nn5d8.apps.googleusercontent.com",
                cookie_policy: "single_host_origin",
                scope: "profile email"
            }).then(
                () => this.authenticate(),
                (error) => console.error(error)
            )
        })
    },
    methods: {
        ...mapActions([
            "authenticate"
        ])
    }
}
</script>

<style></style>
