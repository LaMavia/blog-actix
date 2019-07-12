open Helpers;
open Globals;

module Header = {
  [@react.component]
  let small = (~post: post) => {
    let style =
      ReactDOMRe.Style.make()
      ->ReactDOMRe.Style.unsafeAddProp("--align", "left")
      ->ReactDOMRe.Style.unsafeAddProp("--size", "0.75");

    <div className="header header--small" style>
      <h2 className="header__text"> {post.title |> str} </h2>
      <div className="header__bottom">
        <span className="header__bottom__author"> {post.author |> str} </span>
        <time className="header__bottom__date"> {post.date |> str} </time>
      </div>
    </div>;
  };

  [@react.component]
  let make = (~post: post) => {
    <header className="header">
      <h1 className="header__text"> {post.title |> str} </h1>
      <div className="header__bottom">
        <span className="header__bottom__author"> {post.author |> str} </span>
        <time className="header__bottom__date"> {post.date |> str} </time>
      </div>
    </header>;
  };
};

[@react.component]
let make = (~post: post) => {
  <article className="post">
    <Header post />
    <p className="post__body" dangerouslySetInnerHTML={{"__html": post.body}}></p>
  </article>;
};