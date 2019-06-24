open Helpers;
open Globals;

type action =
  | Add(list(post));

type state = list(post);

let decodePost = json => {
  Json.Decode.{
    id: json |> field("id", int),
    author: json |> field("author", string),
    date: json |> field("date", string),
    title: json |> field("title", string),
    body: json |> field("body", string),
  };
};

let fetchPosts = dispatch => {
  Js.Promise.(
    Fetch.fetch("/api/posts")
    |> then_(Fetch.Response.json)
    |> then_(j =>
         j |> Json.Decode.array(decodePost) |> Belt.List.fromArray |> resolve
       )
    |> then_(ps => Add(ps) |> dispatch |> resolve)
  );
};

[@react.component]
let make = () => {
  let (posts, dispatch) =
    React.useReducer(
      (state, action) =>
        switch (action) {
        | Add(posts) => List.merge((l1, l2) => l1.id - l2.id, state, posts)
        },
      [],
    );

  if (List.length(posts) == 0) {
    dispatch |> fetchPosts;
    ();
  };

  <section className="">
    <Nav posts />
    <h1> {"Hello there" |> React.string} </h1>
  </section>;
};