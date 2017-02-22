module Main exposing (..)

import WebGL exposing (..)
import Math.Vector3 exposing (..)
import Math.Matrix4 exposing (..)
import Html exposing (Html)
import Html.Attributes exposing (width, height)
import AnimationFrame
import Time exposing (Time)


-- Types


type alias Vertex =
    { position : Vec3, color : Vec3 }


main =
    Html.program
        { init = ( 0, Cmd.none )
        , view = view
        , subscriptions = (\model -> AnimationFrame.diffs Basics.identity)
        , update = (\elapsed currentTime -> ( elapsed + currentTime, Cmd.none ))
        }



-- View


view t =
    WebGL.toHtml
        [ width 400, height 400 ]
        [ render vert frag mesh { perspective = perspective (t / 1000) } ]


perspective t =
    mul (makePerspective 45 1 0.01 100)
        (makeLookAt (vec3 (4 * cos t) 0 (4 * sin t)) (vec3 0 0 0) (vec3 0 1 0))



-- Mesh


mesh =
    Triangle
        [ ( Vertex (vec3 0 0 0) (vec3 1 0 0)
          , Vertex (vec3 1 1 0) (vec3 0 1 0)
          , Vertex (vec3 1 -1 0) (vec3 0 0 1)
          )
        ]



-- Shaders


vert =
    [glsl|

attribute vec3 position;
attribute vec3 color;
uniform mat4 perspective;
varying vec3 vcolor;

void main() {
    gl_Position = perspective * vec4(position, 1.0);
    vcolor = color;
}
|]


frag =
    [glsl|
precision mediump float;
varying vec3 vcolor;

void main () {
    gl_FragColor = vec4(vcolor, 1.0);
}
|]