# volumen

volumen is a fork of [termpad](https://github.com/SpyrosRoum/termpad); However, I have my own improvements running on it.
volumen allows you to easily host a pastebin server for saving and viewing text right from the terminal, or the browser.


## Changes

* First line defines what type of syntax the rest of the paste highlights to. Eg, doing a #text will not highlight anything because the software will treat it as just plaintext.

* Longform content no longer has horizontal scroll bars and now wraps correctly.

* Changed highlight colors to not kill my eyes.

## Client Usage

Create a fish function

  place this as volumen.fish into your ~/.config/fish/functions folder):

```
function volumen
  curl --data-binary @/dev/stdin https://volumen.civitat.es
end
```

Then just do, `$ cat reasonsGermansSuck.file | volumen`;

Assuming volumen is running in localhost:8000 you can do this to save text using cURL:  
```shell
$ curl -d "Hello world" localhost:8000
http://localhost/DullMagnificentLock
$ # Or
$ curl --data-binary @path/to/file localhost:8000
http://localhost/BrightAliveMotorcycle
```

And this to retrieve saved text:
```shell
$ curl localhost:8000/raw/TenderCheerfulYacht
$ # You might optionally want to pipe the output to a pager like less
$ curl localhost:8000/raw/TenderCheerfulYacht | less
```

Or this for [HTTPie](https://httpie.io/):
```shell
$ echo "Hello World" | http POST localhost:8000
http://localhost/DullMagnificentLock
$ # Or for files:
$ http POST localhost:8000 < path/to/file
http://localhost/BrightAliveMotorcycle
```
And to get text it's the same as cURL:
```shell
$ http localhost:8000/raw/TenderCheerfulYacht
# Or with a pager
$ http localhost:8000/raw/TenderCheerfulYacht | less
```

Note the `/raw/` in the url, without it you will get html output


## Server Usage
### Note that environment variables are checked before assigning the default value.
### Domain (`-d` or `--domain`, env = `DOMAIN`)
Used to construct the url returned to the user. `http` is added as a prefix (Default: `localhost`)
```shell
$ volumen -d example.com
```
This will return urls like: `http://example.com/BrightAliveMotorcycle`

### Port (`-p` or `--port`, env = `PORT`)
Set the port on which the app runs (Default: `8000`)
```shell
$ volumen -p 8043
```

### Output (`-o` or `--output`, env = `OUTPUT`)
Relative or absolute path to the directory where you want to store user-posted pastes (Default: `~/.local/share/volumen/`)
```shell
$ volumen -o /home/www/pastes/
```

### https (`--https`, env = `HTTPS`)
If set, urls will start with `https` instead of `http`
```shell
$ volumen --https
```


### Delete files (`--delete-after`, env = `DELETE_AFTER`)
How many days to keep files for. If set to `0` it will keep them forever (Default: `120`)
```shell
$ volumen --delete-after 60
```

## Install
### From source:
```shell
$ git clone https://github.com/aindoria/volumen.git
$ cd volumen
$ cargo build --release
$ ./target/release/volumen
```

### From cargo:
```shell
$ cargo install volumen
```

### With docker-compose:
Either `wget https://raw.githubusercontent.com/aindoria/volumen/master/docker-compose.yml` or copy the following into `docker-compose.yml`
```
version: "3.4"

services:
  app:
    image: aindoria/volumen
    ports:
      - 8000:8000
    environment:
      - DOMAIN_NAME=example.com
      - HTTPS=true
    volumes:
      - data:$HOME/.local/share/volumen/
    restart: always

volumes:
  data:
    name: volumen
```
and then `docker-compose up -d`
