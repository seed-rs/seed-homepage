# Convert Markdown files to HTML, then embed in Rust functions that return
# raw strings.
# We use this script and pandoc instead of built-in markdown functionality
# due to the need for syntax highlighting. Rust's Syntec library doesn't
# work on the wasm32-unknown-unknown target.

# Rererence https://pandoc.org/MANUAL.html

import os
import re

# ./pandoc --list-highlight-styles
# pygments tango espresso zenburn kate monochrome breezedark haddock
STYLE = "tango"
VERSION = "0.4.0"


def main():
    filenames = [
        "quickstart",
        "prereqs",
        "structure",
        "events",
        "components",
        "fetch",
        "lifecycle",
        "routing",
        "release_and_debugging",
        "element_deepdive",
        "complex_apps",
        "misc",
        "server_integration",
        "about",
    ]
    
    for filename in filenames:
        # regex = re.compile(r'<body>(.*?)</body>', re.DOTALL)
        # m = re.search(regex, data)
        #
        # body = m.groups(0)[0]
        #
        # # Update all instances of the version, so we don't have to in Markdown.
        # body = re.sub(r'seed/0\.\d\.(\d{1,3})', "seed/" + VERSION, body)

        with open(f'./markdown/{filename}.md', encoding="utf8") as f:
            data = f.read()

            # Create a new rust file
            with open(f'./src/book/{filename}.rs', 'w', encoding="utf8") as f:
                f.write('pub fn text() -> String {\n')
                f.write('r#####"')
                f.write(data)
                f.write('"#####.into()\n')
                f.write('}')


if __name__ == "__main__":
    main()
