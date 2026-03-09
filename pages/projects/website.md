# This Website

The vision for this website was something super simple and easy to create content for. I wanted to write pages in markdown with minimal configuration so the effort required wouldn't put me off from writing.

The site is structured as a root directory. Files in that directory act as static pages, markdown files get some extra fancy formatting. Directories act as lists and render their children as a grid of cards. Files (and soon directories) can have accompanying toml files to customize their behavior.

The / endpoint is hardcoded to point to an index.md file

Some stuff I'd like to implement in the future is caching the html thats generated from markdown files. Right now they are generated each time the page is loaded. It works good enough for now & its nice for testing but something more optimal would be cool. Automatically generating a sitemap.xml file would be great too (I currently don't show up on search engines very well). I also want more control over how images are rendered from markdown, right now they fill the entire width of the post and don't have much customization.
