use std::convert::Infallible;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, DOCTYPE};

#[derive(Clone, Copy)]
pub enum PageKind {
    Direct,
    Boosted,
    Full,
}

#[async_trait]
impl<S> FromRequestParts<S> for PageKind
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if matches!(parts.headers.get("HX-Boosted"), Some(v) if v == "true") {
            return Ok(Self::Boosted);
        }

        if matches!(parts.headers.get("HX-Request"), Some(v) if v == "true") {
            return Ok(Self::Direct);
        }

        return Ok(Self::Full);
    }
}

impl PageKind {
    pub fn builder<S: AsRef<str>>(self, title: S) -> PageBuilder {
        PageBuilder::new(self, title)
    }
}

pub struct PageBuilder {
    kind: PageKind,
    title: String,
    head: Option<Markup>,
    direct: Option<Markup>,
}

pub struct Page {
    content: Markup,
    kind: PageKind,
    title: String,
    head: Option<Markup>,
    direct: Option<Markup>,
}

impl PageBuilder {
    pub fn new<S: AsRef<str>>(kind: PageKind, title: S) -> Self {
        Self {
            kind: kind,
            title: title.as_ref().into(),
            head: None,
            direct: None,
        }
    }

    pub fn with_head(mut self, head: Markup) -> Self {
        self.head = match self.head {
            Some(current) => Some(html! {
                (current)
                (head)
            }),
            None => Some(head),
        };
        self
    }

    pub fn with_description<S: AsRef<str>>(self, description: S) -> Self {
        self.with_head(html! {
            meta name="description" content=(description.as_ref());
        })
    }

    pub fn on_direct_request(mut self, direct: Markup) -> Self {
        self.direct = Some(direct);
        self
    }

    pub fn build(self, content: Markup) -> Page {
        Page {
            content,
            kind: self.kind,
            title: self.title,
            head: self.head,
            direct: self.direct,
        }
    }
}

impl From<Page> for Markup {
    fn from(page: Page) -> Self {
        let has_head = page.head.is_some();

        let head = html! {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link
                    rel="stylesheet"
                    href={"/styles.css?v=" (include_str!("../generated/hashes/styles.css.hash"))};
                link
                    rel="icon"
                    href={"/favicon.ico?v=" (include_str!("../generated/hashes/favicon.ico.hash"))}
                    sizes="any";
                script
                    src={"/main.js?v=" (include_str!("../generated/hashes/styles.css.hash"))}
                    {}
                title { (page.title) }
                @if let Some(append_head) = page.head {
                    (append_head)
                }
            }
        };

        let navbar = html! {
           nav
                {
                    ul
                        {
                            li { a href="/" { "Home" } }
                            li { a href="/posts" { "Posts" } }
                        }
                    ul
                        {
                            li { svg
                                #toggle-dark-mode
                                xmlns="http://www.w3.org/2000/svg"
                                onclick="toggle_dark_mode()"
                                title="Toggle Theme"
                                width="20px"
                                height="20px"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke-width="1.5"
                                stroke="currentColor"
                                {
                                    path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    ;
                                }
                            }
                        }
                }
        };

        match page.kind {
            PageKind::Direct => match page.direct {
                Some(direct) => direct,
                None => page.content,
            },
            PageKind::Boosted => html! {
                @if has_head {
                    (head)
                } else {
                    title { (page.title) }
                }
                header ."container" {
                    (navbar)
                }
                main ."container" {
                    (page.content)
                }
            },
            PageKind::Full => html! {
                (DOCTYPE)
                (head)
                html lang="en" {
                    body
                        hx-boost="true"
                        hx-ext="preload,head-support"
                        hx-indicator="#loading-bar"
                        {
                            header ."container" {
                                (navbar)
                            }
                            main ."container" {
                                (page.content)
                            }
                        }
                }
            },
        }
    }
}

#[async_trait]
impl IntoResponse for Page {
    fn into_response(self) -> Response {
        Markup::from(self).into_response()
    }
}
