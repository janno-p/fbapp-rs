module App.State exposing (init, update, subscriptions)

import App.Types exposing (..)
import Bootstrap.Modal as Modal
import Bootstrap.Navbar as Navbar
import Navigation exposing (Location)
import UrlParser

init : Location -> ( Model, Cmd Msg )
init location =
    let
        ( navState, navCmd ) =
            Navbar.initialState NavMsg
        ( model, urlCmd ) =
            urlUpdate location { navState = navState, page = Home, modalState = Modal.hiddenState }
    in
        ( model, Cmd.batch [ urlCmd, navCmd ])

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UrlChange location ->
            urlUpdate location model
        NavMsg state ->
            ( { model | navState = state }
            , Cmd.none
            )
        ModalMsg state ->
            ( { model | modalState = state }
            , Cmd.none
            )

urlUpdate : Navigation.Location -> Model -> ( Model, Cmd Msg )
urlUpdate location model =
    case decode location of
        Nothing ->
            ( { model | page = NotFound }, Cmd.none )
        Just route ->
            ( { model | page = route }, Cmd.none )

decode : Location -> Maybe Page
decode location =
    UrlParser.parseHash routeParser location

routeParser : UrlParser.Parser ( Page -> a ) a
routeParser =
    UrlParser.oneOf
        [ UrlParser.map Home UrlParser.top
        , UrlParser.map GettingStarted ( UrlParser.s "getting-started" )
        , UrlParser.map Modules ( UrlParser.s "modules" )
        ]

subscriptions : Model -> Sub Msg
subscriptions model =
    Navbar.subscriptions model.navState NavMsg
