import Vue from "vue"
import Vuex from "vuex"

import example from "./module-example"

Vue.use(Vuex)

function getUserPayload (googleUser) {
    let email = ""
    let imageUrl = ""
    let name = ""
    let userId = ""
    let userToken = ""
    if (googleUser) {
        const profile = googleUser.getBasicProfile()
        email = profile.getEmail()
        imageUrl = profile.getImageUrl()
        name = profile.getName()
        userId = profile.getId()
        userToken = googleUser.getAuthResponse().id_token
    }
    return { email, imageUrl, name, userId, userToken }
}

const store = new Vuex.Store({
    state: {
        isGoogleReady: false,
        userToken: "",
        userId: "",
        name: "",
        imageUrl: "",
        email: ""
    },
    modules: {
        example
    },
    actions: {
        authenticate (context) {
            try {
                context.commit("setGoogleReady", { isReady: true })
                const auth = window.gapi.auth2.getAuthInstance()
                context.commit("setUser", getUserPayload(auth.isSignedIn ? auth.currentUser.get() : null))
            } catch (e) {
                console.error(e)
            }
        },
        googleSignIn (context) {
            try {
                const auth = window.gapi.auth2.getAuthInstance()
                auth.signIn().then((googleUser) => context.commit("setUser", getUserPayload(googleUser)))
            } catch (e) {
                console.error(e)
            }
        },
        googleSignOut (context) {
            try {
                const auth = window.gapi.auth2.getAuthInstance()
                auth.disconnect()
                context.commit("setUser", getUserPayload(null))
                // TODO: notify server
            } catch (e) {
                console.error(e)
            }
        }
    },
    mutations: {
        setGoogleReady (state, { isReady }) {
            state.isGoogleReady = isReady
        },
        setUser (state, { email, imageUrl, name, userId, userToken }) {
            state.email = email
            state.imageUrl = imageUrl
            state.name = name
            state.userId = userId
            state.userToken = userToken
        }
    },
    getters: {
        isSignedIn: state => !!state.userToken
    }
})

export default store
