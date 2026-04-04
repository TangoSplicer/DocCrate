# 📚 DocCrate Publisher Guide

Welcome to DocCrate. This guide will walk you through the process of curating, compiling, and packaging offline knowledge bundles (DocPacks) for distribution or sale.

DocCrate is designed to take raw data—like GitHub repositories, codebases, and local markdown notes—and automatically format them into a highly polished, searchable, and completely offline website. You then package this website into a single file to distribute to your end-users.

Here is the standard 4-step workflow to create a commercial knowledge bundle.

---

## Step 1: Staging Your Sources (`add`)
DocCrate uses a "Staging Area" so you can combine multiple different projects into one massive master library. You must queue up your sources before building.

Use the `add` command followed by the local folder path or the Git repository URL.

**Command:**
```bash
cargo run -p doccrate-cli -- add <URL_OR_PATH>
### Examples                                                  ```bash                                                       cargo run -p doccrate-cli -- add [https://github.com/TangoSplicer/project-aletheia.git](https://github.com/TangoSplicer/project-aletheia.git)
cargo run -p doccrate-cli -- add ./my_local_notes
Note: You can run this command as many times as needed to add different repositories to your current build queue.
​Step 2: Reviewing Your Queue (status)
​Before compiling, you can verify exactly which repositories and folders are queued up for the build.
​Command:                                                      ```bash                                                       cargo run -p doccrate-cli -- status
This outputs a numbered list of all your staged sources.
​Step 3: Compiling the Library (build)
​Once your sources are staged, initiate the build process. DocCrate will automatically download the repositories, parse the code and markdown, build the offline search index, and generate a Master Dashboard to navigate it all.
​Command:                                                      ```bash                                                       cargo run -p doccrate-cli -- build
What happens here: DocCrate processes everything in your queue and outputs the finished HTML files into a new folder named ./dist.
​Step 4: Packaging for Distribution (pack)
​You now have a fully functional offline library sitting in your ./dist folder, but it needs to be compressed into a single, distributable file for your customers.
​Use the pack command and specify the final name of your product. The file extension should always be .docpack.
​Command:                                                      ```bash                                                       cargo run -p doccrate-cli -- pack --out <YOUR_CUSTOM_NAME>.docpack
Example:                                                      ```bash                                                       cargo run -p doccrate-cli -- pack --out AI_Compliance_Bundle_v1.docpack
🎯 The End-User Experience
​You now have a single AI_Compliance_Bundle_v1.docpack file on your system. This is the final product you will distribute to your customers.
​How do they use it?
Because a .docpack is simply a standard compressed archive, the end-user only needs to rename the file extension to .zip, extract the folder, and double-click the index.html file to instantly browse and search the library completely offline.
