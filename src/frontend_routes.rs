use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use stylers::{style, style_sheet};

#[component]
pub fn FrontendRoutes(cx: Scope) -> impl IntoView {

    let class_name = style! {"Hello",
    // this comment will be ignored
    div {
        border: 1px solid black;/*This comment also will be ignored */
        margin: 25px 50px 75px 100px;
        background-color: lightblue;
        content: raw_str(r#"\hello"#);
        font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        .two{
            color: yellow;
            background-color: red;
        }
        div .one p{
            color: blue;
            background-color: red;
        }
        div.one{
            color: red;
        }
        div #two{
            color: blue;
        }
        h2,a {
            color: purple;
        }
        .one:hover{
            background-color: green;
        }
        p:lang(it){
            background: yellow;
        }
        p::before {
            content: raw_str("Read this: ");
        }
    };

    provide_meta_context(cx);
    view! {
        cx, class = class_name,
        <Router>
            <div class="one">
                    <div class="one">
                    <h1 id="two">"Hello"</h1>
                    <h2>"World"</h2>
                    <h3>"Hello Kanna"</h3>
                    <p> "This is example conent"</p>
                    <a href="www.google.com">"Visit the link"</a>
                </div>
                <nav>
                    <ul>
                        <li><A href="">"Simple"</A></li>
                        <li><A href="form">"Form-Based"</A></li>
                        <li><A href="multi">"Multi-User"</A></li>
                    </ul>
                </nav>
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <main>
                    <Routes>
                        <Route path="" view=|cx| view! {
                            cx,
                            <HomeComponent/>
                        }/>
                        <Route path="form" view=|cx| view! {
                            cx,
                            <HomeComponent/>
                        }/>
                        <Route path="multi" view=|cx| view! {
                            cx,
                            <HomeComponent/>
                        }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

// This is an example of "single-user" server functions
// The counter value is loaded from the server, and re-fetches whenever
// it's invalidated by one of the user's own actions
// This is the typical pattern for a CRUD app
#[component]
pub fn HomeComponent(cx: Scope) -> impl IntoView {
    let value = 0;

    view! {
        cx,
        <div class="one">
            <h2>"Simple Counter"</h2>
            <p>"This counter sets the value on the server and automatically reloads the new value."</p>
            <div>
                <span>"Value: " {value} "!"</span>
            </div>
        </div>
    }
}
