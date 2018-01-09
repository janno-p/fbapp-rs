module Main exposing (main)

import Html exposing (..)
import Html.Attributes exposing (..)
import Bootstrap.Grid as Grid

main : Html msg
main =
    Grid.container []
        [ Grid.row []
            [ Grid.col []
                [ i [ class "fas fa-cogs fa-2x" ] []
                , text "Some content for my view here ..."
                ]
            ]
        ]
