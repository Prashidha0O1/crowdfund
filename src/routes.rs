use axum::response::Html;

pub async fn landing_page() -> Html<&'static str> {
    Html(include_str!("../frontend/routes/landing.html"))
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("../frontend/routes/login.html"))
}

pub async fn homepage(oauth_id: String) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Welcome to Crowdfund</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            text-align: center;
        }}
        .login-button {{
            display: inline-block;
            background-color: #4285f4;
            color: white;
            padding: 10px 20px;
            text-decoration: none;
            border-radius: 5px;
            margin-top: 20px;
        }}
        .login-button:hover {{
            background-color: #357abd;
        }}
    </style>
</head>
<body>
    <h1>Welcome to Crowdfund!</h1>
    <p>Please sign in to continue</p>
    <a href="https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={oauth_id}&response_type=code&redirect_uri=http://localhost:8081/auth/google/callback" class="login-button">
        Sign in with Google
    </a>
</body>
</html>"#
    ))
} 