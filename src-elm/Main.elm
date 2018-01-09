module Main exposing (main)

import Html exposing (..)
import Bootstrap.CDN as CDN
import Bootstrap.Grid as Grid

main =
    Grid.container []
        [ CDN.stylesheet
        , Grid.row []
            [ Grid.col []
                [ text "Some content for my view here ..." ]
            ]
        ]
