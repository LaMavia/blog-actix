// Generated by BUCKLESCRIPT VERSION 5.0.4, PLEASE EDIT WITH CARE
'use strict';

var List = require("bs-platform/lib/js/list.js");
var Curry = require("bs-platform/lib/js/curry.js");
var React = require("react");
var Belt_List = require("bs-platform/lib/js/belt_List.js");
var Json_decode = require("@glennsl/bs-json/src/Json_decode.bs.js");
var Nav$ReactHooksTemplate = require("./Nav.bs.js");
var Post$ReactHooksTemplate = require("./Post.bs.js");
var Helpers$ReactHooksTemplate = require("./Helpers.bs.js");

function decodePost(json) {
  return /* record */[
          /* id */Json_decode.field("id", Json_decode.$$int, json),
          /* author */Json_decode.field("author", Json_decode.string, json),
          /* date */Json_decode.field("date", Json_decode.string, json),
          /* title */Json_decode.field("title", Json_decode.string, json),
          /* body */Json_decode.field("body", Json_decode.string, json)
        ];
}

function fetchPosts(dispatch) {
  return fetch("/api/posts").then((function (prim) {
                    return prim.json();
                  })).then((function (j) {
                  return Promise.resolve(Belt_List.fromArray(Json_decode.array(decodePost, j)));
                })).then((function (ps) {
                return Promise.resolve(Curry._1(dispatch, /* Add */[ps]));
              }));
}

function App(Props) {
  var match = React.useReducer((function (state, action) {
          return List.merge((function (l1, l2) {
                        return l1[/* id */0] - l2[/* id */0] | 0;
                      }), state, action[0]);
        }), /* [] */0);
  var posts = match[0];
  var match$1 = Helpers$ReactHooksTemplate.useState(undefined);
  var post = match$1[0];
  var match$2 = Helpers$ReactHooksTemplate.useState(true);
  var isopen = match$2[0];
  if (List.length(posts) === 0) {
    fetchPosts(match[1]);
  }
  if (List.length(posts) > 0 && post === undefined) {
    Curry._1(match$1[1], List.nth(posts, 0));
  }
  return React.createElement("section", {
              className: /* array */[
                  "wrapper",
                  isopen ? "open" : ""
                ].join(" ")
            }, React.createElement(Nav$ReactHooksTemplate.make, {
                  posts: posts,
                  openness: /* tuple */[
                    isopen,
                    match$2[1]
                  ]
                }), React.createElement("div", {
                  className: "content",
                  onScroll: (function (e) {
                      console.log(e.currentTarget.scrollx);
                      return /* () */0;
                    })
                }, post !== undefined ? React.createElement(Post$ReactHooksTemplate.make, {
                        post: post
                      }) : React.createElement("h1", {
                        className: "content__ph"
                      }, Helpers$ReactHooksTemplate.str("Search for a post"))));
}

var make = App;

exports.decodePost = decodePost;
exports.fetchPosts = fetchPosts;
exports.make = make;
/* react Not a pure module */