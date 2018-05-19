import Vue from "vue"
import Vuex from "vuex"

import example from "./module-example"

Vue.use(Vuex)

const store = new Vuex.Store({
    state: {
        isGoogleReady: false,
        isSignedIn: false,
        name: "",
        imageUrl: "",
        email: "",
        claims: []
    },
    modules: {
        example
    },
    actions: {
        authenticate (context) {
            try {
                this._vm.$axios.post("/tokeninfo", {}).then(
                    ({ data }) => {
                        context.commit("setUser", data)
                        context.commit("setGoogleReady", { isReady: true })
                    },
                    (e) => console.error(e)
                )
            } catch (e) {
                console.error(e)
            }
        },
        googleSignIn (context) {
            try {
                const auth = window.gapi.auth2.getAuthInstance()
                auth.signIn().then((googleUser) => {
                    this._vm.$axios.post("/tokensignin", { id_token: googleUser.getAuthResponse().id_token }).then(
                        ({ data }) => context.commit("setUser", data),
                        (e) => console.error(e)
                    )
                })
            } catch (e) {
                console.error(e)
            }
        },
        googleSignOut (context) {
            try {
                const auth = window.gapi.auth2.getAuthInstance()
                auth.disconnect()
                this._vm.$axios.post("/tokensignout", {}).then(
                    () => context.commit("setUser", null),
                    (e) => console.error(e)
                )
            } catch (e) {
                console.error(e)
            }
        }
    },
    mutations: {
        setGoogleReady (state, { isReady }) {
            state.isGoogleReady = isReady
        },
        setUser (state, payload) {
            state.isSignedIn = payload ? payload.is_signed_in : false
            state.email = payload ? payload.email : ""
            state.imageUrl = payload ? payload.picture : ""
            state.name = payload ? payload.name : ""
            state.claims = payload ? payload.claims : []
        }
    },
    getters: {
        hasDashboard (state) {
            return state.claims.includes("use_dashboard")
        }
    }
})

export default store
