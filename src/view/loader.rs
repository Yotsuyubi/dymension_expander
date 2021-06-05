fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

pub fn get_html() -> String {
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, height=device-height, initial-scale=1.0, maximum-scale=1.0, user-scalable=0">
                <link rel="stylesheet" href="https://unpkg.com/element-ui/lib/theme-chalk/index.css">
                <script src="https://unpkg.com/element-ui/lib/index.js"></script>
                <style>
                    @import url('https://fonts.googleapis.com/css2?family=Baloo+Tammudu+2:wght@500&display=swap');
                </style>
            </head>
            <body scroll="no">
                <div id="app"></div>
                <!--[if lt IE 9]>
                <div class="ie-upgrade-container">
                    <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                    <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
                </div>
                <![endif]-->
                <!--[if gte IE 9 | !IE ]> <!-->
                {scripts}
                <![endif]-->
                {style}
            </body>
        </html>
        "#,
        style = inline_style(include_str!("style.css")),
        scripts = inline_script(include_str!("bundle.js"))
    )
}
