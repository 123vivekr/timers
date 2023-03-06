# timers

A countdown timer for your command line.

### Features
- Simple notation
- Countdown display
- Timer end notification
- Log usage to output file

## Usage

Set countdown for 1 minute
```
$ timers 1m
```

Set countdown for 1 second
```
$ timers 1s
```

Set countdown for 1 hour
```
$ timers 1h
```

Log usage to output file
```
$ timers -o log.txt 10s
```
Log format: `timer_set`,`start_date`,`end_date`

`start_date` and `end_date` in [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) format.

For more granular control, you can combine units. To set countdown for 10 minutes and 30 seconds,
```
timers 10m30s
```
