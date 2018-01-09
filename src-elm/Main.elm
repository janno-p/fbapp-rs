module Main exposing (main)

import Html exposing (div, Html, i, text)
import Html.Attributes exposing (class, href)
import Navigation exposing (Location)
import UrlParser exposing ((</>))
import Bootstrap.Grid as Grid
import Bootstrap.Navbar as Navbar

menuView : Model -> Html Msg
menuView model =
    Grid.container []
        [ Navbar.config NavMsg
            |> Navbar.withAnimation
            |> Navbar.collapseMedium
            |> Navbar.info
            |> Navbar.brand [ href "#" ]
                [ i [ class "fas fa-futbol" ] []
                , text " FbApp"
                ]
            |> Navbar.items
                [ Navbar.itemLink [ href "#getting-started" ] [ text "Getting started" ]
                , Navbar.itemLink [ href "#modules" ] [ text "Modules" ]
                ]
            |> Navbar.view model.navState
        ]

view : Model -> Html Msg
view model =
    div []
        [ menuView model
        , Grid.container []
            [ Grid.row []
                [ Grid.col [] [ text "Some content for my view here ..." ] ]
            ]
        ]

type alias Model =
    { page : Page
    , navState : Navbar.State
    }

type Msg
    = UrlChange Location
    | NavMsg Navbar.State

type Page
    = Home
    | NotFound

routeParser : UrlParser.Parser ( Page -> a ) a
routeParser =
    UrlParser.oneOf
        [ UrlParser.map Home UrlParser.top ]

decode : Location -> Maybe Page
decode location =
    UrlParser.parseHash routeParser location

urlUpdate : Navigation.Location -> Model -> ( Model, Cmd Msg )
urlUpdate location model =
    case decode location of
        Nothing ->
            ( { model | page = NotFound }, Cmd.none )
        Just route ->
            ( { model | page = route }, Cmd.none )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UrlChange location ->
            urlUpdate location model
        NavMsg state ->
            ( { model | navState = state }, Cmd.none )

subscriptions : Model -> Sub Msg
subscriptions model =
    Navbar.subscriptions model.navState NavMsg

init : Location -> ( Model, Cmd Msg )
init location =
    let
        ( navState, navCmd ) =
            Navbar.initialState NavMsg
        ( model, urlCmd ) =
            urlUpdate location { navState = navState, page = Home }
    in
        ( model, Cmd.batch [ urlCmd, navCmd ] )

main : Program Never Model Msg
main = 
    Navigation.program UrlChange
        { view = view
        , update = update
        , subscriptions = subscriptions
        , init = init
        }
