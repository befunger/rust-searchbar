## 
A tool for custom commands in my Firefox URL bar, implemented with Rust.
Commands are used to direct search to specific websites, such as "maps Stockholm" redirecting the search for "Stockholm" to Google Maps.

### Run instructions:
1. Install the 'Add custom search engine' Firefox extension.
2. Configure a new search engine with URL ```http://localhost:8000/search?cmd=%s``` and set to default.
3. Run rocket server using `cargo run` or running `rusty-bunny.exe`


##### Currently implemented commands
- git:          github
- maps:         Google Maps
- yt:           Youtube
- reddit:       reddit
- Defaults to standard Google Search if no command given
