module Main exposing (main)

import App.State as State
import App.Types exposing (..)
import App.View as View
import Navigation exposing (program)

main : Program Never Model Msg
main =
    Navigation.program UrlChange
        { init = State.init
        , update = State.update
        , subscriptions = State.subscriptions
        , view = View.root
        }
