# Streak

Want to build a habit? Daily streaks are an effective mechanism
to help with accountability and tracking. This is a CLI app that
lets you create, register and track custom streaks for any habit
you want to track.

## Install

```bash
git clone git@github.com:gurukoushik/streak.git
cd streak
cargo install --path .
```

## Usage

```bash
❯ streak -h
streak (noun) [/strēk/]: a continuous period of specified success or luck.
Create, view and track streaks to develop lasting habits by creating 
positive reward signals.


Usage: streak <COMMAND>

Commands:
  create  Create new streak
  log     Log streak for the day
  list    List all the streaks
  remind  Remind about incomplete streaks for the day
  reset   Reset all data (WARNING: This is irreversible and will delete all data)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Create streaks

```bash
# Count streaks on all days of the week
❯ streak create run
+---------------------------+
| Streak created for run 🔥 |
+---------------------------+

# Count streaks on weekdays
❯ streak create work Weekdays
+----------------------------+
| Streak created for work 🔥 |
+----------------------------+
```

### Log streak

```bash
❯ streak log work
+-------------------------+------+
| Streak logged for work! | 1 🔥 |
+-------------------------+------+
```

### List streaks

```bash
❯ streak list
+------+------+
| run  | 0 🔥 |
| work | 1 🔥 |
+------+------+
```

### Remind streaks

```bash
# Remind about incomplete streaks for the day
❯ streak remind
+-----+------+
| run | 0 🔥 |
+-----+------+
```

### Reset streak database

```bash
❯ streak reset
Are you sure you want to reset all the data? (y/n)
y
Streak data reset successfully.
```
