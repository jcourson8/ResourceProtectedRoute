use std::{rc::Rc, cell::Cell};

use leptos_router::*;
use leptos::*;
thread_local! {
    static ROUTE_ID: Cell<usize> = Cell::new(0);
}


/// This is a guarded route component that is protected by a certain resource condition.
/// It defines a route that will display different views based on the state of a certain condition. 
///
/// # Example
///
/// ```ignore
/// <ResourceProtectedRoute 
///     path="/" 
///     resource_condition=authenticated
///     redirect_path="/login" 
///     protected_view=Box::new(|cx| 
///         view! { cx, 
///             <ProtectedView /> 
///         }
///     )
///     fallback_view=Box::new(|cx| 
///         view! { cx, 
///             <LoginView /> 
///         }
///     )
///     resource_err_view=Box::new(|cx| 
///         view! { cx, 
///             <LoginView /> 
///         }
///     )
/// />
/// ```
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "info", skip_all,)
)]
#[component(transparent)]
pub fn ResourceProtectedRoute<P, E>(
    cx: Scope,
    /// The path fragment that this route should match. This can be static (`users`),
    /// include a parameter (`:id`) or an optional parameter (`:id?`), or match a
    /// wildcard (`user/*any`).
    path: P,
    /// The path that will be redirected to if the resource_condition resolves to `false`.
    redirect_path: P,
    /// Resource that resolved to a boolean. Determines if the user is authenticated.
    resource_condition: Resource<(usize, usize, usize), Result<bool, ServerFnError>>,
    /// A boxed closure that defines the view to be displayed when the resource condition evaluates to true.
    protected_view: Box<dyn Fn(Scope) -> E + 'static>,
    /// A boxed closure that defines the fallback view to be displayed when the resource condition is not yet determined.
    fallback_view: Box<dyn Fn(Scope) -> E + 'static>,
    ///A boxed closure that defines the view to be displayed when the resource condition evaluates to an error.
    resource_err_view: Box<dyn Fn(Scope) -> E + 'static>,
    /// If true, inverts the result of the resource condition.
    #[prop(default = false)]
    invert_resource_condition: bool,
    /// The mode that this route prefers during server-side rendering. Defaults to out-of-order streaming.
    #[prop(optional)]
    ssr: SsrMode,
    /// The HTTP methods that this route can handle (defaults to only `GET`).
    #[prop(default = &[Method::Get])]
    methods: &'static [Method],
    /// `children` may be empty or include nested routes.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView
where
    E: IntoView + 'static,
    P: std::fmt::Display + 'static,
{
    // use crate::Redirect;
    let redirect_path = redirect_path.to_string();
    resource_condition.refetch();

    define_route(
        cx,
        children,
        path.to_string(),
        Rc::new(move |cx| {
            // resource_condition.refetch();
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

#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "info", skip_all,)
)]
fn define_route(
    cx: Scope,
    children: Option<Children>,
    path: String,
    view: Rc<dyn Fn(Scope) -> View>,
    ssr_mode: SsrMode,
    methods: &'static [Method],
) -> RouteDefinition {
    let children = children
        .map(|children| {
            children(cx)
                .as_children()
                .iter()
                .filter_map(|child| {
                    child
                        .as_transparent()
                        .and_then(|t| t.downcast_ref::<RouteDefinition>())
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let id = ROUTE_ID.with(|id| {
        let next = id.get() + 1;
        id.set(next);
        next
    });

    RouteDefinition {
        id,
        path,
        children,
        view,
        ssr_mode,
        methods,
    }
}
