# m3u to hmtl
This program converts playlists into html with many streaming links

## Features

-  **m3u to html** - Vonverts .m3u files into playlist
-  **VLC database convert** - choose playlist or favorite or all from database
-  **Cli support** - working in terminal
-  **Fetching covers** - Geting covers from LastFm
-  **Artist Data** - share artist data with your friends

## Arguments
       -p, --preview                generate preview in html_path
       -v, --vlc <VLC>              use VLC database for cli mode
       -P, --vlcplaylist            show VLC playlists names cli mode
       -c, --cover                  generate covers for cli mode
       -a, --artist                 generate artist data for cli mode
       -i, --input <INPUT>          cli mode file input
           --html-path <HTML_PATH>  path to html folder default config_dir/html
           --css-path <CSS_PATH>    path to css file default config_dir/css/main.css
       -o, --output <OUTPUT>        output file input default ./{playlistname}_playlist.html
       -h, --help                   Print help
       -V, --version                Print version

## TODO

- **Generic Overview** - Overview stats
- **Count of entries** - Total numbers of songs
