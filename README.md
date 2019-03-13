# Alamo Drafthouse Commandline (adc)

This is a project that I've spent a few weeks working on in order to properly learn Rust. I'd written some
shell scripts a while back to collect lists of movies from the Alamo Drafthouse website sicne they show a lot
of interesting and obscure movies; especially Terror Tuesday and Weird Wednesday. At one point, I'd also used
my scripts to figure out when tickets for movies go on sale by polling their API and waiting for movies that
match the name I'm looking for come up.

So, with this project, you can do this, too!

## Building and installing

> Right now, this project isn't published so you have to manually build it. I don't know if I'm gonna publish it
> or not

```console
$ cargo build --release
$ cp target/release/adc /usr/local/bin/
```

This will compile the `adc` tool and move it to a location that you can execute it.

## Using adc

Currently the `adc` commandline uses the subcommand format. To get help, run:

```console
$ adc help
adc 0.1.0
Spike Grobstein <me@spike.cx>
Query the Alamo Drafthouse schedule to get lists of upcoming films playing in theaters.

USAGE:
    adc [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cinema     List available cinemas.
    films      List films for the given theater
    get        Fetch the given cinema
    get-all    Update all local cinema data for every cinema
    help       Prints this message or the help of the given subcommand(s)

adc is in no way affiliated with the Alamo Drafthouse Cinemas.
I'm just a huge fan.
```

To get started you can get a list of all of the available cinemas:

```console
$ adc cinema
```

> You can get it in json by passing the `--json` flag (super useful for you `jq` fans).

Then, to get a list of the movies currently playing at a theatre, pass the numeric ID to the `films`
subcommand:

```console
$ adc films 0801
```

`adc` will download and cache the cinema response for up to 24 hours (cache is stored in `~/.alamo/` by
default), so repeated use of the same cinema will not hammer their API. You can customize the location of this
directory by setting the environment variable `ADC_DATA_DIR`.

You can also filter films from a theatre by type. For example, to only view the current Terror Tuesday
showings:

```console
$ adc films 0801 --type 'terror tuesday'
```

## Disclaimer

This project is in no way affiliated with the Alamo Drafthouse Cinemas. I'm just a huge fan of the theatres
and wanted an easy way to collect lists of movies that they show.

