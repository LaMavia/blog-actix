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

  let (post: option(post), setpost) = useState(None);

  let (isopen, setOpen) = useState(true);

  if (posts |> List.length === 0) {
    dispatch |> fetchPosts;
    ();
  };

  if (posts |> List.length > 0 && post === None) {
    setpost(posts->List.nth(0)->Some);
  };

  <section
    className={
      [|"wrapper", if (isopen) {"open"} else {""}|] |> Js.Array.joinWith(" ")
    }>
    <Nav posts openness=(isopen, setOpen) />
    <div
      className="content"
      onScroll={e => {
        // e |> ReactEvent.UI.preventDefault;
        e->ReactEvent.UI.currentTarget##scrollx |> Js_console.log;
        ();
      }}>
      {switch (post) {
       | Some(post) => <Post post />
       | None => <h1 className="content__ph"> "Search for a post"->str </h1>
       }}
    </div>
  </section>;
};