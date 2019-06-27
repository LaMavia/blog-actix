open Helpers;
open Globals;

[@react.component]
let make = (~posts: list(post), ~openness, ~setPost) => {
  let (searchStr, setSearch) = useState("");
  let (isOpen, setOpen) = openness;
  // flex-column
  <nav className="nav">
    <form
      className="nav__form"
      onSubmit={e => e |> ReactEvent.Form.preventDefault}>
      <input
        ariaLabel="posts search"
        className="nav__form__input"
        placeholder="Search"
        type_="text"
        onChange={e => e->ReactEvent.Form.currentTarget##value |> setSearch}
      />
      <button
        className="nav__form__burger"
        onClick={e => {
          e |> ReactEvent.Mouse.preventDefault;
          setOpen(!isOpen);
        }}>
        (if (isOpen) {"X"} else {">"})->str
      </button>
    </form>
    <ol className="nav__results">
      {posts
       ->Array.of_list
       ->Belt.Array.reduceWithIndex([||], (acc, p, i) =>
           if (p.title
               |> Js.Re.test_(
                    Js.Re.fromStringWithFlags(searchStr, ~flags="ig"),
                  )) {
             let a = [|
               <li className="nav__results__item" key={i |> string_of_int}>
                 <a
                   className="nav__results__item__link"
                   href=""
                   onClick={e => {
                     e |> ReactEvent.Mouse.preventDefault;
                     p->Some->setPost;
                     setOpen(false);
                   }}>
                   <Post.Header.small post=p />
                 </a>
               </li>,
             |];
             Belt.Array.concat(acc, a);
           } else {
             acc;
           }
         )
       ->React.array}
    </ol>
  </nav>;
};