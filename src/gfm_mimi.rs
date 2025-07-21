use comrak::RenderPlugins;

pub struct GfmMimiRenderer<'a> {
    options: comrak::Options<'a>,
    codefence_syntax_highlighter: comrak::plugins::syntect::SyntectAdapter,
}

impl GfmMimiRenderer<'_> {
    pub fn new() -> Self {
        let options = comrak::Options {
            extension: comrak::ExtensionOptions::builder()
                .table(true)
                .tasklist(true)
                .strikethrough(true)
                .shortcodes(false)
                .build(),
            parse: comrak::ParseOptions::builder()
                .relaxed_tasklist_matching(true)
                .build(),
            render: comrak::RenderOptions::builder()
                .escape(true)
                .ignore_empty_links(true)
                .tasklist_classes(true)
                .build(),
        };

        let codefence_syntax_highlighter = comrak::plugins::syntect::SyntectAdapterBuilder::new()
            // TODO: Set options like theme etc
            .build();

        Self {
            options,
            codefence_syntax_highlighter,
        }
    }

    pub fn gfm_mimi_to_html(&self, markdown: &str) -> String {
        comrak::markdown_to_html_with_plugins(
            markdown,
            &self.options,
            &comrak::Plugins::builder()
                .render(
                    RenderPlugins::builder()
                        .codefence_syntax_highlighter(&self.codefence_syntax_highlighter)
                        .build(),
                )
                .build(),
        )
    }

    pub fn gfm_mimi_to_commonmark(&self, markdown: &str) -> String {
        comrak::markdown_to_commonmark(markdown, &self.options)
    }
}

impl Default for GfmMimiRenderer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
