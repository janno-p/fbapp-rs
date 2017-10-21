module Home.View exposing (root)

import Html exposing (h1, text)

import Home.Types exposing (Model)

root : Model -> Html Msg
root model =
    h1 [] [ text "text" ]
