# ResourceProtectedRoute
Here I have the logic of how the custom route should perform:

```rs
#[component]
pub fn ResourceProtectedRoute<E>(
    cx: Scope,
    #[prop(into)]
    path: String,
    resource_condition: Resource<(usize, usize, usize), Result<bool, ServerFnError>>, //(usize, usize, usize)
    protected_view: E,
    fallback_view: E,
    resource_err_view: E,
    #[prop(into)]
    redirect_path: String,
    #[prop(default = false)]
    invert_resource_condition: bool,
    
) -> impl IntoView 
where
    E: IntoView, 
{
    let redirect_view = view! { cx, <Redirect path=redirect_path/> };
    view! { cx,
        <Route
            path=path
            view=move |cx| {
                resource_condition.refetch();
                view! { cx,
                    {move || {
                        resource_condition
                            .read(cx)
                            .map(|resource| match resource {
                                Ok(authenticated) => {
                                    let auth_condition = if invert_resource_condition {
                                        !authenticated
                                    } else {
                                        authenticated
                                    };

                                    if auth_condition == true  {
                                        view! { cx, protected_view}
                                    } else {
                                        view! { cx, redirect_view}
                                    }
                                }
                                Err(_) => {

                                    view! { cx, resource_err_view }
                                }
                            })
                            .unwrap_or_else(|| {

                                view! { cx, fallback_view }
                            })
                    }}
                }
            }
        />
    }
}
```

I was trying to take from the source of ProtectedRoute and got here:

```rs
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "info", skip_all,)
)]
#[component(transparent)]
pub fn ResourceProtectedRoute<P, E, F>(
    cx: Scope,
    path: P,
    redirect_path: P,
    resource_condition: Resource<(usize, usize, usize), Result<bool, ServerFnError>>,
    protected_view: F,
    fallback_view: F,
    resource_err_view: F,
    #[prop(default = false)]
    invert_resource_condition: bool,
    #[prop(optional)]
    ssr: SsrMode,
    #[prop(default = &[Method::Get])]
    methods: &'static [Method],
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView
where
    E: IntoView,
    F: Fn(Scope) -> E + 'static,
    P: std::fmt::Display + 'static,
    // R: Fn(Scope) -> Result<bool, ServerFnError> + 'static,
{
    // use crate::Redirect;
    let redirect_path = redirect_path.to_string();

    define_route(
        cx,
        children,
        path.to_string(),
        Rc::new(move |cx| {
            resource_condition.refetch();
            let result = resource_condition.read(cx);
            if let Some(Ok(authenticated)) = result {

                let auth_condition = if invert_resource_condition {
                    !authenticated
                } else {
                    authenticated
                };
                if auth_condition {
                    protected_view(cx).into_view(cx)
                } else {
                    view! { cx, <Redirect path=redirect_path.clone()/> }.into_view(cx)
                }
            } else if let Some(Err(_)) = result {
                resource_err_view(cx).into_view(cx)
            } else {
                fallback_view(cx).into_view(cx)
            }
        }),
        ssr,
        methods,
    )
}
```

for this:
```rs
<ResourceProtectedRoute 
                    
    path="/" 
    resource_condition=authenticated
    redirect_path="/login" 
    protected_view=|cx| 
        view! { cx, 
            <Chat /> 
        }
    fallback_view=|cx| 
        view! { cx, 
            <LoginView /> 
        }
    resource_err_view=|cx| 
        view! { cx, 
            <LoginView /> 
        }
                        
/>

```

i am running into:

```rs
error[E0308]: mismatched types
   --> src/views/app.rs:329:39
    |
325 |                           protected_view=|cx| 
    |                                          ---- the expected closure
...
329 |                           fallback_view=|cx| 
    |  _________________________-------------_^
    | |                         |
    | |                         arguments to this method are incorrect
330 | |                             view! { cx, 
331 | |                                 <LoginView /> 
332 | |                             }
    | |_____________________________^ expected closure, found a different closure
    |
    = note: expected closure `[closure@src/views/app.rs:325:40: 325:44]`
               found closure `[closure@src/views/app.rs:329:39: 329:43]`
    = note: no two closures, even if identical, have the same type
    = help: consider boxing your closure and/or using it as a trait object
```