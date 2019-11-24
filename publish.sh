python3 setup_markdown.py
cargo make build_release
rm pkg/.gitignore
git add .
git commit -am "update"
git push
