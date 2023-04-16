
// This is the <Form/> counter
// It uses the same invalidation pattern as the plain counter,
// but uses HTML forms to submit the actions
#[component]
pub fn FormCounter(cx: Scope) -> impl IntoView {
    let adjust = create_server_action::<AdjustServerCount>(cx);
    let clear = create_server_action::<ClearServerCount>(cx);

    let counter = create_resource(
        cx,
        move || (adjust.version().get(), clear.version().get()),
        |_| {
            log::debug!("FormCounter running fetcher");
            get_server_count()
        },
    );
    let value = move || {
        log::debug!("FormCounter looking for value");
        counter
            .read(cx)
            .map(|n| n.ok())
            .flatten()
            .map(|n| n)
            .unwrap_or(0)
    };

    view! {
        cx,
        <div>
            <h2>"Form Counter"</h2>
            <p>"This counter uses forms to set the value on the server. When progressively enhanced, it should behave identically to the “Simple Counter.”"</p>
            <div>
                // calling a server function is the same as POSTing to its API URL
                // so we can just do that with a form and button
                <ActionForm action=clear>
                    <input type="submit" value="Clear"/>
                </ActionForm>
                // We can submit named arguments to the server functions
                // by including them as input values with the same name
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="-1"/>
                    <input type="hidden" name="msg" value="form value down"/>
                    <input type="submit" value="-1"/>
                </ActionForm>
                <span>"Value: " {move || value().to_string()} "!"</span>
                <ActionForm action=adjust>
                    <input type="hidden" name="delta" value="1"/>
                    <input type="hidden" name="msg" value="form value up"/>
                    <input type="submit" value="+1"/>
                </ActionForm>
            </div>
        </div>
    }
}

// This is a kind of "multi-user" counter
// It relies on a stream of server-sent events (SSE) for the counter's value
// Whenever another user updates the value, it will update here
// This is the primitive pattern for live chat, collaborative editing, etc.
#[component]
pub fn MultiuserCounter(cx: Scope) -> impl IntoView {
    let dec = create_action(cx, |_| adjust_server_count(-1, "dec dec goose".into()));
    let inc = create_action(cx, |_| adjust_server_count(1, "inc inc moose".into()));
    let clear = create_action(cx, |_| clear_server_count());

    #[cfg(not(feature = "ssr"))]
    let multiplayer_value = {
        use futures::StreamExt;

        let mut source = gloo_net::eventsource::futures::EventSource::new("/api/events")
            .expect("couldn't connect to SSE stream");
        let s = create_signal_from_stream(
            cx,
            source.subscribe("message").unwrap().map(|value| {
                match value {
                    Ok(value) => {
                        value.1.data().as_string().expect("expected string value")
                    },
                    Err(_) => "0".to_string(),
                }
            })
        );

        on_cleanup(cx, move || source.close());
        s
    };

    #[cfg(feature = "ssr")]
    let (multiplayer_value, _) =
        create_signal(cx, None::<i32>);

    view! {
        cx,
        <div>
            <h2>"Multi-User Counter"</h2>
            <p>"This one uses server-sent events (SSE) to live-update when other users make changes."</p>
            <div>
                <button on:click=move |_| clear.dispatch(())>"Clear"</button>
                <button on:click=move |_| dec.dispatch(())>"-1"</button>
                <span>"Multiplayer Value: " {move || multiplayer_value.get().unwrap_or_default().to_string()}</span>
                <button on:click=move |_| inc.dispatch(())>"+1"</button>
            </div>
        </div>
    }
}