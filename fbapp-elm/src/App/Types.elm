module App.Types exposing (..)

import Bootstrap.Navbar as Navbar
import Bootstrap.Modal as Modal
import Navigation exposing (Location)

type alias Model =
    { page : Page
    , navState : Navbar.State
    , modalState : Modal.State
    }

type Page
    = Home
    | GettingStarted
    | Modules
    | NotFound

type Msg
    = UrlChange Location
    | NavMsg Navbar.State
    | ModalMsg Modal.State
