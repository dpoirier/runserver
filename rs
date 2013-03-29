#!/bin/sh

# Start the Django project in the current directory using runserver,
# with a unique port if the project name is known, and then open
# the front page in a browser.

if [ "$1" = "." ] ; then
   # "." means don't open the page in a browser
   OPEN=false
   shift
else
  # Try to figure out how to open a page on this system, and set
  # OPEN to the command.
  case `uname` in
    Darwin) OPEN=open;;
    Linux)  OPEN=xdg-open;;
    *) echo "Don't know how to open URLs on "`uname`; exit 1;;
  esac
fi

case $VIRTUAL_ENV in
  */Project1) PORT=8001;;
  */Project2) PORT=8002;;
  # ...
  *) PORT=8000;;
esac

TARGET=localhost:$PORT

# Open page in browser after giving Django a few seconds to start
if [ "$OPEN" != "false" ] ; then
  (sleep 4;$OPEN http://$TARGET)&
fi

exec python manage.py runserver "$@" "$TARGET"
