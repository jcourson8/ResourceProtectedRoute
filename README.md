# ResourceProtectedRoute

This is a guarded route component that is protected by a certain resource condition. It defines a route that will display different views based on the state of a certain condition.

## Example
### Protected page that requires user to be logged in:
```rs
<ResourceProtectedRoute
    path="/"
    resource_condition=authenticated
    redirect_path="/login"
    protected_view=Box::new(|cx|
        view! { cx,
            <ProtectedView />
        }
    )
    fallback_view=Box::new(|cx|
        view! { cx,
            <LoginView />
        }
    )
    resource_err_view=Box::new(|cx|
        view! { cx,
            <LoginView />
        }
    )
/>
```
### Login page redirects away when a user is logged in:
```rs
<ResourceProtectedRoute 
                    
    path="login" 
    resource_condition=authenticated
    invert_resource_condition=true
    redirect_path="/" 
    protected_view=Box::new(|cx|
        view! { cx,
            <LoginView /> 
        }
    )
    fallback_view=Box::new(|cx| 
        view! { cx, 
            <LoginView /> 
        }
    )
    resource_err_view=Box::new(|cx| 
        view! { cx, 
            <LoginView /> 
        }
    )                        
/>
```

## Required Props
**cx:** Scope
**path:** P
- The path fragment that this route should match. This can be static (users), include a parameter (:id) or an optional parameter (:id?), or match a wildcard (user/*any).
redirect_path: P
The path that will be redirected to if the resource_condition resolves to false.
**resource_condition:** Resource<(usize, usize, usize), Result<bool, ServerFnError>>
- Resource that resolved to a boolean. Determines if the user is authenticated.
**protected_view:** Box<dyn Fn(Scope) -> E + 'static>
- A boxed closure that defines the view to be displayed when the resource condition evaluates to true.
**fallback_view:** Box<dyn Fn(Scope) -> E + 'static>
- A boxed closure that defines the fallback view to be displayed when the resource condition is not yet determined.
**resource_err_view:** Box<dyn Fn(Scope) -> E + 'static>
- A boxed closure that defines the view to be displayed when the resource condition evaluates to an error.
**invert_resource_condition:** bool
- If true, inverts the result of the resource condition.
**methods:** [&'static [Method]]
- The HTTP methods that this route can handle (defaults to only GET).

## Optional Props
**ssr:** SsrMode
- The mode that this route prefers during server-side rendering. Defaults to out-of-order streaming.
**children:** Children
- children may be empty or include nested routes.
