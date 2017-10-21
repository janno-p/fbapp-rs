module Home.State exposing (init, update, subscriptions)

import Home.Types exposing (..)

init : ( Model, Cmd Msg )
init =
    ( (), Cmd.none )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model ->
    case msg of
        NoOp ->
            model

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
