<script lang="tsx">

import moment from "moment";
import Vue, { CreateElement } from "vue";
import { Component } from "vue-property-decorator";

@Component({})
export default class App extends Vue {
    year = moment().format("YYYY");

    createRouterLink(h: CreateElement, path: string, display: string) {
        return <li class={ this.$route.path === path ? "active" : undefined }>
            <router-link to={ path }>{ display }</router-link>
        </li>;
    }

    render(h: CreateElement) {
        return <div class="app">
            <nav class="navbar navbar-default">
                <div class="navbar-header">
                    <button type="button" class="navbar-toggle collapsed" data-toggle="collapse" data-target="#bs-example-navbar-collapse-1" aria-expanded="false">
                        <span class="sr-only">Toggle navigation</span>
                        <span class="icon-bar"></span>
                        <span class="icon-bar"></span>
                        <span class="icon-bar"></span>
                    </button>
                    <router-link class="navbar-brand" to="/">Ennustusmäng</router-link>
                </div>
                <div class="collapse navbar-collapse" id="bs-example-navbar-collapse-1">
                    <ul class="nav navbar-nav">
                        { this.createRouterLink(h, "/", "Home") }
                        { this.createRouterLink(h, "/about", "About") }
                        { this.createRouterLink(h, "/contact", "Contact") }
                    </ul>
                    <ul class="nav navbar-nav navbar-right">
                        <li><router-link to="/dashboard"><i class="fa fa-dashboard fa-lg"></i></router-link></li>
                    </ul>
                </div>
            </nav>
            <main role="main" class="container">
                <router-view />
            </main>
            <footer>
                <hr />
                <div class="container">
                    <p class="text-center">&copy; { this.year } &ndash; FbApp</p>
                </div>
            </footer>
        </div>;
    }
}
</script>
