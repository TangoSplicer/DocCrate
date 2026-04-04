#!/bin/bash

echo "🧹 Cleaning previous builds..."
rm -rf ./dist
rm -f TangoSplicer_Library.docpack
mkdir -p ./dist

REPOS=(
    "https://github.com/TangoSplicer/project-aletheia.git"
    "https://github.com/TangoSplicer/Swarm-Runtime.git"
    "https://github.com/TangoSplicer/OfflineKnowledgeGraph.git"
    "https://github.com/TangoSplicer/SynQ.git"
)

echo "🚀 Beginning multi-repo fetch..."
for repo in "${REPOS[@]}"; do
    cargo run -q -p doccrate-cli -- build --source "$repo" --out ./dist
done

echo "🎨 Generating Master Library Index..."
cat << 'HTML_EOF' > ./dist/index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DocCrate Library</title>
    <style>
        :root { --bg: #f6f8fa; --card: #ffffff; --border: #d0d7de; --text: #24292f; --link: #0969da; --hover: #f3f4f6; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; line-height: 1.6; margin: 0; padding: 2rem 1rem; color: var(--text); background-color: var(--bg); }
        .container { max-width: 900px; margin: 0 auto; background-color: var(--card); padding: 2.5rem; border-radius: 12px; box-shadow: 0 4px 14px rgba(0,0,0,0.05); border: 1px solid var(--border); }
        h1 { border-bottom: 2px solid var(--border); padding-bottom: 0.3em; margin-top: 0; }
        p { color: #57606a; font-size: 1.1em; }
        .folder-list { list-style: none; padding: 0; border: 1px solid var(--border); border-radius: 8px; overflow: hidden; margin-top: 1.5rem; }
        .folder-list li { border-bottom: 1px solid var(--border); }
        .folder-list li:last-child { border-bottom: none; }
        .folder-list a { display: flex; align-items: center; gap: 10px; padding: 16px; color: var(--text); text-decoration: none; font-weight: 600; font-size: 1.1em; transition: background 0.2s; }
        .folder-list a:hover { background-color: var(--hover); color: var(--link); }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 Offline Knowledge Library</h1>
        <p>Welcome to your completely offline documentation bundle. Select a project below to browse its files:</p>
        <ul class="folder-list">
HTML_EOF

for dir in ./dist/*/; do
    if [ -d "$dir" ]; then
        dirname=$(basename "$dir")
        echo "            <li><a href=\"$dirname/index.html\">📁 $dirname</a></li>" >> ./dist/index.html
    fi
done

cat << 'HTML_EOF' >> ./dist/index.html
        </ul>
    </div>
</body>
</html>
HTML_EOF

echo "📦 Packaging bundle..."
# Run our new pack command!
cargo run -q -p doccrate-cli -- pack --source ./dist --out TangoSplicer_Library.docpack

echo "🎉 Done! Your library is packed into TangoSplicer_Library.docpack"
echo "🌐 Starting local library server on port 8000 (Ctrl+C to stop)"
python -m http.server 8000 --directory ./dist
