open Helpers;
open Globals;

type action =
  | Add(list(post));

type state = {
  posts: list(post),
  search: string,
};

[@react.component]
let make = () => {
  let ph_post: post = {
    id: 0,
    author: "cat",
    date: "2019-06-22",
    title: "Cats Should Rule The World, Soon",
    body: "Sure they will, the question is - when?",
  };
  let (state, dispatch) =
    React.useReducer(
      (state, action) =>
        switch (action) {
        | Add(posts) => {
            ...state,
            posts: List.merge((l1, l2) => l1.id - l2.id, state.posts, posts),
          }
        },
      {posts: [ph_post, ph_post, ph_post], search: ""},
    );

  // flex-column
  <nav className="nav">
    <input className="nav__input" placeholder="Search" type_="text" />
    <button className="nav__burger"> "X"->str </button>
    <ol className="nav__results">
      {state.posts
       ->Array.of_list
       ->Belt.Array.mapWithIndex((i, p) =>
           <li className="nav__results__item" key={i |> string_of_int}>
             <Post.Header.small post=p />
           </li>
         )
       |> React.array}
    </ol>
  </nav>;
};