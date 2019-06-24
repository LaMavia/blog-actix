open Helpers;
open Globals;

[@react.component]
let make = (~posts: list(post)) => {
  let (isOpen, setOpen) = useState(true);
  let (searchStr, setSearch) = useState("");
  // flex-column
  <nav
    className={
      ["nav", if (isOpen) {"nav--open"} else {""}]
      ->Belt.List.reduce("", (acc, x) => acc ++ " " ++ x)
    }>
    <form action="" onSubmit={e => e |> ReactEvent.Form.preventDefault}>
      <input
        ariaLabel="posts search"
        className="nav__form__input"
        placeholder="Search"
        type_="text"
      />
      <button
        className="nav__form__burger"
        onClick={e => {
          e |> ReactEvent.Mouse.preventDefault;
          setOpen(false);
        }}>
        "X"->str
      </button>
    </form>
    <ol className="nav__results">
      {posts
       ->Array.of_list
       ->Belt.Array.mapWithIndex((i, p) =>
           <li className="nav__results__item" key={i |> string_of_int}>
             <Post.Header.small post=p />
           </li>
         )
       ->React.array}
    </ol>
  </nav>;
};